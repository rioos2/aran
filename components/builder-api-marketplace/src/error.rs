// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder api

use std::error;
use std::fmt;
use std::io;
use std::result;
use url;

use common;
use rio_core;
use db;
use bodyparser;

const MISSING_PARAMETER: &'static str = "Missing parameters: ";
const MISSING_BODY: &'static str = "Missing body, empty: ";
const MUST_BE_NUMERIC: &'static str = "Must be a numeric: ";
const MISSING_QUERY_PARMETER: &'static str = "Missing query parameters:";
const MUST_BE_ALPHANUMERIC: &'static str = "Must be alpha numeric : ";

pub enum ErrorMessage {
    MissingParameter(String),
    MissingBody,
    MustBeNumeric(String),
    MustBeAlphanumeric(String),
    MissingQueryParameter(String),
    CannotParseBody(String, String),
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        match *self {
            ErrorMessage::MissingParameter(ref m) => format!("{} {}.", MISSING_PARAMETER, m),
            ErrorMessage::MissingBody => format!("{} {}.", MISSING_BODY, "forgot the payload json ?"),
            ErrorMessage::MustBeNumeric(ref m) => format!("{} {}.", MUST_BE_NUMERIC, m),
            ErrorMessage::MustBeAlphanumeric(ref m) => format!("{} {}.", MUST_BE_ALPHANUMERIC, m),
            ErrorMessage::MissingQueryParameter(ref m) => format!("{} {}.", MISSING_QUERY_PARMETER, m),
            ErrorMessage::CannotParseBody(ref m, ref n) => format!("{} {}.", m, n),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    BadPort(String),
    RioosAranCore(rio_core::Error),
    RioosBodyError(bodyparser::BodyError),
    RioosAranCommon(common::Error),
    UrlParseError(url::ParseError),
    IO(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::RioosAranCore(ref e) => format!("{}", e),
            Error::RioosBodyError(ref e) => format!("{:?}, {:?}", e.detail, e.cause),
            Error::RioosAranCommon(ref e) => format!("{}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
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
            Error::RioosAranCore(ref err) => err.description(),
            Error::RioosBodyError(ref err) => err.description(),
            Error::RioosAranCommon(ref err) => err.description(),
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

impl From<bodyparser::BodyError> for Error {
    fn from(err: bodyparser::BodyError) -> Self {
        Error::RioosBodyError(err)
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Error {
        Error::Db(err)
    }
}
