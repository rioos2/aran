// Copyright (c) 2017 RioCorp Inc.

//! Libraries  module used by builder authorization

extern crate chrono;
extern crate rioos_builder_protocol as protocol;
extern crate rioos_builder_db as db;

#[macro_use]
extern crate postgres;
extern crate rand;
extern crate serde;
extern crate serde_json;


pub mod authorize_ds;
pub mod error;


pub use self::error::{Error, Result};
