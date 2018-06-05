use std::error::Error as StdError;
use std::result::Result as StdResult;

#[derive(Debug)]
pub enum Error {
    Todos(&'static str),
    Todo(Box<StdError>),
}

pub type Result<T> = StdResult<T, Error>;

impl<T> From<T> for Error
where
    T: StdError + 'static,
{
    fn from(err: T) -> Error {
        Error::Todo(Box::new(err))
    }
}
