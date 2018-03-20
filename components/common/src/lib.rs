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

pub use self::error::{Error, Result};

pub mod error;
pub mod ui;
