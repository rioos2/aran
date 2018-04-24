// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_http_client as rioos_http;
extern crate rioos_telemetry as telemetry;
extern crate rioos_net as rio_net;

extern crate postgres;
#[macro_use]
extern crate serde_json;
extern crate iron;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod diagnostics_ds;

pub type StatusOutput = Option<diagnostics_ds::MasterSystemStatus>;

