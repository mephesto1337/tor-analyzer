use std::env;

use tor_analyzer_lib::error::Error;
use tor_analyzer_lib::geoip::GeoIP;
use tor_analyzer_lib::socket::Socket;
use tor_analyzer_lib::tor::NomParse;

fn main() -> Result<(), Error> {
    let _gi = GeoIP::new();

    let remote = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:9051".into());
    let mut conn = tor_analyzer_lib::tor::conn::Connection::new(Socket::new(remote)?);
    conn.authenticate()?;

    let circuits = conn.get_info("circuit-status")?;
    println!("circuits: {circuits:?}");
    let (_rest, circuits) = nom::multi::many0(
        tor_analyzer_lib::tor::circuit::Circuit::parse::<nom::error::VerboseError<&str>>,
    )(circuits.as_str())?;
    for (i, c) in circuits.iter().enumerate() {
        println!("{i:02}: {c}");
    }

    Ok(())
}
