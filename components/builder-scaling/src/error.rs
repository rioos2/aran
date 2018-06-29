// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use db;
use postgres;
use std::error;
use std::fmt;
use std::result;
use telemetry;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    HSCreate(postgres::error::Error),
    HSGet(postgres::error::Error),
    HSSetStatus(postgres::error::Error),
    HSUpdate(postgres::error::Error),
    VSCreate(postgres::error::Error),
    VSGet(postgres::error::Error),
    VSSetStatus(postgres::error::Error),
    VSUpdate(postgres::error::Error),
    RioNetError(telemetry::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::HSCreate(ref e) => {
                format!("Database error creating a horizontal_scaling, {}", e)
            }
            Error::HSGet(ref e) => format!("Database error get horizontal_scaling, {}", e),
            Error::HSSetStatus(ref e) => format!("Database error while update status, {}", e),
            Error::HSUpdate(ref e) => {
                format!("Database error while update horizontal scaling, {}", e)
            }
            Error::VSCreate(ref e) => format!("Database error creating a vertical_scaling, {}", e),
            Error::VSGet(ref e) => format!("Database error get vertical_scaling, {}", e),
            Error::VSSetStatus(ref e) => {
                format!("Database error while vertical scaling update status, {}", e)
            }
            Error::RioNetError(ref e) => format!("Prometheus connection refused , {}", e),
            Error::VSUpdate(ref e) => {
                format!("Database error while update vertical scaling, {}", e)
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::HSCreate(ref err) => err.description(),
            Error::HSGet(ref err) => err.description(),
            Error::HSSetStatus(ref err) => err.description(),
            Error::HSUpdate(ref err) => err.description(),
            Error::VSCreate(ref err) => err.description(),
            Error::VSGet(ref err) => err.description(),
            Error::VSSetStatus(ref err) => err.description(),
            Error::RioNetError(ref err) => err.description(),
            Error::VSUpdate(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<telemetry::error::Error> for Error {
    fn from(err: telemetry::error::Error) -> Error {
        Error::RioNetError(err)
    }
}
