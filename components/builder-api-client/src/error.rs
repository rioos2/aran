// Copyright 2018 The Rio Advancement Inc
//

use std::error;
use std::io;
use std::fmt;
use std::result;

use reqwest;
use serde_json;
use url;

use rioos_http;
use http_gateway;

#[derive(Debug)]
pub enum Error {
    RioHttpClient(rioos_http::Error),
    RioNetError(http_gateway::Error),
    ReqwestError(reqwest::Error),
    IO(io::Error),
    Json(serde_json::Error),
    UrlParseError(url::ParseError),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::RioNetError(ref e) => format!("{}", e),
            Error::RioHttpClient(ref e) => format!("{}", e),
            Error::ReqwestError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RioHttpClient(ref err) => err.description(),
            Error::RioNetError(ref err) => err.description(),
            Error::ReqwestError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::UrlParseError(ref err) => err.description(),
        }
    }
}

impl From<rioos_http::Error> for Error {
    fn from(err: rioos_http::Error) -> Error {
        Error::RioHttpClient(err)
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<http_gateway::Error> for Error {
    fn from(err: http_gateway::Error) -> Error {
        Error::RioNetError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}
