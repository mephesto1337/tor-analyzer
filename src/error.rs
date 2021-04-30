#[derive(Debug)]
pub enum Error {
    Connection(torut::control::ConnError),
    Io(std::io::Error),
    Incomplete(nom::Needed),
    Parsing {
        data: String,
        kind: nom::error::ErrorKind,
    },
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::convert::From<torut::control::ConnError> for Error {
    fn from(e: torut::control::ConnError) -> Self {
        Self::Connection(e)
    }
}

fn get_input_error(s: &str) -> String {
    s.into()
    // const MAX_STRING_LEN: usize = 16usize;
    // if s.len() <= MAX_STRING_LEN {
    //     s.into()
    // } else {
    //     (&s[..MAX_STRING_LEN]).into()
    // }
}

impl std::convert::From<nom::Err<nom::error::Error<&'_ str>>> for Error {
    fn from(e: nom::Err<nom::error::Error<&'_ str>>) -> Self {
        match e {
            nom::Err::Incomplete(n) => Self::Incomplete(n),
            nom::Err::Error(e) => Self::Parsing {
                data: get_input_error(e.input),
                kind: e.code,
            },
            nom::Err::Failure(e) => Self::Parsing {
                data: get_input_error(e.input),
                kind: e.code,
            },
        }
    }
}

impl std::convert::From<nom::Err<(&'_ str, nom::error::ErrorKind)>> for Error {
    fn from(e: nom::Err<(&'_ str, nom::error::ErrorKind)>) -> Self {
        match e {
            nom::Err::Incomplete(n) => Self::Incomplete(n),
            nom::Err::Error((data, kind)) => Self::Parsing {
                data: get_input_error(data),
                kind,
            },
            nom::Err::Failure((data, kind)) => Self::Parsing {
                data: get_input_error(data),
                kind,
            },
        }
    }
}
