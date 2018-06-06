// Copyright 2018 The Rio Advancement Inc
//
#[macro_use]
extern crate lazy_static;

extern crate crypto;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_authsrv as auth;
extern crate rioos_builder_db as db;
extern crate rioos_builder_servicesrv as secret;
extern crate rioos_builder_servicesrv as serviceaccount;
extern crate rioos_builder_session as session;
extern crate rioos_common as common;

#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate iron;
extern crate rand;
extern crate regex;
extern crate serde;

extern crate openssl;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

#[cfg(not(test))]
extern crate serde_json;

pub mod config;
pub mod error;
pub mod rbac;
pub mod rioos;
pub mod util;
