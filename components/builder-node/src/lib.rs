// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate chrono;
extern crate ipnet;
extern crate oping;
extern crate rand;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_telemetry as telemetry;

extern crate postgres;
extern crate serde_json;

mod discover;
pub mod error;
pub mod models;

pub use self::error::{Error, Result};

//// The public types of outputs

pub type NodeOutputList = Result<Option<Vec<protocol::api::node::Node>>>;

pub type NodeOutput = Result<Option<protocol::api::node::Node>>;

//sensei type of outputs
pub type SenseiOutputList = Result<Option<Vec<protocol::api::senseis::Senseis>>>;

pub type SenseiOutput = Result<Option<protocol::api::senseis::Senseis>>;
