use std::fmt;

#[derive(Debug)]
pub enum Error {
    Connection(torut::control::ConnError),
    Protocol(u16),
    Io(std::io::Error),
    Incomplete(nom::Needed),
    Parsing {
        data: String,
        kind: nom::error::VerboseErrorKind,
        trace: Option<String>,
    },
    Base64(base64::DecodeError),
}

pub type Result<T> = std::result::Result<T, Error>;

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

impl std::convert::From<nom::Err<nom::error::VerboseError<&'_ str>>> for Error {
    fn from(e: nom::Err<nom::error::VerboseError<&'_ str>>) -> Self {
        match e {
            nom::Err::Incomplete(n) => Self::Incomplete(n),
            nom::Err::Error(mut e) => {
                let (input, kind) = e.errors.pop().unwrap();
                Self::Parsing {
                    data: get_input_error(input),
                    kind,
                    trace: Some(nom::error::convert_error(input, e)),
                }
            }
            nom::Err::Failure(mut e) => {
                let (input, kind) = e.errors.pop().unwrap();
                Self::Parsing {
                    data: get_input_error(input),
                    kind,
                    trace: Some(nom::error::convert_error(input, e)),
                }
            }
        }
    }
}

impl std::convert::From<nom::Err<nom::error::Error<&'_ str>>> for Error {
    fn from(e: nom::Err<nom::error::Error<&'_ str>>) -> Self {
        match e {
            nom::Err::Incomplete(n) => Self::Incomplete(n),
            nom::Err::Error(e) => Self::Parsing {
                data: get_input_error(e.input),
                kind: nom::error::VerboseErrorKind::Nom(e.code),
                trace: None,
            },
            nom::Err::Failure(e) => Self::Parsing {
                data: get_input_error(e.input),
                kind: nom::error::VerboseErrorKind::Nom(e.code),
                trace: None,
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
                kind: nom::error::VerboseErrorKind::Nom(kind),
                trace: None,
            },
            nom::Err::Failure((data, kind)) => Self::Parsing {
                data: get_input_error(data),
                kind: nom::error::VerboseErrorKind::Nom(kind),
                trace: None,
            },
        }
    }
}

impl std::convert::From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Self::Base64(e)
    }
}

impl nom::error::ParseError<&str> for Error {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        Self::Parsing {
            data: input.into(),
            kind: nom::error::VerboseErrorKind::Nom(kind),
            trace: None,
        }
    }
    fn append(_input: &str, _e: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connection(ref ce) => write!(f, "Connection: {:?}", ce),
            Self::Protocol(ref code) => write!(f, "Protocol error: code={}", code),
            Self::Io(ref io) => write!(f, "IO: {}", io),
            Self::Incomplete(ne) => match ne {
                nom::Needed::Unknown => write!(f, "Missing bytes"),
                nom::Needed::Size(ref s) => write!(f, "Missing {} bytes", s),
            },
            Self::Parsing {
                ref data,
                kind,
                ref trace,
            } => {
                let kind = match kind {
                    nom::error::VerboseErrorKind::Nom(kind) => format!("{:?}", kind),
                    nom::error::VerboseErrorKind::Context(ctx) => (*ctx).into(),
                    nom::error::VerboseErrorKind::Char(c) => format!("Bad char {:?}", c),
                };
                write!(f, "Parsing: {} at {:?}", kind, data)?;
                if let Some(trace) = trace.as_ref() {
                    write!(f, "\n{}", trace)?;
                }
                Ok(())
            }
            Self::Base64(be) => write!(f, "Base64: {}", be),
        }
    }
}
