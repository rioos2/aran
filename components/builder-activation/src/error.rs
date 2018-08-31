// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder activation

use db;
use postgres;
use std::error;
use std::fmt;
use std::io;
use std::num;
use std::result;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    AccountIdFromString(num::ParseIntError),
    IO(io::Error),
    WizardGet(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::AccountIdFromString(ref e) => {
                format!("Cannot convert from string to Account ID, {}", e)
            }
            Error::IO(ref e) => format!("{}", e),
            Error::WizardGet(ref e) => format!("Error getting account from database for wizard , {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::AccountIdFromString(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::WizardGet(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::AccountIdFromString(err)
    }
}
