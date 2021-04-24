pub mod circuit;
mod utils;

pub trait NomParse: Sized {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}

pub struct Router;
