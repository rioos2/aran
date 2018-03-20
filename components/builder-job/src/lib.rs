// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder node

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate serde_json;

pub mod job_ds;
pub mod error;

pub use self::error::{Error, Result};

pub type JobOutputList = Result<Option<Vec<protocol::api::job::Jobs>>>;

pub type JobOutput = Result<Option<protocol::api::job::Jobs>>;
