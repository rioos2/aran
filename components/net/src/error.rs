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

use protocol;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io;
use std::result;

use hyper;
use serde_json;

use auth;

#[derive(Debug)]
pub enum Error {
    Auth(auth::default::AuthErr),
    GitHubAPI(hyper::status::StatusCode, HashMap<String, String>),
    IO(io::Error),
    Json(serde_json::Error),
    MaxHops,
    HTTP(hyper::status::StatusCode),
    RequiredConfigField(&'static str),
    Sys,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Auth(ref e) => format!("GitHub Authentication error, {}", e),
            Error::GitHubAPI(ref c, ref m) => format!("[{}] {:?}", c, m),
            Error::HTTP(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::MaxHops => format!("Received a message containing too many network hops"),
            Error::RequiredConfigField(ref e) => format!("Missing required field in configuration, {}", e),
            Error::Sys => format!("Internal system error"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Auth(_) => "GitHub authorization error.",
            Error::GitHubAPI(_, _) => "GitHub API error.",
            Error::IO(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::Json(ref err) => err.description(),
            Error::MaxHops => "Received a message containing too many network hops",
            Error::RequiredConfigField(_) => "Missing required field in configuration.",
            Error::Sys => "Internal system error",
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<auth::default::AuthErr> for Error {
    fn from(err: auth::default::AuthErr) -> Self {
        Error::Auth(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
