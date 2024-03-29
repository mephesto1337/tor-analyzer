use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead, BufReader, Read, Write};

use rand::RngCore;

use crate::error::Error;
use crate::tor::auth::{AuthChallengeResponse, AuthMethods, ProtocolInfo};
use crate::tor::protocol::ResponseLine;
use crate::tor::utils::{hex_encode, parse_single_key_value};
use crate::tor::NomParse;

const TOR_CLIENT_HASH_KEY: &[u8] = b"Tor safe cookie authentication controller-to-server hash";
const TOR_SERBER_HASH_KEY: &[u8] = b"Tor safe cookie authentication server-to-controller hash";

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    pub code: u16,
    pub data: String,
}

impl std::convert::AsRef<str> for Response {
    fn as_ref(&self) -> &str {
        self.data.as_str()
    }
}

impl std::convert::AsRef<[u8]> for Response {
    fn as_ref(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

pub struct Connection<S> {
    conn: BufReader<S>,
    async_events: Option<HashMap<String, VecDeque<String>>>,
}

impl<S> Connection<S>
where
    S: Read + Write,
{
    pub fn new(s: S) -> Self {
        Self {
            conn: BufReader::new(s),
            async_events: None,
        }
    }

    pub fn authenticate(&mut self) -> Result<(), Error> {
        let raw_protocol_info = self.send_command("PROTOCOLINFO 1")?;
        if raw_protocol_info.code != 250 {
            return Err(raw_protocol_info.into());
        }
        let (_, protocol_info) =
            ProtocolInfo::parse::<nom::error::VerboseError<&str>>(raw_protocol_info.data.as_str())?;

        if protocol_info.auth_methods.contains(&AuthMethods::Null) {
            return Ok(());
        }

        if protocol_info
            .auth_methods
            .contains(&AuthMethods::SafeCookie)
        {
            let cookie = protocol_info.cookie_file.as_ref().ok_or_else(|| {
                Error::Io(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "No cookie file provided by controller",
                ))
            })?;

            return self.safe_cookie_auth(cookie);
        }

        todo!("No other authentication mecanism");
    }

    fn safe_cookie_auth(&mut self, cookie: &String) -> Result<(), Error> {
        log::debug!("Trying to read: {}", cookie);
        let mut input = std::fs::read(cookie)?;

        let mut client_nonce = [0u8; 64];
        rand::thread_rng().fill_bytes(&mut client_nonce);
        let response = self.send_command(format!(
            "AUTHCHALLENGE SAFECOOKIE {}",
            hex_encode(client_nonce)
        ))?;

        let (_, acr) =
            AuthChallengeResponse::parse::<nom::error::VerboseError<&str>>(response.data.as_str())?;
        input.extend_from_slice(&client_nonce[..]);
        input.extend_from_slice(&acr.server_nonce[..]);

        let client_hash = hmac_sha256::HMAC::mac(&input[..], TOR_CLIENT_HASH_KEY);
        let computed_server_hash = hmac_sha256::HMAC::mac(&input[..], TOR_SERBER_HASH_KEY);

        if computed_server_hash != acr.server_hash {
            log::error!("Bad hash from server, cookie file changed?!");
            return Err(Error::Protocol(format!(
                "Invalid server hash (computed={} received={})",
                hex_encode(computed_server_hash),
                hex_encode(acr.server_hash)
            )));
        }

        let response = self.send_command(format!("AUTHENTICATE {}", hex_encode(client_hash)))?;
        if response.code != 250 {
            return Err(response.into());
        }

        log::debug!("Connection is now authenticated");

        Ok(())
    }

    fn read_response_line(&mut self, line: &mut String) -> Result<ResponseLine, Error> {
        loop {
            match ResponseLine::parse::<nom::error::VerboseError<&str>>(line.as_str()) {
                Ok((rest, response_line)) => {
                    if !rest.is_empty() {
                        log::warn!("read too much: {}", rest);
                    }
                    let rest = rest.to_owned();
                    line.clear();
                    line.push_str(rest.as_str());
                    return Ok(response_line);
                }
                Err(nom::Err::Incomplete(_)) => {}
                Err(e) => return Err(e.into()),
            }
            self.conn.read_line(line)?;
        }
    }

    pub fn get_info<B: AsRef<str>>(&mut self, cmd: B) -> Result<String, Error> {
        let cmd = cmd.as_ref();
        let response = self.send_command(format!("GETINFO {cmd}"))?;
        if response.code != 250 {
            return Err(response.into());
        }

        if let Some((key, val)) = parse_single_key_value(response.data.as_str()) {
            if key != cmd {
                return Err(Error::Protocol(format!(
                    "Invalid prefix (expected: {cmd:?} received={key:?})",
                )));
            }
            Ok(val.into())
        } else {
            Err(Error::Protocol(format!(
                "Cannot find key/value pair in {:?}",
                response.data
            )))
        }
    }

    fn receive_response(&mut self) -> Result<Response, Error> {
        let mut line = String::with_capacity(1024);

        let first_response = self.read_response_line(&mut line)?;
        let code = first_response.get_code();

        let mut is_end = first_response.is_end();
        let mut data = first_response.take_data();
        data.push_str("\r\n");

        while !is_end {
            let response_line = self.read_response_line(&mut line)?;
            if response_line.get_code() != code {
                unreachable!(
                    "Buggy protocol or parser, got codes {} and {}",
                    code,
                    response_line.get_code()
                );
            }

            is_end = response_line.is_end();
            data.push_str(response_line.take_data().as_str());
            data.push_str("\r\n");
        }

        if log::log_enabled!(log::Level::Trace) {
            let data = if let Some(idx) = data.rfind("\r\n") {
                &data[..idx]
            } else {
                &data[..]
            };
            log::trace!("Received: {} {}", code, data);
        }
        Ok(Response { code, data })
    }

    pub fn send_command<B: AsRef<str>>(&mut self, cmd: B) -> Result<Response, Error> {
        let mut cmd = cmd.as_ref().to_owned();
        if !cmd.ends_with("\r\n") {
            cmd.push_str("\r\n");
        }
        if log::log_enabled!(log::Level::Trace) {
            let cmd = cmd.split("\r\n").next().unwrap();
            log::trace!("Sending command: {}", cmd);
        }
        self.conn.get_mut().write_all(cmd.as_bytes())?;

        loop {
            let response = self.receive_response()?;
            if response.code == 650 {
                if self.async_events.is_none() {
                    log::error!("Received async event, but none were subscribed");
                    continue;
                }

                if let Some((key, val)) = response.data.split_once(' ') {
                    if let Some(events) = self.async_events.as_mut().unwrap().get_mut(key) {
                        events.push_back(val.to_owned());
                    } else {
                        log::error!(
                            "Received async event for {}, but it was not subscribed",
                            key
                        );
                    }
                } else {
                    log::warn!("Buggy async response, no first word");
                }

                continue;
            }

            return Ok(response);
        }
    }

    pub fn handle_async_event(&mut self, events: &[&dyn AsRef<str>]) -> Result<(), Error> {
        let mut async_events = self.async_events.take().unwrap_or_default();
        let mut cmd = String::from("SETEVENTS");
        for event in events {
            let event = event.as_ref().to_owned();
            cmd.push(' ');
            cmd.push_str(event.as_str());
            async_events.entry(event).or_insert_with(VecDeque::new);
        }

        let response = self.send_command(cmd)?;
        if response.code != 250 {
            Err(response.into())
        } else {
            self.async_events = Some(async_events);
            Ok(())
        }
    }

    pub fn consome_async_event(&mut self) -> Option<(String, String)> {
        let events = self.async_events.as_mut()?;

        for (key, vals) in events.iter_mut() {
            if let Some(val) = vals.pop_front() {
                return Some((key.clone(), val));
            }
        }

        None
    }
}
