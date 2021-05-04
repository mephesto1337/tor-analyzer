use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::{context, ContextError, ParseError};
use nom::multi::separated_list1;
use nom::sequence::tuple;

use crate::tor::utils::quoted_string;
use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq)]
pub enum AuthMethods {
    /// Null - no authentication. Just issue authenticate command to be authenticated
    Null,

    /// In order to authenticate password is required
    HashedPassword,

    /// Cookie file has to be read in order to authenticate
    Cookie,

    /// CookieFile has to be read and hashes with both server's and client's nonce has to match on server side.
    /// This way evil server won't be able to copy response and act as an evil proxy
    SafeCookie,
}

impl NomParse for AuthMethods {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "authentication method",
            alt((
                map(tag("NULL"), |_| Self::Null),
                map(tag("HASHEDPASSWORD"), |_| Self::HashedPassword),
                map(tag("COOKIE"), |_| Self::Cookie),
                map(tag("SAFECOOKIE"), |_| Self::SafeCookie),
            )),
        )(input)
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct ProtocolInfo {
    pub auth_methods: Vec<AuthMethods>,
    pub cookie_file: Option<String>,
    pub version: String,
}

impl NomParse for ProtocolInfo {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, _) = tag("AUTH METHODS=")(input)?;
        let (rest, auth_methods) = separated_list1(tag(","), AuthMethods::parse)(rest)?;
        let (rest, cookie_file) = opt(map(tuple((tag(" COOKIEFILE="), quoted_string)), |x| {
            String::from(x.1)
        }))(rest)?;

        let (rest, version) = context(
            "version",
            map(
                tuple((tag("\r\nVERSION Tor="), quoted_string, tag("\r\n"))),
                |x| String::from(x.1),
            ),
        )(rest)?;

        Ok((
            rest,
            Self {
                auth_methods,
                cookie_file,
                version,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_info() {
        let pi = ProtocolInfo {
            auth_methods: vec![AuthMethods::Cookie, AuthMethods::SafeCookie],
            cookie_file: Some("/var/lib/tor/control_auth_cookie".into()),
            version: "0.4.5.7".into(),
        };
        assert_eq!(
            ProtocolInfo::parse::<nom::error::VerboseError<&str>>(
                "AUTH METHODS=COOKIE,SAFECOOKIE COOKIEFILE=\"/var/lib/tor/control_auth_cookie\"\r\n\
                VERSION Tor=\"0.4.5.7\"\r\n"
            ),
            Ok(("", pi))
        );
    }
}
/*
    fn retrieve_protocol_info(&mut self) -> Result<&ProtocolInfo, Error> {
        let response = self.send_command("PROTOCOLINFO 2")?;
        eprintln!("Response: {:?}", response);
        self.protocol_info = Some(ProtocolInfo::default());
        Ok(self.protocol_info.as_ref().unwrap())
    }

    pub fn protocol_info(&mut self) -> Result<&ProtocolInfo, Error> {
        if let Some(ref pi) = self.protocol_info {
            return Ok(pi);
        }
        self.retrieve_protocol_info()
    }

*/
