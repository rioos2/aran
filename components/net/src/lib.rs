// Copyright 2018 The Rio Advancement Inc
//

extern crate ansi_term;
extern crate rioos_builder_servicesrv as serviceaccount;
extern crate rioos_builder_servicesrv as secret;
extern crate rioos_entitlement as entitlement;

extern crate chrono;
extern crate crypto;
extern crate persistent;
#[macro_use]
extern crate lazy_static;
extern crate bodyparser;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_authsrv as authorize;
extern crate rioos_builder_db as db;
extern crate rioos_builder_session as session;
extern crate rioos_common as common;
extern crate rioos_core as core;
extern crate rioos_auth as auth;

#[macro_use]
extern crate horrorshow;

extern crate brotli;
extern crate iron;
extern crate libflate;
#[macro_use]
extern crate log;
extern crate mount;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate unicase;
extern crate base64;
extern crate url;

extern crate router;

pub mod config;
pub mod error;
pub mod http;
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
