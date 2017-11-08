// Copyright (c) 2017 RiioCorp Inc
//

extern crate chrono;
extern crate ansi_term;
extern crate fnv;
extern crate rioos_common as common;
extern crate rioos_builder_protocol as protocol;
extern crate rioos_core as core;
extern crate rioos_builder_db as db;
extern crate rioos_builder_session as session;
extern crate rioos_builder_authsrv as authorize;
extern crate crypto;
extern crate itertools;

#[macro_use]
extern crate hyper;
extern crate hyper_openssl;
extern crate iron;
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
extern crate curl;

extern crate router;

pub mod config;
pub mod error;
pub mod http;
pub mod auth;
pub mod metrics;
pub mod util;
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
