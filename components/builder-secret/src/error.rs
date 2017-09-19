// Copyright (c) 2017 RioCorp Inc.

//! A module containing the errors handling for the builder scaling
use extern_url;
use rio_core;
use postgres;
use std::error;
use std::fmt;
use std::io;
use std::result;
use db;


#[derive(Debug)]
pub enum Error {
    BadPort(String),
    Db(db::error::Error),
    CaughtPanic(String, String),
    RioosAranCore(rio_core::Error),
    InvalidUrl,
    IO(io::Error),
    SecretCreate(postgres::error::Error),
    HSGet(postgres::error::Error),
    HSSetStatus(postgres::error::Error),
    ProjectJobsGet(postgres::error::Error),
    UnknownVCS,
    UnknownJobState,
}


pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::Db(ref e) => format!("{}", e),
            Error::CaughtPanic(ref msg, ref source) => format!("Caught a panic: {}. {}", msg, source),
            Error::RioosAranCore(ref e) => format!("{}", e),
            Error::InvalidUrl => format!("Bad URL!"),
            Error::IO(ref e) => format!("{}", e),
            Error::SecretCreate(ref e) => format!("Database error creating a horizontal_scaling, {}", e),
            Error::HSGet(ref e) => format!("Database error get horizontal_scaling, {}", e),
            Error::HSSetStatus(ref e) => format!("Database error while update status, {}", e),
            Error::ProjectJobsGet(ref e) => format!("Database error getting jobs for project, {}", e),
            Error::UnknownVCS => format!("Unknown VCS"),
            Error::UnknownJobState => format!("Unknown Job State"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::Db(ref err) => err.description(),
            Error::CaughtPanic(_, _) => "Caught a panic",
            Error::RioosAranCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::InvalidUrl => "Bad Url!",
            Error::SecretCreate(ref err) => err.description(),
            Error::HSGet(ref err) => err.description(),
            Error::HSSetStatus(ref err) => err.description(),
            Error::ProjectJobsGet(ref err) => err.description(),
            Error::UnknownJobState => "Unknown Job State",
            Error::UnknownVCS => "Unknown VCS",
        }
    }
}

impl From<rio_core::Error> for Error {
    fn from(err: rio_core::Error) -> Error {
        Error::RioosAranCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<extern_url::ParseError> for Error {
    fn from(_err: extern_url::ParseError) -> Self {
        Error::InvalidUrl
    }
}
