// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate rioos_net as rio_net;
extern crate rioos_builder_apimachinery as protocol;

extern crate influx_db_client;
extern crate serde_json;

pub mod models;
pub mod error;

pub use self::error::Result;

//// The public types of outputs
pub type LogOutputList = Result<Option<Vec<protocol::api::log::LogOutput>>>;
