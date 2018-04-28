// Copyright 2018 The Rio Advancement Inc
//

use std::error;
use std::fmt;
use std::result;

use serde_json;

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    RequiredConfigField(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Json(ref e) => format!("{}", e),
            Error::RequiredConfigField(ref e) => format!("Missing required field in configuration, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Json(ref err) => err.description(),
            Error::RequiredConfigField(_) => "Missing required field in configuration.",
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
