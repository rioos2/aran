// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use std::result;
use influx_db_client;

use std::error;
use std::fmt;
use std::io;
use http_client;

use serde_json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Json(serde_json::Error),
    RequiredConfigField(String),
    CryptoError(String),
    InfluxError(influx_db_client::error::Error),
    RioHttpClient(http_client::Error),
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::RequiredConfigField(ref e) => format!("Missing required field in configuration, {}", e),
            Error::CryptoError(ref e) => format!("Crypto error: {}", e),
            Error::InfluxError(ref e) => format!("{}", e),
            Error::RioHttpClient(ref e) => format!("{}", e),

        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::CryptoError(_) => "Crypto error",
            Error::RequiredConfigField(_) => "Missing required field in configuration.",
            Error::InfluxError(_) => "InfluxError",
            Error::RioHttpClient(ref err) => err.description(),

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

impl From<http_client::Error> for Error {
    fn from(err: http_client::Error) -> Error {
        Error::RioHttpClient(err)
    }
}
