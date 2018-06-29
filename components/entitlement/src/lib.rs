// Copyright 2018 The Rio Advancement Inc

#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate handlebars;
extern crate libloading as lib_load;
extern crate rioos_core as rio_core;
#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate rioos_http_client as http_client;

pub use self::error::{Error, Result};

pub mod config;
pub mod error;
pub mod licensecloud;
pub mod licensor;
pub mod nalperion;
