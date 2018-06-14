// Copyright 2018 The Rio Advancement Inc
//

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate bodyparser;
extern crate brotli;
extern crate chrono;
extern crate crypto;
extern crate iron;
extern crate libflate;
extern crate regex;
extern crate rioos_auth as auth;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_authorizer as authorize;
extern crate rioos_builder_db as db;
extern crate rioos_common as common;
extern crate url;
#[macro_use]
extern crate log;
extern crate mount;
extern crate num_cpus;
extern crate persistent;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate hyper_native_tls;
extern crate serde_json;
extern crate unicase;

pub mod app;
pub mod config;
pub mod http;
pub mod util;

pub use app::start;

pub mod error;

pub use self::error::{Error, Result};
