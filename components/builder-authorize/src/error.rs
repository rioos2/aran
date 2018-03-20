// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder authorize

use postgres;
use std::error;
use std::fmt;
use std::result;
use db;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    RolesCreate(postgres::error::Error),
    RolesGet(postgres::error::Error),
    RoleGet(postgres::error::Error),
    PermissionsCreate(postgres::error::Error),
    PermissionsGet(postgres::error::Error),
    PermissionGet(postgres::error::Error),
    RolePermissionsGet(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::RolesCreate(ref e) => format!("Database error creating a role, {}", e),
            Error::RolesGet(ref e) => format!("Database error get roles, {}", e),
            Error::RoleGet(ref e) => format!("Database error get role, {}", e),
            Error::PermissionsCreate(ref e) => format!("Database error creating a permission, {}", e),
            Error::RolePermissionsGet(ref e) => format!("Database error get role based permission, {}", e),
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
            Error::RolesCreate(ref err) => err.description(),
            Error::RolesGet(ref err) => err.description(),
            Error::RoleGet(ref err) => err.description(),
            Error::PermissionsCreate(ref err) => err.description(),
            Error::PermissionsGet(ref err) => err.description(),
            Error::PermissionGet(ref err) => err.description(),
            Error::RolePermissionsGet(ref err) => err.description(),
        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}
