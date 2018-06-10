// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder api

use httpbis;
use rioos_http;
use serde_json;
use std::error;
use std::fmt;
use std::io;
use std::result;
use std::str::Utf8Error;
use url;

use bodyparser;
use common;
use db;
use openio_sdk_rust::aws;
use reqwest;
use rio_core;
use serde_yaml;
use service;

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
            ErrorMessage::MissingBody => {
                format!("{} {}.", MISSING_BODY, "forgot the payload json ?")
            }
            ErrorMessage::MustBeNumeric(ref m) => format!("{} {}.", MUST_BE_NUMERIC, m),
            ErrorMessage::MustBeAlphanumeric(ref m) => format!("{} {}.", MUST_BE_ALPHANUMERIC, m),
            ErrorMessage::MissingQueryParameter(ref m) => {
                format!("{} {}.", MISSING_QUERY_PARMETER, m)
            }
            ErrorMessage::CannotParseBody(ref m, ref n) => format!("{} {}.", m, n),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    Secret(service::Error),
    BadPort(String),
    MissingConfiguration(String),
    WatchServer(httpbis::Error),
    UNKNOWSECRET,
    SetupNotDone,
    SyncNotDone,
    RioosAranCore(rio_core::Error),
    RioosBodyError(bodyparser::BodyError),
    RioHttpClient(rioos_http::Error),
    RioosAranCommon(common::Error),
    ReqwestError(reqwest::Error),
    OpenIOCredentialsError(aws::errors::creds::CredentialsError),
    OpenIOS3Error(aws::errors::s3::S3Error),
    HTTP(reqwest::StatusCode),
    UrlParseError(url::ParseError),
    IO(io::Error),
    Json(serde_json::Error),
    Utf8Error(Utf8Error),
    Yaml(serde_yaml::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::MissingConfiguration(ref e) => format!("{},", e),
            Error::Secret(ref e) => format!("{}", e),
            Error::WatchServer(ref e) => format!("{}", e),
            Error::RioosAranCore(ref e) => format!("{}", e),
            Error::RioosBodyError(ref e) => format!("{:?}, {:?}", e.detail, e.cause),
            Error::RioHttpClient(ref e) => format!("{}", e),
            Error::RioosAranCommon(ref e) => format!("{}", e),
            Error::ReqwestError(ref e) => format!("{}", e),
            Error::OpenIOCredentialsError(ref e) => format!("{}", e),
            Error::OpenIOS3Error(ref e) => format!("{}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::Utf8Error(ref e) => format!("{}", e),
            Error::UNKNOWSECRET => format!("SecretType not found"),
            Error::SetupNotDone => format!(
                "Rio/OS setup not done. Run `rioos-apiserver setup` before attempting start"
            ),
            Error::SyncNotDone => format!(
                "Rio.Marketplace sync not done. Run `rioos-apiserver sync` before attempting start"
            ),
            Error::Yaml(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::MissingConfiguration(ref err) => err,
            Error::Secret(ref err) => err.description(),
            Error::WatchServer(ref err) => err.description(),
            Error::RioHttpClient(ref err) => err.description(),
            Error::RioosAranCore(ref err) => err.description(),
            Error::RioosBodyError(ref err) => err.description(),
            Error::RioosAranCommon(ref err) => err.description(),
            Error::ReqwestError(ref err) => err.description(),
            Error::OpenIOCredentialsError(ref err) => err.description(),
            Error::OpenIOS3Error(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::UrlParseError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::Utf8Error(ref err) => err.description(),
            Error::UNKNOWSECRET => "Unknown SecretType",
            Error::SetupNotDone => {
                "Rio/OS setup not done. Run `rioos-apiserver setup` before attempting start"
            }
            Error::SyncNotDone => {
                "Rio.Marketplace sync not done. Run `rioos-apiserver sync` before attempting start"
            }

            Error::Yaml(ref err) => err.description(),
        }
    }
}

impl From<common::Error> for Error {
    fn from(err: common::Error) -> Error {
        Error::RioosAranCommon(err)
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Error {
        Error::Db(err)
    }
}

impl From<rio_core::Error> for Error {
    fn from(err: rio_core::Error) -> Error {
        Error::RioosAranCore(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<bodyparser::BodyError> for Error {
    fn from(err: bodyparser::BodyError) -> Self {
        Error::RioosBodyError(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}
