use std::env;
use std::io;
use std::path::Path;

use tokio::net::{TcpStream, UnixStream};

use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::geoip::GeoIP;
use tor_analyzer_lib::socket::Socket;
use tor_analyzer_lib::tor::NomParse;
use torut::control::{AsyncEvent, ConnError, UnauthenticatedConn};

async fn event_handler(event: AsyncEvent<'static>) -> Result<(), ConnError> {
    eprintln!("Event: {:?}", event);
    Ok(())
}

async fn run() -> Result<(), Error> {
    let _gi = GeoIP::new();

    let socket: Socket = match env::args().skip(1).next() {
        Some(a) => {
            let path = Path::new(a.as_str());
            if path.exists() {
                UnixStream::connect(a).await?.into()
            } else {
                TcpStream::connect(a).await?.into()
            }
        }
        None => TcpStream::connect("127.0.0.1:9051").await?.into(),
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
    let mut conn = anon_conn.into_authenticated().await;
    conn.set_async_event_handler(Some(event_handler));

    let circuits = conn.get_info("circuit-status").await?;
    println!("circuits: {}", circuits);
    let mut rest_circuits = circuits.as_str();
    let mut count = 0usize;
    while !rest_circuits.is_empty() {
        let (rest, c) = tor_analyzer_lib::tor::circuit::Circuit::parse::<
            nom::error::VerboseError<&str>,
        >(rest_circuits)?;
        println!("{:2}: {}", count, c);
        rest_circuits = rest;
        count += 1;
    }
    eprintln!("{:?}", rest_circuits);

    Ok(())
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {:?}", e);
        }
    }
}
