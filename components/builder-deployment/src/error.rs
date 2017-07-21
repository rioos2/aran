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
    AssemblyCreate(postgres::error::Error),
    AssemblyGet(postgres::error::Error),
    AssemblyFactoryCreate(postgres::error::Error),
    AssemblyFactoryGet(postgres::error::Error),
    JobMarkArchived(postgres::error::Error),
    JobPending(postgres::error::Error),
    JobReset(postgres::error::Error),
    JobSetLogUrl(postgres::error::Error),
    JobSetState(postgres::error::Error),
    LogDirDoesNotExist(PathBuf, io::Error),
    LogDirIsNotDir(PathBuf),
    LogDirNotWritable(PathBuf),
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
            Error::AssemblyCreate(ref e) => format!("Database error creating a new assembly, {}", e),
            Error::AssemblyGet(ref e) => format!("Database error getting assembly data, {}", e),
            Error::AssemblyFactoryCreate(ref e) => format!("Database error creating a new assembly factory, {}", e),
            Error::AssemblyFactoryGet(ref e) => format!("Database error getting assembly factory data, {}", e),
            Error::JobMarkArchived(ref e) => format!("Database error marking job as archived, {}", e),
            Error::JobPending(ref e) => format!("Database error getting pending jobs, {}", e),
            Error::JobReset(ref e) => format!("Database error reseting jobs, {}", e),
            Error::JobSetLogUrl(ref e) => format!("Database error setting job log URL, {}", e),
            Error::JobSetState(ref e) => format!("Database error setting job state, {}", e),
            Error::LogDirDoesNotExist(ref path, ref e) => format!("Build log directory {:?} doesn't exist!: {:?}", path, e),
            Error::LogDirIsNotDir(ref path) => format!("Build log directory {:?} is not a directory!", path),
            Error::LogDirNotWritable(ref path) => format!("Build log directory {:?} is not writable!", path),
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
            Error::AssemblyCreate(ref err) => err.description(),
            Error::AssemblyGet(ref err) => err.description(),
            Error::AssemblyFactoryCreate(ref err) => err.description(),
            Error::AssemblyFactoryGet(ref err) => err.description(),
            Error::JobMarkArchived(ref err) => err.description(),
            Error::JobPending(ref err) => err.description(),
            Error::JobReset(ref err) => err.description(),
            Error::JobSetLogUrl(ref err) => err.description(),
            Error::JobSetState(ref err) => err.description(),
            Error::LogDirDoesNotExist(_, ref err) => err.description(),
            Error::LogDirIsNotDir(_) => "Build log directory is not a directory",
            Error::LogDirNotWritable(_) => "Build log directory is not writable",
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
