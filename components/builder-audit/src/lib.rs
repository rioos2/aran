// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate influx_db_client;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_http_client as http_client;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod config;
pub mod error;
pub mod models;
pub mod vulnerable;

pub use self::error::Result;

//// The public types of outputs
pub type LogOutputList = Result<Option<Vec<protocol::api::log::LogOutput>>>;
