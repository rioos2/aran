// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder authorize

use db;
use postgres;
use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    TeamsCreate(postgres::error::Error),
    TeamsGet(postgres::error::Error),
    TeamGet(postgres::error::Error),
    PermissionsCreate(postgres::error::Error),
    PermissionsGet(postgres::error::Error),
    PermissionGet(postgres::error::Error),
    TeamPermissionsGet(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::TeamsCreate(ref e) => format!("Database error creating a team, {}", e),
            Error::TeamsGet(ref e) => format!("Database error get teams, {}", e),
            Error::TeamGet(ref e) => format!("Database error get team, {}", e),
            Error::PermissionsCreate(ref e) => {
                format!("Database error creating a permission, {}", e)
            }
            Error::TeamPermissionsGet(ref e) => {
                format!("Database error get team based permission, {}", e)
            }
            Error::PermissionsGet(ref e) => format!("Database error get permissions, {}", e),
            Error::PermissionGet(ref e) => format!("Database error get permission, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::TeamsCreate(ref err) => err.description(),
            Error::TeamsGet(ref err) => err.description(),
            Error::TeamGet(ref err) => err.description(),
            Error::PermissionsCreate(ref err) => err.description(),
            Error::PermissionsGet(ref err) => err.description(),
            Error::PermissionGet(ref err) => err.description(),
            Error::TeamPermissionsGet(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
