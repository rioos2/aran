// Copyright 2018 The Rio Advancement Inc
//

extern crate ansi_term;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate rioos_core as rcore;
#[cfg(test)]
extern crate tempdir;
extern crate term;
extern crate toml;
extern crate hyper;
extern crate regex;
extern crate uuid;
extern crate serde;
extern crate serde_json;
extern crate rpassword;

pub use self::error::{Error, Result};

pub mod error;
pub mod ui;
