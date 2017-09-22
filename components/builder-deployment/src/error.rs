// Copyright (c) 2017 RioCorp Inc.

//! A module containing the errors handling for the builder deployment

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
    AssemblyCreate(postgres::error::Error),
    AssemblyGet(postgres::error::Error),
    AssemblyFactoryCreate(postgres::error::Error),
    AssemblyFactoryGet(postgres::error::Error),
    PlanGet(postgres::error::Error),
    PlanGetResponse(postgres::error::Error),
    AsmFactorySetStatus(postgres::error::Error),
    AsmSetStatus(postgres::error::Error),
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
            Error::AssemblyCreate(ref e) => format!("Database error creating a new assembly, {}", e),
            Error::AssemblyGet(ref e) => format!("Database error getting assembly data, {}", e),
            Error::AssemblyFactoryCreate(ref e) => format!("Database error creating a new assembly factory, {}", e),
            Error::AssemblyFactoryGet(ref e) => format!("Database error getting assembly factory data, {}", e),
            Error::PlanGet(ref e) => format!("Database error getting plan data, {}", e),
            Error::PlanGetResponse(ref e) => format!("Database error listing plan_factory data, {}", e),
            Error::AsmFactorySetStatus(ref e) => format!("Database error setting Assembly Factory status, {}", e),
            Error::AsmSetStatus(ref e) => format!("Database error setting Assembly status, {}", e),
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
            Error::AssemblyCreate(ref err) => err.description(),
            Error::AssemblyGet(ref err) => err.description(),
            Error::AssemblyFactoryCreate(ref err) => err.description(),
            Error::AssemblyFactoryGet(ref err) => err.description(),
            Error::PlanGet(ref err) => err.description(),
            Error::PlanGetResponse(ref err) => err.description(),
            Error::AsmFactorySetStatus(ref err) => err.description(),
            Error::AsmSetStatus(ref err) => err.description(),
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
