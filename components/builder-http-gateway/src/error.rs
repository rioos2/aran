// Copyright 2018 The Rio Advancement Inc

use std::error;
use std::fmt;
use std::io;
use std::result;
use url;

use serde_json;

//use auth;
use auth::rioos::AuthErr;

#[derive(Debug)]
pub enum Error {
    Auth(AuthErr),
    IO(io::Error),
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
            Error::Auth(ref e) => format!("Rio/OS authorization error, {}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::RequiredConfigField(ref e) => {
                format!("Missing required field in configuration, {}", e)
            }
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
            Error::Auth(_) => "Rio/OS authorization error.",
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::CryptoError(_) => "Crypto error",
            Error::RequiredConfigField(_) => "Missing required field in configuration.",
            Error::UrlParseError(ref err) => err.description(),
            Error::Sys => "Internal system error",
        }
    }
}
