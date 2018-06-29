// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder storage

extern crate chrono;
extern crate postgres;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate serde_json;

pub mod error;
pub mod storage_ds;

pub use self::error::{Error, Result};

//// The public types of outputs

/// StorageConnector output loaded from the database
pub type StorageConnectorOutput = Result<Option<protocol::api::storage::Storage>>;

/// StorageConnector output list loaded from the database
pub type StorageConnectorOutputList = Result<Option<Vec<protocol::api::storage::Storage>>>;

/// StoragePool output list loaded from the database
pub type StoragePoolOutput = Result<Option<protocol::api::storage::StoragePool>>;
/// StoragePool output list loaded from the database
pub type StoragePoolOutputList = Result<Option<Vec<protocol::api::storage::StoragePool>>>;

/// Datacenter output list loaded from the database
pub type DatacenterOutput = Result<Option<protocol::api::storage::DataCenter>>;
/// Datacenter output list loaded from the database
pub type DatacenterOutputList = Result<Option<Vec<protocol::api::storage::DataCenter>>>;
