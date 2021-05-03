use nom::branch::alt;
use nom::bytes::complete::{escaped, tag};
use nom::character::complete::{alphanumeric1, digit1, none_of, one_of, space1};
use nom::combinator::{map, map_opt, opt, verify};
use nom::error::context;
use nom::multi::{count, separated_list1};
use nom::sequence::tuple;

use crate::tor::utils::{base32_word, parse_hex, word};
use crate::tor::NomParse;

#[derive(Debug)]
pub struct ControlResponse {
    pub code: u16,
    pub lines: Vec<String>,
}

impl NomParse for ControlResponse {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (rest, code) = map(verify(digit1, |s: &str| s.len() == 3), |s: &str| {
            s.parse::<u16>()
        })(input)?;
    }
}
