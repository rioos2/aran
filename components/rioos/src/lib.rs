// Copyright 2018 The Rio Advancement Inc
//

#![recursion_limit = "128"]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate handlebars;
extern crate rioos_api_client as api_client;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_common as common;
extern crate rioos_core;
extern crate rioos_net as rio_net;

#[macro_use]
extern crate clap;
extern crate human_size;
#[macro_use]
extern crate log;
#[macro_use]
extern crate prettytable;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate url;
extern crate serde_yaml;

#[macro_use]
extern crate lazy_static;

pub mod cli;
pub mod command;
pub mod config;
pub mod error;

pub const PRODUCT: &'static str = "rioos";
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const AUTH_TOKEN_ENVVAR: &'static str = "RIO_AUTH_TOKEN";
pub const AUTH_EMAIL_ENVVAR: &'static str = "RIO_AUTH_EMAIL";
pub const ORIGIN_ENVVAR: &'static str = "RIO_ORIGIN";
pub const API_SERVER_ENVVAR: &'static str = "RIO_API_SERVER";
