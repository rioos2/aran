// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use postgres;
use std::error;
use std::fmt;
use std::result;
use db;
use rio_net;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    NodeCreate(postgres::error::Error),
    NodeList(postgres::error::Error),
    NodeSetStatus(postgres::error::Error),
    NodeGet(postgres::error::Error),
    PromoStatusGetError(rio_net::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::NodeCreate(ref e) => format!("Database error creating a node, {}", e),
            Error::NodeList(ref e) => format!("Database error list nodes, {}", e),
            Error::NodeSetStatus(ref e) => format!("Database error update node status, {}", e),
            Error::NodeGet(ref e) => format!("Database error get node , {}", e),
            Error::PromoStatusGetError(ref e) => format!("Prometheus connection refused , {}", e),
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
            Error::NodeGet(ref err) => err.description(),
            Error::PromoStatusGetError(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
impl From<rio_net::Error> for Error {
    fn from(err: rio_net::Error) -> Error {
        Error::PromoStatusGetError(err)
    }
}
