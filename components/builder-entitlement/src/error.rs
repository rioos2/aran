// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder license

use postgres;
use std::error;
use std::fmt;
use std::result;
use db;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    LicenseCreate(postgres::error::Error),
    LicenseUpdate(postgres::error::Error),
    LicenseGet(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::LicenseCreate(ref e) => format!("Database error creating a license, {}", e),
            Error::LicenseUpdate(ref e) => format!("Database error update license status, {}", e),
            Error::LicenseGet(ref e) => format!("Database error get license, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::LicenseCreate(ref err) => err.description(),
            Error::LicenseUpdate(ref err) => err.description(),
            Error::LicenseGet(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
