// Copyright 2018 The Rio Advancement Inc

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io;
use std::result;
use url;

use reqwest;
use serde_json;


#[derive(Debug)]
pub enum Error {
    APIError(reqwest::StatusCode, String),
    PrometheusAPI(reqwest::StatusCode, HashMap<String, String>),
    AnchotreAPI(reqwest::StatusCode, HashMap<String, String>),
    IO(io::Error),
    ReqwestError(reqwest::Error),
    Json(serde_json::Error),
    RequiredConfigField(String),
    CryptoError(String),
    UrlParseError(url::ParseError),
    Sys,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::APIError(ref c, ref m) if m.len() > 0 => format!("[{}] {}", c, m),
            Error::APIError(ref c, _) => format!("[{}]", c),
            Error::PrometheusAPI(ref c, ref m) => format!("[{}] {:?}", c, m),
            Error::AnchotreAPI(ref c, ref m) => format!("[{}] {:?}", c, m),
            Error::ReqwestError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::RequiredConfigField(ref e) => format!("Missing required field in configuration, {}", e),
            Error::CryptoError(ref e) => format!("Crypto error: {}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::Sys => format!("Internal system error"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::APIError(_, _) => "Received a non-2XX response code from API",
            Error::PrometheusAPI(_, _) => "Prometheus API error.",
            Error::AnchotreAPI(_, _) => "Anchore API error.",
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::CryptoError(_) => "Crypto error",
            Error::ReqwestError(ref err) => err.description(),
            Error::RequiredConfigField(_) => "Missing required field in configuration.",
            Error::UrlParseError(ref err) => err.description(),
            Error::Sys => "Internal system error",
        }
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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}
