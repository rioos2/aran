// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder service account

extern crate chrono;
extern crate postgres;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate serde_json;

pub mod error;
pub mod models;

pub use self::error::{Error, Result};

//// The public types of outputs

pub type SecretOutputList = Result<Option<Vec<protocol::api::secret::Secret>>>;

pub type SecretOutput = Result<Option<protocol::api::secret::Secret>>;

pub type ServiceAccountOutputList =
    Result<Option<Vec<protocol::api::service_account::ServiceAccount>>>;

pub type ServiceAccountOutput = Result<Option<protocol::api::service_account::ServiceAccount>>;

pub type SettingsMapOutput = Result<Option<protocol::api::settings_map::SettingsMap>>;
