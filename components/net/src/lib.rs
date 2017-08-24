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

extern crate fnv;
extern crate rioos_builder_protocol as protocol;
extern crate rioos_core as core;
extern crate rioos_builder_db as db;
extern crate rioos_builder_session as session;
extern crate crypto;
#[macro_use]
extern crate hyper;
extern crate hyper_openssl;
extern crate iron;
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate unicase;
extern crate rand;


pub mod config;
pub mod error;
pub mod http;
pub mod auth;
pub mod server;
use std::process::Command;

pub use self::error::{Error, Result};
pub use self::server::Application;

pub fn hostname() -> Result<String> {
    let output = try!(
        Command::new("sh")
            .arg("-c")
            .arg("hostname | awk '{printf \"%s\", $NF; exit}'")
            .output()
    );
    match output.status.success() {
        true => {
            debug!(
                "Hostname address is {}",
                String::from_utf8_lossy(&output.stdout)
            );
            let hostname = try!(String::from_utf8(output.stdout).or(Err(Error::Sys)));
            Ok(hostname)
        }
        false => {
            debug!(
                "Hostname address command returned: OUT: {} ERR: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            Err(Error::Sys)
        }
    }
}
