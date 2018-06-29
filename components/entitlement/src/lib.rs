// Copyright 2018 The Rio Advancement Inc

#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate libloading as lib_load;
extern crate rioos_core as rio_core;
extern crate handlebars;
#[macro_use]
extern crate serde_json;
extern crate rand;
extern crate rioos_http_client as http_client;
extern crate rioos_builder_db as db;
extern crate rioos_builder_entitlement as entitlement;
extern crate rioos_builder_apimachinery as protocol;

pub use self::error::{Error, Result};

pub mod error;
pub mod licensor;
pub mod nalperion;
pub mod licensecloud;
pub mod config;
pub mod softwarekey;
