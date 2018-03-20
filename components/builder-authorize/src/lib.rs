// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder authorization

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate serde;
extern crate serde_json;

pub mod models;
pub mod error;

pub use self::error::{Error, Result};
