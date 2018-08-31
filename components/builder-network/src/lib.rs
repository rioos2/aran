// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder network

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate serde_json;

pub mod error;
pub mod network_ds;

pub use self::error::{Error, Result};

/// Network output
pub type NetworkOutput = Result<std::option::Option<protocol::api::network::Network>>;

/// Network output as list
pub type NetworkOutputList = Result<Option<Vec<protocol::api::network::Network>>>;
