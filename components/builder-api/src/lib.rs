// Copyright (c) 2017 RioCorp Inc.

//! Libraries  module used by builder api

extern crate bodyparser;
extern crate rioos_builder_protocol as protocol;
#[macro_use]
extern crate rioos_core as rio_core;
extern crate rioos_common as common;
extern crate rioos_net as rio_net;
extern crate rioos_builder_asmsrv as deploy;
extern crate rioos_builder_session as session;
extern crate rioos_builder_scalesrv as scale;
extern crate rioos_builder_authsrv as authorize;
extern crate rioos_builder_nodesrv as node;
extern crate rioos_builder_servicesrv as service;

extern crate rioos_builder_db as db;
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate log;
extern crate mount;
extern crate params;
extern crate persistent;
#[macro_use]
extern crate router;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate staticfile;
extern crate toml;
extern crate unicase;

pub mod config;
pub mod error;
pub mod http;
pub mod command;
pub mod server;

pub use self::config::Config;
pub use self::error::{Error, Result};
