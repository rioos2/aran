// Copyright 2018 The Rio Advancement Inc
//


use reqwest;
use rio_core;
use serde_json;
use std::error;
use std::fmt;
use std::io;
use std::result;
use url;

#[derive(Debug)]
pub enum Error {
    APIError(reqwest::StatusCode, String),
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
            Error::APIError(ref c, ref m) if m.len() > 0 => format!("[{}] {}", c, m),
            Error::APIError(ref c, _) => format!("[{}]", c),
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
            Error::APIError(_, _) => "Received a non-2XX response code from API",
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
