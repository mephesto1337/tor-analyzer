use std::fmt;
use std::net::{IpAddr, Ipv6Addr};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::combinator::{map, map_opt, opt, verify};
use nom::error::{context, ContextError, ParseError};
use nom::sequence::tuple;

use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct StreamID(pub String);

impl NomParse for StreamID {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "stream id",
            map(
                verify(alphanumeric1, |s: &str| s.len() <= 16),
                |id: &str| Self(id.into()),
            ),
        )(input)
    }
}

impl fmt::Display for StreamID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CircuitID(pub String);

impl NomParse for CircuitID {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Circuit ID",
            map(
                verify(alphanumeric1, |id: &str| id.len() <= 16),
                |id: &str| Self(id.into()),
            ),
        )(input)
    }
}

impl fmt::Display for CircuitID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Target {
    pub addr: IpAddr,
    pub port: u16,
}

impl NomParse for Target {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, is_ipv6) = opt(tag("["))(input)?;

        let (rest, addr) = if is_ipv6.is_some() {
            let (rest, ip6) = context(
                "Target ipv6",
                map_opt(
                    take_while(|c: char| c.is_ascii_hexdigit() || c == ':'),
                    |s: &str| s.parse::<Ipv6Addr>().ok().map(|ip| IpAddr::V6(ip)),
                ),
            )(rest)?;
            let (rest, _) = tag("]")(rest)?;
            (rest, ip6)
        } else {
            context(
                "Target ip",
                map_opt(
                    take_while(|c: char| c.is_ascii_digit() || c == '.'),
                    |s: &str| s.parse::<IpAddr>().ok(),
                ),
            )(rest)?
        };

        let (rest, port) = context(
            "Target port",
            map(
                tuple((
                    alt((tag(":"), space1)),
                    map_opt(digit1, |s: &str| s.parse::<u16>().ok()),
                )),
                |x| x.1,
            ),
        )(rest)?;

        Ok((rest, Self { addr, port }))
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.addr {
            IpAddr::V4(ref ip4) => write!(f, "{}:{}", ip4, self.port),
            IpAddr::V6(ref ip6) => write!(f, "[{}]:{}", ip6, self.port),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Time {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub mseconds: u32,
}

impl NomParse for Time {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, (year, _, month, _, day, _, hour, _, minute, _, second)) = context(
            "Time created",
            tuple((
                map(verify(digit1, |s: &str| s.len() == 4), |s: &str| {
                    s.parse::<u16>().unwrap()
                }),
                tag("-"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                tag("-"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                alt((tag("T"), tag(" "))),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                tag(":"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
                tag(":"),
                map(verify(digit1, |s: &str| s.len() <= 2), |s: &str| {
                    s.parse::<u8>().unwrap()
                }),
            )),
        )(input)?;
        let (rest, opt_mseconds) = opt(tuple((
            tag("."),
            map(verify(digit1, |s: &str| s.len() == 6), |s: &str| {
                s.parse::<u32>().unwrap()
            }),
        )))(rest)?;
        let mseconds = opt_mseconds.map(|x| x.1).unwrap_or_default();

        Ok((
            rest,
            Self {
                year,
                month,
                day,
                hour,
                minute,
                second,
                mseconds,
            },
        ))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.mseconds
        )
    }
}
