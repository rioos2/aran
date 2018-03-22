// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder session

extern crate chrono;
extern crate ldap3;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate rand;
extern crate serde_json;
pub mod models;
pub mod origin_ds;
pub mod team_ds;
pub mod ldap;
pub mod error;
pub use self::error::{Error, Result};

//// The public types of outputs

pub type OriginOutputList = Result<Option<Vec<protocol::api::origin::Origin>>>;

pub type OriginOutput = Result<Option<protocol::api::origin::Origin>>;

pub type SamlOutputList = Result<Option<Vec<protocol::api::session::SamlProvider>>>;

pub type OpenIdOutputList = Result<Option<Vec<protocol::api::session::OidcProvider>>>;
