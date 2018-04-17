// Copyright (c) 2018 Rio Advancement Inc
//

use std::error;
use std::fmt;

use hab_net::conn;
use protocol;
use zmq;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Connection(conn::ConnErr),
    Protocol(protocol::ProtocolError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Connection(ref e) => format!("{}", e),
            AppError::Protocol(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::Connection(ref err) => err.description(),
            AppError::Protocol(ref err) => err.description(),
        }
    }
}

impl From<conn::ConnErr> for AppError {
    fn from(err: conn::ConnErr) -> AppError {
        AppError::Connection(err)
    }
}

impl From<protocol::ProtocolError> for AppError {
    fn from(err: protocol::ProtocolError) -> AppError {
        AppError::Protocol(err)
    }
}

impl From<zmq::Error> for AppError {
    fn from(err: zmq::Error) -> AppError {
        Self::from(conn::ConnErr::from(err))
    }
}
