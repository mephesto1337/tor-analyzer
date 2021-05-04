use std::future::Future;
use std::io;

use tokio::net::{TcpStream, UnixStream};

use torut::control::{AsyncEvent, AuthenticatedConn, ConnError, UnauthenticatedConn};

mod bindings;
pub mod country;
pub mod error;
pub mod geoip;
pub mod socket;
pub mod tor;

use crate::tor::ns::OnionRouter;
use crate::tor::stream::Stream;
use error::Result;
use socket::Socket;
use tor::circuit::Circuit;
use tor::NomParse;

pub mod prelude {
    pub use crate::geoip::GeoIP;
    pub use crate::socket::Socket;
    pub use crate::tor::circuit::Circuit;
    pub use crate::tor::common::{CircuitID, StreamID, Target, Time};
    pub use crate::tor::ns::OnionRouter;
    pub use crate::tor::stream::Stream;
    pub use crate::tor::NomParse;
    pub use crate::TorController;
}

// type EventHandler =
//     fn(AsyncEvent<'static>) -> dyn Future<Output = std::result::Result<(), ConnError>>;
//
pub fn void_async_event_handler(
    event: AsyncEvent<'static>,
) -> impl Future<Output = std::result::Result<(), ConnError>> {
    // unreachable!("Unhandle async event {:?}", event);
    std::future::ready(Ok(()))
}

pub struct TorController<H> {
    ctrl: AuthenticatedConn<Socket, H>,
}

impl<F, H> TorController<H>
where
    H: Fn(AsyncEvent<'static>) -> F,
    F: Future<Output = std::result::Result<(), ConnError>> + Send,
{
    pub async fn new<S: AsRef<str>>(s: S) -> Result<Self> {
        let host_or_path = s.as_ref();

        let path = std::path::Path::new(host_or_path);
        let socket = if path.exists() {
            Socket::Unix(UnixStream::connect(path).await?)
        } else {
            Socket::Net(TcpStream::connect(host_or_path).await?)
        };

        let mut anon_conn = UnauthenticatedConn::new(socket);

        let infos = anon_conn.load_protocol_info().await?;
        let auth_data = match infos.make_auth_data()? {
            Some(data) => data,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Cannot authenticate, maybe HashPassword is missing?",
                )
                .into());
            }
        };

        anon_conn.authenticate(&auth_data).await?;
        let ctrl = anon_conn.into_authenticated().await;
        // ctrl.set_async_event_handler(Some(async_event_handler));

        Ok(Self { ctrl })
    }

    pub fn set_async_event_handler(&mut self, h: H) {
        self.ctrl.set_async_event_handler(Some(h));
    }

    pub async fn get_circuits(&mut self) -> Result<Vec<Circuit>> {
        let circuits_string = self.ctrl.get_info("circuit-status").await?;
        let (_rest, circuits) = nom::multi::many1(
            Circuit::parse::<nom::error::VerboseError<&str>>,
        )(circuits_string.as_str())?;

        Ok(circuits)
    }

    pub async fn get_streams(&mut self) -> Result<Vec<Stream>> {
        let streams_string = self.ctrl.get_info("stream-status").await?;
        let (_rest, streams) = nom::multi::many0(Stream::parse::<nom::error::VerboseError<&str>>)(
            streams_string.as_str(),
        )?;

        Ok(streams)
    }

    pub async fn get_onion_router<D: std::fmt::Display>(&mut self, hash: D) -> Result<OnionRouter> {
        let or_str = self.ctrl.get_info(&format!("ns/id/${}", hash)).await?;
        let (_rest, or) = OnionRouter::parse::<nom::error::VerboseError<&str>>(or_str.as_str())?;

        Ok(or)
    }

    pub async fn get_all_onion_router(&mut self) -> Result<Vec<OnionRouter>> {
        let or_str = self.ctrl.get_info("ns/all").await?;
        let (_rest, ors) = nom::multi::many0(OnionRouter::parse::<nom::error::VerboseError<&str>>)(
            or_str.as_str(),
        )?;

        Ok(ors)
    }
}
