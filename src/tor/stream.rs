use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::{map, opt};
use nom::error::{context, ContextError, ParseError};
use nom::sequence::tuple;

use crate::tor::common::{CircuitID, StreamID, Target};
use crate::tor::utils::word;
use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq)]
pub struct Stream {
    pub id: StreamID,
    pub status: String,
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

        let (rest, (_, status)) = context(
            "stream status",
            tuple((space1, map(word, |s: &str| s.to_owned()))),
        )(rest)?;

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
