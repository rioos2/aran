// Copyright (c) 2017 RioCorp Inc.

//! Libraries  module used by builder deployment

extern crate petgraph;
extern crate chrono;
extern crate rioos_builder_protocol as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_builder_servicesrv as service;


extern crate postgres;
extern crate rand;
extern crate serde;
extern crate serde_json;

pub mod deployment_ds;
pub mod error;
pub mod replicas;


pub use self::error::{Error, Result};
