mod bindings;
pub mod country;
pub mod error;
pub mod geoip;
pub mod socket;
pub mod tor;

use std::fmt;

use error::Result;
use socket::Socket;
use tor::circuit::Circuit;
use tor::common::{CircuitID, StreamID};
use tor::conn::Connection;
use tor::NomParse;

use crate::tor::ns::OnionRouter;
use crate::tor::stream::Stream;
pub mod prelude {
    pub use crate::geoip::GeoIP;
    pub use crate::socket::Socket;
    pub use crate::tor::circuit::Circuit;
    pub use crate::tor::common::{CircuitID, HostOrAddr, StreamID, Target, Time};
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
        let s = s.as_ref();
        log::debug!("Tor controller at {}", s);
        let sock = Socket::new(s)?;
        let mut ctrl = Connection::new(sock);

        ctrl.authenticate()?;

        Ok(Self { ctrl })
    }

    pub fn get_circuits(&mut self) -> Result<Vec<Circuit>> {
        let circuits_string = self.ctrl.get_info("circuit-status")?;
        let (rest, _newline) = nom::combinator::opt(nom::bytes::complete::tag::<
            &str,
            &str,
            nom::error::VerboseError<&str>,
        >("\r\n"))(circuits_string.as_str())?;
        let (_rest, circuits) =
            nom::multi::many0(Circuit::parse::<nom::error::VerboseError<&str>>)(rest)?;

        Ok(circuits)
    }

    pub fn get_streams(&mut self) -> Result<Vec<Stream>> {
        let streams_string = self.ctrl.get_info("stream-status")?;
        let (_rest, streams) = nom::multi::many0(Stream::parse::<nom::error::VerboseError<&str>>)(
            streams_string.as_str(),
        )?;

        Ok(streams)
    }

    pub fn get_onion_router<D: fmt::Display>(&mut self, hash: D) -> Result<OnionRouter> {
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

    pub fn extend_circuit(&mut self, id: CircuitID, path: Vec<String>) -> Result<String> {
        let mut path_str =
            String::with_capacity(path.iter().map(|s| s.len()).sum::<usize>() + path.len() - 1);
        let mut first = true;
        for p in path.iter() {
            if !first {
                path_str.push(',');
            }
            first = false;
            path_str.push_str(p.as_str());
        }
        let response = self
            .ctrl
            .send_command(format!("EXTENDCIRCUIT {} {}", id, path_str))?;
        Ok(response.data)
    }

    pub fn attach_stream(&mut self, stream_id: StreamID, circuit_id: CircuitID) -> Result<String> {
        let response = self
            .ctrl
            .send_command(format!("ATTACHSTREAM {} {}", stream_id, circuit_id))?;
        Ok(response.data)
    }

    pub fn set_conf<D1: fmt::Display, D2: fmt::Display>(
        &mut self,
        keyword: D1,
        value: Option<D2>,
    ) -> Result<()> {
        let cmd = if let Some(value) = value {
            format!("SETCONF {}={}", keyword, value)
        } else {
            format!("SETCONF {}", keyword)
        };

        self.ctrl.send_command(cmd)?;
        Ok(())
    }

    pub fn get_conf<D: fmt::Display>(&mut self, keyword: D) -> Result<String> {
        let response = self.ctrl.send_command(format!("GETCONF {}", keyword))?;
        Ok(response.data)
    }
}
