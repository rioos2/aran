// Copyright 2018 The Rio Advancement Inc
//

use std::error;
use std::io;
use std::fmt;
use std::result;

use rio_core;
use reqwest;
use serde_json;
use url;

#[derive(Debug)]
pub enum Error {
    RioosAranCore(rio_core::Error),
    ReqwestError(reqwest::Error),
    /// Occurs when an improper http or https proxy value is given.
    Json(serde_json::Error),
    InvalidProxyValue(String),
    IO(io::Error),
    /// When an error occurs attempting to parse a string into a URL.
    UrlParseError(url::ParseError),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::RioosAranCore(ref e) => format!("{}", e),
            Error::ReqwestError(ref err) => format!("{}", err),
            Error::Json(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::InvalidProxyValue(ref e) => format!("Invalid proxy value: {:?}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RioosAranCore(ref err) => err.description(),
            Error::ReqwestError(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::InvalidProxyValue(_) => "Invalid proxy value",
            Error::UrlParseError(ref err) => err.description(),
        }
    }
}

impl From<rio_core::Error> for Error {
    fn from(err: rio_core::Error) -> Error {
        Error::RioosAranCore(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::UrlParseError(err)
    }
}
