// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate rioos_builder_db as db;
extern crate rioos_builder_health_nodes as nodesrv;
extern crate rioos_http_client as rioos_http;
extern crate rioos_telemetry as telemetry;

#[macro_use]
extern crate log;
extern crate postgres;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
pub mod models;

use models::diagnostics::Services;

pub type StatusOutput = Option<Services>;
