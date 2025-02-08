pub mod error;

use error::*;
use std::{env, num::ParseIntError};

pub fn var(key: &str) -> Result<String> {
    env::var(key).map_err(Error::Var)
}

// FIXME: Only accepts int
pub fn var_as<T>(key: &str) -> Result<T>
where
    T: std::str::FromStr<Err = ParseIntError>,
{
    var(key)?.parse().map_err(Error::ParseInt)
}
