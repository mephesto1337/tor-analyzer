use std::fmt;

use nom::bytes::complete::{escaped, tag, take, take_while};
use nom::character::complete::{none_of, one_of};
use nom::combinator::map_opt;
use nom::error::{ContextError, ParseError};
use nom::sequence::tuple;

pub(crate) fn word<'a, E>(s: &'a str) -> nom::IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    take_while(|c: char| c.is_ascii_alphanumeric() || c == '_')(s)
}

/*
pub(crate) fn no_space<'a, E>(s: &'a str) -> nom::IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    take_while(|c: char| !c.is_ascii_whitespace())(s)
}
*/

pub(crate) fn base64_word<'a, E>(s: &'a str) -> nom::IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    take_while(|c: char| c.is_ascii_alphanumeric() || c == '/' || c == '+' || c == '=')(s)
}

pub(crate) fn base32_word<'a, E>(s: &'a str) -> nom::IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    take_while(|c: char| {
        c.is_ascii_lowercase()
            || c == '2'
            || c == '3'
            || c == '4'
            || c == '5'
            || c == '6'
            || c == '7'
    })(s)
}

pub(crate) fn parse_hex<'a, E>(s: &'a str) -> nom::IResult<&'a str, u8, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let (rest, h) = map_opt(take(2usize), |h| {
        let x = u8::from_str_radix(h, 16).ok();
        x
    })(s)?;
    Ok((rest, h))
}

pub(crate) fn quoted_string<'a, E>(s: &'a str) -> nom::IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    let (rest, (_, string, _)) = tuple((
        tag("\""),
        escaped(none_of("\\\""), '\\', one_of("\\\"")),
        tag("\""),
    ))(s)?;

    Ok((rest, string))
}

pub(crate) fn hex_encode_inplace<F: std::fmt::Write, B: AsRef<[u8]>>(
    f: &mut F,
    s: B,
) -> fmt::Result {
    let bytes = s.as_ref();
    for b in bytes.iter() {
        write!(f, "{:02X}", b)?;
    }
    Ok(())
}

pub fn hex_encode<B: AsRef<[u8]>>(s: B) -> String {
    let bytes = s.as_ref();
    let mut hex = String::with_capacity(bytes.len() * 2);
    hex_encode_inplace(&mut hex, bytes).unwrap();
    hex
}

pub(crate) fn parse_single_key_value(s: &str) -> Option<(&str, &str)> {
    if let Some(idx) = s.find('=') {
        let key = &s[..idx];
        let val = &s[(idx + 1)..];
        Some((key, val))
    } else {
        None
    }
}
