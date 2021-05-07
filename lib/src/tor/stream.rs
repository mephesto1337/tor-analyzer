use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::{map, opt};
use nom::error::{context, ContextError, ParseError};
use nom::sequence::tuple;

use crate::tor::common::{CircuitID, StreamID, Target};
use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq)]
pub enum StreamStatus {
    /// New request to connect
    New,

    /// New request to resolve an address
    NewResolve,

    /// Address re-mapped to another
    Remap,

    /// Sent a connect cell along a circuit
    SentConnect,

    /// Sent a resolve cell along a circuit
    SentResolve,

    /// Received a reply; stream established
    Succeeded,

    /// Stream failed and not retriable
    Failed,

    /// Stream closed
    Closed,

    /// Detached from circuit; still retriable
    Detached,

    /// Waiting for controller to use ATTACHSTREAM (new in 0.4.5.1-alpha)
    ControllerWait,
}

impl NomParse for StreamStatus {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "Circuit status",
            alt((
                map(tag("NEW"), |_| Self::New),
                map(tag("NEWRESOLVE"), |_| Self::NewResolve),
                map(tag("REMAP"), |_| Self::Remap),
                map(tag("SENTCONNECT"), |_| Self::SentConnect),
                map(tag("SENTRESOLVE"), |_| Self::SentResolve),
                map(tag("SUCCEEDED"), |_| Self::Succeeded),
                map(tag("FAILED"), |_| Self::Failed),
                map(tag("CLOSED"), |_| Self::Closed),
                map(tag("DETACHED"), |_| Self::Detached),
                map(tag("CONTROLLER_WAIT"), |_| Self::ControllerWait),
            )),
        )(input)
    }
}

impl fmt::Display for StreamStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => f.write_str("NEW"),
            Self::NewResolve => f.write_str("NEWRESOLVE"),
            Self::Remap => f.write_str("REMAP"),
            Self::SentConnect => f.write_str("SENTCONNECT"),
            Self::SentResolve => f.write_str("SENTRESOLVE"),
            Self::Succeeded => f.write_str("SUCCEEDED"),
            Self::Failed => f.write_str("FAILED"),
            Self::Closed => f.write_str("CLOSED"),
            Self::Detached => f.write_str("DETACHED"),
            Self::ControllerWait => f.write_str("CONTROLLER_WAIT"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Stream {
    pub id: StreamID,
    pub status: StreamStatus,
    pub circuit_id: CircuitID,
    pub target: Target,
}

impl NomParse for Stream {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, _newline) = opt(tag("\r\n"))(input)?;
        let (rest, id) = StreamID::parse(rest)?;
        let (rest, (_, status)) = tuple((space1, StreamStatus::parse))(rest)?;
        let (rest, (_, circuit_id)) = tuple((space1, CircuitID::parse))(rest)?;
        let (rest, (_, target)) = tuple((space1, Target::parse))(rest)?;

        Ok((
            rest,
            Self {
                id,
                status,
                circuit_id,
                target,
            },
        ))
    }
}
