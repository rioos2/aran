// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder api
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate hyper;

extern crate base64;

extern crate chrono;
extern crate handlebars;

extern crate ansi_term;
extern crate bodyparser;
extern crate rand;
extern crate regex;
extern crate tempdir;
extern crate urlencoded;

extern crate rioos_auth as auth;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_audit as audit;
extern crate rioos_builder_authorizer as authorize;
extern crate rioos_builder_db as db;
extern crate rioos_builder_deployment as deploy;
extern crate rioos_builder_devtooling as devtooling;
extern crate rioos_builder_diagnostics as rio_diago;
extern crate rioos_builder_health_nodes as clusters;
extern crate rioos_builder_httpgateway as http_gateway;
extern crate rioos_builder_jobsbuilder as job;
extern crate rioos_builder_scalers as scale;
extern crate rioos_builder_services as service;
extern crate rioos_builder_session as session;
extern crate rioos_builder_storages as storage;
extern crate rioos_builder_virtual_network as network;
extern crate rioos_common as common;
extern crate rioos_core as rio_core;
extern crate rioos_entitlement as entitlement;
extern crate rioos_http2 as httpbis;
extern crate rioos_telemetry as telemetry;
extern crate rioos_ws as ws;

extern crate iron;
extern crate mount;
extern crate params;
extern crate persistent;
extern crate postgres;
extern crate reqwest;
extern crate rioos_http_client as rioos_http;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate mio;
extern crate serde_yaml;

extern crate typemap;
extern crate url;

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_timer;

extern crate fallible_iterator;

extern crate openssl;
extern crate tls_api;
extern crate tls_api_openssl;

extern crate schedule_recv;

#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

extern crate lettre;
extern crate lettre_email;
extern crate openio_sdk_rust;

pub mod config;
pub mod error;
#[macro_use]
pub mod api;

pub mod command;
pub mod events;
pub mod hooks;
pub mod node;
pub mod server;
pub mod validator;
pub mod watch;

pub use self::config::Config;
pub use self::error::{Error, Result};

extern crate bytes;

pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
