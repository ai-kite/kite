pub use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Var(std::env::VarError),
    ParseInt(ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Var(err) => write!(f, "{err}"),
            Error::ParseInt(err) => write!(f, "{err}"),
        }
    }
}

impl StdError for Error {}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::Var(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}
