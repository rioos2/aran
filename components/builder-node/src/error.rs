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
    NodeCreate(postgres::error::Error),
    NodeList(postgres::error::Error),
    NodeSetStatus(postgres::error::Error),
}


pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::NodeCreate(ref e) => format!("Database error creating a node, {}", e),
            Error::NodeList(ref e) => format!("Database error list nodes, {}", e),
            Error::NodeSetStatus(ref e) => format!("Database error update node status, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::NodeCreate(ref err) => err.description(),
            Error::NodeList(ref err) => err.description(),
            Error::NodeSetStatus(ref err) => err.description(),
        }
    }
}


impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
