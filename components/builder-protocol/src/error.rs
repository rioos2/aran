// Copyright (c) 2017 RioCorp Inc
//

use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    BadSearchEntity(String),
    BadSearchKey(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadSearchEntity(ref e) => format!("Search not implemented for entity: {}", e),
            Error::BadSearchKey(ref e) => format!("Search not implemented for entity with key: {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadSearchEntity(_) => "Search not implemented for entity.",
            Error::BadSearchKey(_) => "Entity not indexed by the given key.",
        }
    }
}
