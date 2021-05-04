use std::io::{self, BufRead, BufReader, Read, Write};

use crate::error::Error;
use crate::tor::auth::{AuthMethods, ProtocolInfo};
use crate::tor::protocol::ResponseLine;
use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    pub code: u16,
    pub data: String,
}

pub struct Connection<S> {
    conn: BufReader<S>,
}

impl<S> Connection<S>
where
    S: Read + Write,
{
    pub fn new(s: S) -> Self {
        Self {
            conn: BufReader::new(s),
        }
    }

    pub fn authenticate(&mut self) -> Result<(), Error> {
        let raw_protocol_info = self.send_command("PROTOCOLINFO 1")?;
        if raw_protocol_info.code != 250 {
            return Err(Error::Protocol(raw_protocol_info.code));
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

        Ok(())
    }

    fn safe_cookie_auth(&mut self, cookie: &String) -> Result<(), Error> {
        let _ = cookie;
        todo!()
    }

    fn read_response_line(&mut self, line: &mut String) -> Result<ResponseLine, Error> {
        loop {
            match ResponseLine::parse::<nom::error::VerboseError<&str>>(line.as_str()) {
                Ok((rest, response_line)) => {
                    if !rest.is_empty() {
                        eprintln!("read too much: {}", rest);
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

    pub fn send_command<B: AsRef<str>>(&mut self, cmd: B) -> Result<Response, Error> {
        let mut cmd = cmd.as_ref().to_owned();
        if !cmd.ends_with("\r\n") {
            cmd.push_str("\r\n");
        }
        self.conn.get_mut().write_all(cmd.as_bytes())?;

        let mut line = String::with_capacity(1024);

        let first_response = self.read_response_line(&mut line)?;
        let code = first_response.get_code();
        if first_response.take_data() != &cmd[..(cmd.len() - 2)] {
            unreachable!("Invalid first line");
        }
        let mut data = String::new();

        loop {
            let response_line = self.read_response_line(&mut line)?;
            if response_line.get_code() != code {
                unreachable!(
                    "Buggy protocol or parser, got codes {} and {}",
                    code,
                    response_line.get_code()
                );
            }
            if response_line.is_end() {
                break;
            } else {
                data.push_str(response_line.take_data().as_str());
                data.push_str("\r\n");
            }
        }

        Ok(Response { code, data })
    }
}
