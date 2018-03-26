// Copyright 2018 The Rio Advancement Inc

#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate libloading as lib_load;
extern crate rioos_core as rio_core;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
extern crate failure;
extern crate rand;

pub use self::error::{Error, Result};

pub mod error;
pub mod nalperion;
pub mod config;
