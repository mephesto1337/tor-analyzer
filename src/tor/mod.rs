use nom::error::{ContextError, ParseError};
macro_rules! impl_from_str {
    ($type:ty) => {
        impl FromStr for $type {
            type Err = $crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use $crate::tor::NomParse;
                Ok(NomParse::parse::<nom::error::VerboseError<&str>>(s)?.1)
            }
        }
    };
}

pub mod circuit;
pub mod common;
pub mod ns;
pub mod stream;
mod utils;

pub trait NomParse: Sized {
    fn parse<'a, E>(input: &'a str) -> nom::IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str> + ContextError<&'a str>;
}
