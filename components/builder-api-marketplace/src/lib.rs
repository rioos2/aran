// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder api
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;


#[macro_use]
extern crate lazy_static;

extern crate ansi_term;
extern crate bodyparser;
extern crate regex;
extern crate rand;

extern crate rioos_builder_httpgateway as http_gateway;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_marketplacesrv as marketplace;
extern crate rioos_builder_session as session;
extern crate rioos_common as common;
extern crate rioos_core as rio_core;
extern crate rioos_auth as auth;
extern crate rioos_builder_db as db;

extern crate iron;
extern crate mount;
extern crate persistent;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate url;

pub mod config;
pub mod error;
pub mod api;
pub mod server;
pub mod node;
pub mod command;

pub use self::config::Config;
pub use self::error::{Error, Result};

pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
