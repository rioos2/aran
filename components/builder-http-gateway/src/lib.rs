// Copyright 2018 The Rio Advancement Inc
//

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate base64;
extern crate bodyparser;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_authsrv as authorize;
extern crate rioos_builder_db as db;
extern crate rioos_builder_session as session;
extern crate rioos_common as common;
extern crate rioos_core as core;
extern crate rioos_auth as auth;
#[macro_use]
extern crate hyper;
extern crate brotli;
extern crate iron;
extern crate libflate;
#[macro_use]
extern crate log;
extern crate mount;
extern crate rand;
extern crate reqwest;
extern crate mount;
extern crate num_cpus;
extern crate params;
extern crate persistent;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate staticfile;
extern crate toml;
extern crate unicase;
extern crate urlencoded;

pub mod app;
pub mod config;
pub mod http;
pub mod util;

pub use app::start;
