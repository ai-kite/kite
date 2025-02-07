use std::error::Error as StdError;
use std::fmt::{self, Display};

use indexer::error::Error as IndexerError;
use llm::error::Error as LLMError;
use toml::de::Error as TomlError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Indexer(IndexerError),
    LLM(LLMError),
    TomlParse(TomlError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Indexer(err) => write!(f, "{err}"),
            Error::LLM(err) => write!(f, "{err}"),
            Error::TomlParse(err) => write!(f, "{err}"),
        }
    }
}

impl StdError for Error {}

impl From<IndexerError> for Error {
    fn from(err: IndexerError) -> Self {
        Self::Indexer(err)
    }
}

impl From<LLMError> for Error {
    fn from(err: LLMError) -> Self {
        Self::LLM(err)
    }
}

impl From<TomlError> for Error {
    fn from(err: TomlError) -> Self {
        Self::TomlParse(err)
    }
}
