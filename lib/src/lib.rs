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
use tor::NomParse;
use tor::{circuit::Circuit, conn::Connection};

pub mod prelude {
    pub use crate::geoip::GeoIP;
    pub use crate::socket::Socket;
    pub use crate::tor::circuit::Circuit;
    pub use crate::tor::common::{CircuitID, StreamID, Target, Time};
    pub use crate::tor::ns::OnionRouter;
    pub use crate::tor::stream::Stream;
    pub use crate::tor::utils::hex_encode;
    pub use crate::tor::NomParse;
    pub use crate::TorController;
}

pub struct TorController {
    ctrl: crate::tor::conn::Connection<Socket>,
}

impl TorController {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self> {
        let sock = Socket::new(s)?;
        let mut ctrl = Connection::new(sock);

        ctrl.authenticate()?;

        Ok(Self { ctrl })
    }

    pub fn get_circuits(&mut self) -> Result<Vec<Circuit>> {
        let circuits_string = self.ctrl.get_info("circuit-status")?;
        let (_rest, circuits) = nom::multi::many1(
            Circuit::parse::<nom::error::VerboseError<&str>>,
        )(circuits_string.as_str())?;

        Ok(circuits)
    }

    pub fn get_streams(&mut self) -> Result<Vec<Stream>> {
        let streams_string = self.ctrl.get_info("stream-status")?;
        let (_rest, streams) = nom::multi::many0(Stream::parse::<nom::error::VerboseError<&str>>)(
            streams_string.as_str(),
        )?;

        Ok(streams)
    }

    pub fn get_onion_router<D: std::fmt::Display>(&mut self, hash: D) -> Result<OnionRouter> {
        let or_str = self.ctrl.get_info(&format!("ns/id/{}", hash))?;
        let (_rest, or) = OnionRouter::parse::<nom::error::VerboseError<&str>>(or_str.as_str())?;

        Ok(or)
    }

    pub fn get_all_onion_router(&mut self) -> Result<Vec<OnionRouter>> {
        let or_str = self.ctrl.get_info("ns/all")?;
        let (_rest, ors) = nom::multi::many0(OnionRouter::parse::<nom::error::VerboseError<&str>>)(
            or_str.as_str(),
        )?;

        Ok(ors)
    }
}
