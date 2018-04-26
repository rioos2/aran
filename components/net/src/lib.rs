// Copyright 2018 The Rio Advancement Inc
//

extern crate ansi_term;
extern crate rioos_builder_servicesrv as serviceaccount;
extern crate rioos_builder_servicesrv as secret;
extern crate rioos_entitlement as entitlement;

extern crate chrono;
extern crate crypto;
extern crate persistent;
#[macro_use]
extern crate lazy_static;
extern crate bodyparser;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_authsrv as authorize;
extern crate rioos_builder_db as db;
extern crate rioos_builder_session as session;
extern crate rioos_common as common;
extern crate rioos_core as core;
extern crate rioos_auth as auth;

extern crate handlebars;

extern crate brotli;
extern crate iron;
extern crate libflate;
#[macro_use]
extern crate log;
extern crate mount;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate unicase;
extern crate base64;
extern crate url;

extern crate router;

pub mod error;
use std::process::Command;

pub use self::error::{Error, Result};
