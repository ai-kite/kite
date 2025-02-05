pub mod error;

use error::*;
use std::{env, num::ParseIntError};

pub fn var(key: &str) -> Result<String> {
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(err) => return Err(Error::Var(err)),
    }
}

pub fn var_as<T>(key: &str) -> Result<T>
where
    T: std::str::FromStr<Err = ParseIntError>,
{
    let val = var(key)?;
    match val.parse::<T>() {
        Ok(v) => Ok(v),
        Err(err) => return Err(Error::ParseInt(err)),
    }
}
