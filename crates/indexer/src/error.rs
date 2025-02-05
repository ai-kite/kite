use std::error::Error as StdError;
use std::fmt::{self, Display};

use common::env::error::Error as EnvError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(EnvError),
    Reqwest(reqwest::Error),
    InvalidDatasetRange,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Env(err) => write!(f, "{err}"),
            Error::Reqwest(err) => write!(f, "{err}"),
            Error::InvalidDatasetRange => write!(
                f,
                "Dataset height is less than the contract deployment block"
            ),
        }
    }
}

impl StdError for Error {}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Self {
        Self::Env(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}
