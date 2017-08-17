// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use extern_url;
use hab_core;
use hab_net;
use postgres;
use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::result;
use db;


#[derive(Debug)]
pub enum Error {
    BadPort(String),
    Db(db::error::Error),
    CaughtPanic(String, String),
    HabitatCore(hab_core::Error),
    InvalidUrl,
    IO(io::Error),
    RolesCreate(postgres::error::Error),
    RolesGet(postgres::error::Error),
    PermissionsCreate(postgres::error::Error),
    PermissionsGet(postgres::error::Error),
    RolePermissionsGet(postgres::error::Error),
    HSSetStatus(postgres::error::Error),
    NetError(hab_net::Error),
    ProjectJobsGet(postgres::error::Error),
    UnknownVCS,
    UnknownJobState,
}


pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::Db(ref e) => format!("{}", e),
            Error::CaughtPanic(ref msg, ref source) => format!("Caught a panic: {}. {}", msg, source),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::InvalidUrl => format!("Bad URL!"),
            Error::IO(ref e) => format!("{}", e),
            Error::RolesCreate(ref e) => format!("Database error creating a role, {}", e),
            Error::RolesGet(ref e) => format!("Database error get role, {}", e),
            Error::PermissionsCreate(ref e) => format!("Database error creating a permission, {}", e),
            Error::RolePermissionsGet(ref e) => format!("Database error get role based permission, {}", e),
            Error::PermissionsGet(ref e) => format!("Database error get permissions, {}", e),
            Error::HSSetStatus(ref e) => format!("Database error while update status, {}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::ProjectJobsGet(ref e) => format!("Database error getting jobs for project, {}", e),
            Error::UnknownVCS => format!("Unknown VCS"),
            Error::UnknownJobState => format!("Unknown Job State"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::Db(ref err) => err.description(),
            Error::CaughtPanic(_, _) => "Caught a panic",
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::InvalidUrl => "Bad Url!",
            Error::RolesCreate(ref err) => err.description(),
            Error::RolesGet(ref err) => err.description(),
            Error::PermissionsCreate(ref err) => err.description(),
            Error::PermissionsGet(ref err) => err.description(),
            Error::RolePermissionsGet(ref err) => err.description(),
            Error::HSSetStatus(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::ProjectJobsGet(ref err) => err.description(),
            Error::UnknownJobState => "Unknown Job State",
            Error::UnknownVCS => "Unknown VCS",
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Self {
        Error::NetError(err)
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<extern_url::ParseError> for Error {
    fn from(_err: extern_url::ParseError) -> Self {
        Error::InvalidUrl
    }
}
