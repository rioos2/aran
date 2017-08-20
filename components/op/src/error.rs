// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors


use std::error;
use std::fmt;
use std::result;

use rcore;

#[derive(Debug)]
pub enum Error {
    NoFile,
    NoOrigin,
    HabitatCore(rcore::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::NoFile => format!("No file was specified to hash"),
            Error::NoOrigin => format!("No origin was specified to get a shard from"),
            Error::HabitatCore(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NoFile => "No file was specified to hash",
            Error::NoOrigin => "No origin was specified to get a shard from",
            Error::HabitatCore(ref err) => err.description(),
        }
    }
}

impl From<rcore::Error> for Error {
    fn from(err: rcore::Error) -> Error {
        Error::HabitatCore(err)
    }
}
