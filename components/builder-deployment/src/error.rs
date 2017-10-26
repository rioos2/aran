// Copyright (c) 2017 RioCorp Inc.

//! A module containing the errors handling for the builder deployment

use postgres;
use std::error;
use std::fmt;
use std::result;
use db;


#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    AssemblyCreate(postgres::error::Error),
    AssemblyUpdate(postgres::error::Error),
    AssemblyGet(postgres::error::Error),
    AssemblyFactoryCreate(postgres::error::Error),
    AssemblyFactoryGet(postgres::error::Error),
    PlanGet(postgres::error::Error),
    PlanGetResponse(postgres::error::Error),
    AsmFactorySetStatus(postgres::error::Error),
    AsmSetStatus(postgres::error::Error),
    EndPointsGet(postgres::error::Error),
}


pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::AssemblyCreate(ref e) => format!("Database error creating a new assembly, {}", e),
            Error::AssemblyUpdate(ref e) => format!("Database error updating a assembly, {}", e),
            Error::AssemblyGet(ref e) => format!("Database error getting assembly data, {}", e),
            Error::AssemblyFactoryCreate(ref e) => format!("Database error creating a new assembly factory, {}", e),
            Error::AssemblyFactoryGet(ref e) => format!("Database error getting assembly factory data, {}", e),
            Error::PlanGet(ref e) => format!("Database error getting plan data, {}", e),
            Error::PlanGetResponse(ref e) => format!("Database error listing plan_factory data, {}", e),
            Error::AsmFactorySetStatus(ref e) => format!("Database error setting Assembly Factory status, {}", e),
            Error::EndPointsGet(ref e) => format!("Error retrive endpoint, {}", e),
            Error::AsmSetStatus(ref e) => format!("Database error setting Assembly status, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::AssemblyCreate(ref err) => err.description(),
            Error::AssemblyUpdate(ref err) => err.description(),
            Error::AssemblyGet(ref err) => err.description(),
            Error::AssemblyFactoryCreate(ref err) => err.description(),
            Error::AssemblyFactoryGet(ref err) => err.description(),
            Error::PlanGet(ref err) => err.description(),
            Error::PlanGetResponse(ref err) => err.description(),
            Error::AsmFactorySetStatus(ref err) => err.description(),
            Error::EndPointsGet(ref err) => err.description(),
            Error::AsmSetStatus(ref err) => err.description(),
        }
    }
}


impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
