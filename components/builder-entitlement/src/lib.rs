// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder authorization
#[macro_use]
extern crate log;
extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate serde;
extern crate serde_json;

pub mod error;
pub mod models;

pub use self::error::{Error, Result};

/// Roles output
pub type LicenseOutput = Result<std::option::Option<protocol::api::licenses::Licenses>>;

