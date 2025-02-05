use std::error::Error as StdError;
use std::fmt::{self, Display};

use common::env::error::Error as EnvError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Env(EnvError),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Gemini(google_generative_ai_rs::v1::errors::GoogleAPIError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Env(err) => write!(f, "{err}"),
            Error::Reqwest(err) => write!(f, "{err}"),
            Error::SerdeJson(err) => write!(f, "{err}"),
            Error::Gemini(err) => write!(f, "{err}"),
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<google_generative_ai_rs::v1::errors::GoogleAPIError> for Error {
    fn from(err: google_generative_ai_rs::v1::errors::GoogleAPIError) -> Self {
        Self::Gemini(err)
    }
}
