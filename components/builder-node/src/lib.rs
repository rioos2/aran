// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_telemetry as telemetry;
extern crate itertools;
extern crate oping;
extern crate cidr;
extern crate rand;
extern crate ipnet;

extern crate postgres;
extern crate serde_json;

pub mod node_ds;
pub mod error;
pub mod models;

pub use self::error::{Error, Result};

//// The public types of outputs

pub type NodeOutputList = Result<Option<Vec<protocol::api::node::Node>>>;

pub type NodeOutput = Result<Option<protocol::api::node::Node>>;
