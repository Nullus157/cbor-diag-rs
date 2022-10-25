use std::{borrow::Cow, fmt};

#[derive(Debug)]
pub enum Error {
    Todo(Cow<'static, str>),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Error {
        Error::Todo(err.into())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::Todo(err.into())
    }
}

impl From<data_encoding::DecodeError> for Error {
    fn from(err: data_encoding::DecodeError) -> Error {
        err.to_string().into()
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Todo(s) => write!(f, "TODO cbor-diag::Error: {s}"),
        }
    }
}
