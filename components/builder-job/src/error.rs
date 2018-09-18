// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling

use db;
use postgres;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    JobsCreate(postgres::error::Error),
    JobSetStatus(postgres::error::Error),
    JobsGet(postgres::error::Error),
    JobError(String),
    METRICLIMITERROR,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::JobError(ref e) => format!("{}", e),
            Error::JobsCreate(ref e) => format!("Database error jobs create, {}", e),
            Error::JobSetStatus(ref e) => format!("Database error status update in jobs, {}", e),
            Error::JobsGet(ref e) => format!("Database error get all the jobs, {}", e),
            Error::METRICLIMITERROR => format!("Metric limit not satisfied for the assembly"),
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
            Error::METRICLIMITERROR => "Metric limit not satisfied for the assembly",
            Error::JobError(ref err) => err,
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
