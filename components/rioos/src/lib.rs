// Copyright (c) 2017 RioCorp Inc.
//

#![recursion_limit="128"]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rioos_core;
extern crate rioos_common as common;
extern crate rioos_api_client as api_client;
extern crate rioos_http_client as http_client;
extern crate rioos_builder_protocol as protocol;
extern crate handlebars;


extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate regex;
extern crate retry;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate walkdir;
extern crate base64;
extern crate human_size;
#[macro_use]
extern crate prettytable;

#[cfg(test)]
extern crate tempdir;
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
