// Copyright 2018 The Rio Advancement Inc
//

extern crate ansi_term;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate rpassword;
extern crate serde;
extern crate serde_json;
#[cfg(test)]
extern crate tempdir;
extern crate term;
extern crate toml;
extern crate uuid;

pub use self::error::{Error, Result};

pub mod error;
pub mod ui;
