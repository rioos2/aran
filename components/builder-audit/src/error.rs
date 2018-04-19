// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use std::result;
use influx_db_client;

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io;

use reqwest;
use serde_json;
use url;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AnchotreAPI(reqwest::StatusCode, HashMap<String, String>),
    IO(io::Error),
    ReqwestError(reqwest::Error),
    Json(serde_json::Error),
    RequiredConfigField(String),
    CryptoError(String),
    InfluxError(influx_db_client::error::Error),
    UrlParseError(url::ParseError),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::AnchotreAPI(ref c, ref m) => format!("[{}] {:?}", c, m),
            Error::ReqwestError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::RequiredConfigField(ref e) => format!("Missing required field in configuration, {}", e),
            Error::CryptoError(ref e) => format!("Crypto error: {}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::InfluxError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AnchotreAPI(_, _) => "Anchore API error.",
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::CryptoError(_) => "Crypto error",
            Error::ReqwestError(ref err) => err.description(),
            Error::RequiredConfigField(_) => "Missing required field in configuration.",
            Error::UrlParseError(ref err) => err.description(),
            Error::InfluxError(_) => "InfluxError",

        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<influx_db_client::error::Error> for Error {
    fn from(err: influx_db_client::error::Error) -> Error {
        Error::InfluxError(err)
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
