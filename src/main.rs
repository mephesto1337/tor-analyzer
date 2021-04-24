use std::env;
use std::io;

use tokio::net::{TcpStream, UnixStream};

use tor_analyzer::geoip::GeoIP;
use torut::control::{AsyncEvent, ConnError, UnauthenticatedConn};

async fn event_handler(event: AsyncEvent<'static>) -> Result<(), ConnError> {
    eprintln!("Event: {:?}", event);
    Ok(())
}

async fn run() -> Result<(), ConnError> {
    let _gi = GeoIP::new();

    let path = env::args().skip(1).next().expect("Usage: PROG SOCKET");
    let stream = UnixStream::connect(path).await?;
    let mut anon_conn = UnauthenticatedConn::new(stream);

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
    while !rest_circuits.is_empty() {
        let (rest, c) = tor_analyzer::tor::circuit::Circuit::parse(rest_circuits).map_err(|e| {
            ConnError::IOError(io::Error::new(io::ErrorKind::Other, format!("{}", e)))
        })?;
        println!("{}", c);
        rest_circuits = rest;
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
