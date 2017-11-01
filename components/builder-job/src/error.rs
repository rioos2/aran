// Copyright (c) 2017 RioCorp Inc.

//! A module containing the errors handling for the builder scaling
use postgres;
use std::error;
use std::fmt;
use std::result;
use db;


#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    JobsCreate(postgres::error::Error),
    JobSetStatus(postgres::error::Error),
    JobsGet(postgres::error::Error),

}


pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::JobsCreate(ref e) => format!("Database error jobs create, {}", e),
            Error::JobSetStatus(ref e) => format!("Database error status update in jobs, {}", e),
            Error::JobsGet(ref e) => format!("Database error get all the jobs, {}", e),

        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::JobsCreate(ref err) => err.description(),
            Error::JobsGet(ref err) => err.description(),
            Error::JobSetStatus(ref err) => err.description(),
        }
    }
}


impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
