// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder marketplace

use db;
use postgres;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    MarketPlaceCreate(postgres::error::Error),
    MarketPlaceGet(postgres::error::Error),
    PackageCreate(postgres::error::Error),
    PackageGet(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::MarketPlaceCreate(ref e) => {
                format!("Database error creating a marketplace, {}", e)
            }
            Error::MarketPlaceGet(ref e) => {
                format!("Database error getting marketplace  data, {}", e)
            }
            Error::PackageCreate(ref e) => format!("Database error Package create, {}", e),
            Error::PackageGet(ref e) => format!("Database error getting package  data, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::MarketPlaceCreate(ref err) => err.description(),
            Error::MarketPlaceGet(ref err) => err.description(),
            Error::PackageCreate(ref err) => err.description(),
            Error::PackageGet(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
