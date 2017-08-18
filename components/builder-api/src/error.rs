// Copyright (c) 2017 RioCorp Inc.

use std::error;
use std::fmt;
use std::io;
use std::result;

use hab_core;
use hyper;
use deploy;
use protocol::net::{self, ErrCode};


#[derive(Debug)]
pub enum Error {
    BadPort(String),
    HabitatCore(hab_core::Error),
    HyperError(hyper::error::Error),
    HTTP(hyper::status::StatusCode),
    IO(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HyperError(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::HabitatCore(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::IO(ref err) => err.description(),
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
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
