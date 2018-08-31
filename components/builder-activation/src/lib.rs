// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder activation

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate serde_json;
pub mod error;
pub mod models;
pub use self::error::{Error, Result};
