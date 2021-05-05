use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::error::{context, ContextError, ParseError};
use nom::multi::{count, separated_list1};
use nom::sequence::tuple;
use std::fmt;

use crate::tor::utils::{hex_encode, parse_hex, quoted_string};
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
        let (rest, _) = tag("PROTOCOLINFO 1\r\n")(input)?;
        let (rest, _) = tag("AUTH METHODS=")(rest)?;
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

#[derive(Eq, PartialEq, Default)]
pub struct AuthChallengeResponse {
    pub server_hash: [u8; 32],
    pub server_nonce: [u8; 32],
}

impl fmt::Debug for AuthChallengeResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthChallengeResponse")
            .field("server_hash", &hex_encode(self.server_hash))
            .field("server_nonce", &hex_encode(self.server_nonce))
            .finish()
    }
}

impl NomParse for AuthChallengeResponse {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, _) = tag("AUTHCHALLENGE ")(input)?;
        let (rest, server_hash) = context(
            "Server hash",
            map(tuple((tag("SERVERHASH="), count(parse_hex, 32))), |x| x.1),
        )(rest)?;

        let (rest, server_nonce) = context(
            "Server hash",
            map(tuple((tag(" SERVERNONCE="), count(parse_hex, 32))), |x| x.1),
        )(rest)?;

        let mut me = Self::default();
        me.server_hash.copy_from_slice(&server_hash[..]);
        me.server_nonce.copy_from_slice(&server_nonce[..]);

        Ok((rest, me))
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
