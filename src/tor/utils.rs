use nom::bytes::complete::{take, take_while};
use nom::combinator::map_opt;

pub(crate) fn word(s: &str) -> nom::IResult<&str, &str> {
    take_while(|c: char| c.is_ascii_alphanumeric() || c == '_')(s)
}

pub(crate) fn base32_word(s: &str) -> nom::IResult<&str, &str> {
    take_while(|c: char| {
        c.is_ascii_uppercase()
            || c == '2'
            || c == '3'
            || c == '4'
            || c == '5'
            || c == '6'
            || c == '7'
    })(s)
}

pub(crate) fn parse_hex(s: &str) -> nom::IResult<&str, u8> {
    let (rest, h) = map_opt(take(2usize), |h| {
        let x = u8::from_str_radix(h, 16).ok();
        x
    })(s)?;
    Ok((rest, h))
}
