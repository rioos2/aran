// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the blockchain server

use std::error;
use std::fmt;
use std::io;
use std::str::Utf8Error;
use std::result;

use common;
use rio_core;
use serde_json;

#[derive(Debug)]
pub enum Error {
    RioosBlockchain(rio_core::Error),
    RioosBlockchainCommon(common::Error),
    MissingTls(String),
    IO(io::Error),
    Json(serde_json::Error),
    Utf8Error(Utf8Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::MissingTls(ref e) => format!("{} TLS certificate is missing.", e),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::RioosBlockchain(ref e) => format!("{}", e),
            Error::RioosBlockchainCommon(ref e) => format!("{}", e),
            Error::Utf8Error(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MissingTls(_) => "Tls certificate is missing, Watch server need tls certificate.",
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::Utf8Error(ref err) => err.description(),
            Error::RioosBlockchain(ref err) => err.description(),
            Error::RioosBlockchainCommon(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<common::Error> for Error {
    fn from(err: common::Error) -> Error {
        Error::RioosBlockchainCommon(err)
    }
}

impl From<rio_core::Error> for Error {
    fn from(err: rio_core::Error) -> Error {
        Error::RioosBlockchain(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}
