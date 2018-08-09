// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder authorization
#[macro_use]
extern crate log;
extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate serde;
extern crate serde_json;

pub mod error;
pub mod models;

pub use self::error::{Error, Result};

/// Teams output
pub type TeamsOutput = Result<std::option::Option<protocol::api::authorize::Teams>>;
/// Teams output as list
pub type TeamsOutputList = Result<Option<Vec<protocol::api::authorize::Teams>>>;

/// permission output
pub type PermissionsOutput = Result<std::option::Option<protocol::api::authorize::Permissions>>;
/// permission output as list
pub type PermissionsOutputList = Result<Option<Vec<protocol::api::authorize::Permissions>>>;
