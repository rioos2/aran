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
extern crate rioos_builder_db as db;
extern crate rioos_builder_entitlement as entitlement;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_auth as auth;

pub use self::error::{Error, Result};

pub mod config;
pub mod error;
pub mod softwarekeys;
