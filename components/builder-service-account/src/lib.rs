// Copyright (c) 2017 RioCorp Inc.

//! Libraries  module used by builder service account

extern crate chrono;
extern crate rioos_builder_protocol as protocol;
extern crate rioos_builder_db as db;
extern crate postgres;
extern crate rand;
extern crate serde;
extern crate serde_json;

pub mod service_account_ds;
pub mod error;

pub use self::error::{Error, Result};
