use nom::branch::alt;
use nom::bytes::streaming::{tag, take_until};
use nom::character::streaming::digit1;
use nom::combinator::{map, verify};
use nom::error::{context, ContextError, ParseError};
use nom::sequence::tuple;

use crate::tor::NomParse;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct StatusCode(pub(crate) u16);

impl NomParse for StatusCode {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        context(
            "status code",
            map(verify(digit1, |s: &str| s.len() == 3), |s: &str| {
                Self(s.parse::<u16>().unwrap())
            }),
        )(input)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ResponseLine {
    SingleLine { code: StatusCode, data: String },
    MultiLine { code: StatusCode, data: String },
    End { code: StatusCode, data: String },
}

impl ResponseLine {
    pub(crate) fn get_code(&self) -> u16 {
        match self {
            Self::SingleLine { code, .. } => code.0,
            Self::MultiLine { code, .. } => code.0,
            Self::End { code, .. } => code.0,
        }
    }

    pub(crate) fn take_data(self) -> String {
        match self {
            Self::SingleLine { data, .. } => data,
            Self::MultiLine { data, .. } => data,
            Self::End { data, .. } => data,
        }
    }

    pub(crate) fn is_end(&self) -> bool {
        if let Self::End { .. } = self {
            true
        } else {
            false
        }
    }
}

impl NomParse for ResponseLine {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>,
    {
        let (rest, code) = StatusCode::parse(input)?;
        let code = code.0;
        let (rest, response_line) = alt((
            map(
                context(
                    "single line",
                    tuple((tag("-"), take_until("\r\n"), tag("\r\n"))),
                ),
                |x: (&str, &str, &str)| Self::SingleLine {
                    code: StatusCode(code),
                    data: x.1.into(),
                },
            ),
            map(
                context(
                    "multi line",
                    tuple((tag("+"), take_until("\r\n.\r\n"), tag("\r\n.\r\n"))),
                ),
                |x: (&str, &str, &str)| Self::MultiLine {
                    code: StatusCode(code),
                    data: x.1.into(),
                },
            ),
            map(
                context(
                    "end line",
                    tuple((tag(" "), take_until("\r\n"), tag("\r\n"))),
                ),
                |x: (&str, &str, &str)| Self::End {
                    code: StatusCode(code),
                    data: x.1.into(),
                },
            ),
        ))(rest)?;

        Ok((rest, response_line))
    }
}
