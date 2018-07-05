// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use cidr;
use db;
use oping;
use postgres;
use std::error;
use std::fmt;
use std::result;
use telemetry;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    NodeCreate(postgres::error::Error),
    NodeList(postgres::error::Error),
    NodeSetStatus(postgres::error::Error),
    NodeGet(postgres::error::Error),
    NodeUpdate(postgres::error::Error),
    PromoStatusGetError(telemetry::error::Error),
    PingError(oping::PingError),
    NetworkError(cidr::NetworkParseError),
    SenseiCreate(postgres::error::Error),
    SenseiGet(postgres::error::Error),
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
            Error::NodeUpdate(ref e) => format!("Database error update node , {}", e),
            Error::PromoStatusGetError(ref e) => format!("Prometheus connection refused , {}", e),
            Error::PingError(ref e) => format!("PingError , {}", e),
            Error::NetworkError(ref e) => format!("PingError , {}", e),
            Error::SenseiCreate(ref e) => format!("Database error creating a Sensei, {}", e),
            Error::SenseiGet(ref e) => format!("Database error get sensei , {}", e),
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
            Error::NodeUpdate(ref err) => err.description(),
            Error::NodeSetStatus(ref err) => err.description(),
            Error::NodeGet(ref err) => err.description(),
            Error::PromoStatusGetError(ref err) => err.description(),
            Error::PingError(ref err) => err.description(),
            Error::NetworkError(ref err) => err.description(),
            Error::SenseiCreate(ref err) => err.description(),
            Error::SenseiGet(ref err) => err.description(),
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
        Error::PromoStatusGetError(err)
    }
}

impl From<oping::PingError> for Error {
    fn from(err: oping::PingError) -> Error {
        Error::PingError(err)
    }
}

impl From<cidr::NetworkParseError> for Error {
    fn from(err: cidr::NetworkParseError) -> Error {
        Error::NetworkError(err)
    }
}
