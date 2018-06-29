// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder marketplace

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate rand;
extern crate serde_json;

pub mod error;
pub mod marketplace_ds;
pub mod package_attacher;
pub mod package_ds;

pub use self::error::{Error, Result};

/// MarketPlaceOutput
pub type MarketPlaceOutput = Result<std::option::Option<protocol::api::marketplace::MarketPlace>>;

/// MarketPlaceOutput output as list
pub type MarketPlaceOutputList =
    Result<std::option::Option<Vec<protocol::api::marketplace::MarketPlace>>>;

/// PackageOutputList
pub type PackageOutputList = Result<Option<Vec<protocol::api::package::Package>>>;

/// PackageOutput
pub type PackageOutput = Result<Option<protocol::api::package::Package>>;
