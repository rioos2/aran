// Copyright (c) 2017 RioCorp Inc.

//! A module containing the errors handling for the builder api

use std::error;
use std::fmt;
use std::io;
use std::result;
use rioos_http;
use url;

use common;
use rio_core;
use hyper;
use db;
use service;
pub const MISSING_FIELD: &'static str = "Missing value for field:";
pub const BODYNOTFOUND: &'static str = "nothing found in body";
pub const IDMUSTNUMBER: &'static str = "id must be a number";


#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    Secret(service::Error),
    BadPort(String),
    RioosAranCore(rio_core::Error),
    HabitatHttpClient(rioos_http::Error),
    RioosAranCommon(common::Error),
    HyperError(hyper::error::Error),
    HTTP(hyper::status::StatusCode),
    UrlParseError(url::ParseError),
    IO(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::Secret(ref e) => format!("{}", e),
            Error::RioosAranCore(ref e) => format!("{}", e),
            Error::HabitatHttpClient(ref e) => format!("{}", e),
            Error::RioosAranCommon(ref e) => format!("{}", e),
            Error::HyperError(ref e) => format!("{}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::Secret(ref err) => err.description(),
            Error::HabitatHttpClient(ref err) => err.description(),
            Error::RioosAranCore(ref err) => err.description(),
            Error::RioosAranCommon(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::UrlParseError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
        }
    }
}

impl From<common::Error> for Error {
    fn from(err: common::Error) -> Error {
        Error::RioosAranCommon(err)
    }
}

impl From<rio_core::Error> for Error {
    fn from(err: rio_core::Error) -> Error {
        Error::RioosAranCore(err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Self {
        Error::HyperError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}
