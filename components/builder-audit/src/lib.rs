// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_http_client as http_client;
extern crate influx_db_client;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod models;
pub mod error;
pub mod vulnerable;
pub mod config;

pub use self::error::Result;

//// The public types of outputs
pub type LogOutputList = Result<Option<Vec<protocol::api::log::LogOutput>>>;
