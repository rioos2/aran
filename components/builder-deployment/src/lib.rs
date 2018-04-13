// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder deployment

extern crate chrono;
extern crate petgraph;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_builder_jobsrv as job;
extern crate rioos_net as rio_net;
extern crate postgres;
extern crate serde_json;

pub mod error;
pub mod models;
mod builder;
pub mod assembler;
pub mod replicas_expander;

pub use self::error::{Error, Result};

//The plan category that applies to services.
const APPLICABLE_TO: &'static [&'static str] = &["blockchain_template", "containers"];

//The plan category that is eligible to be stand still
const APPLICABLE_TO_STAND_STILL: &'static [&'static str] = &["blockchain_template"];

// AssemblyFactory output
pub type AssemblyFactoryOutput = Result<std::option::Option<protocol::api::deploy::AssemblyFactory>>;

/// AssemblyFactory output as list
pub type AssemblyFactoryOutputList = Result<std::option::Option<Vec<protocol::api::deploy::AssemblyFactory>>>;

/// Assembly output
pub type AssemblyOutput = Result<std::option::Option<protocol::api::deploy::Assembly>>;

/// Assembly output as list
pub type AssemblyOutputList = Result<std::option::Option<Vec<protocol::api::deploy::Assembly>>>;

/// Service output
pub type ServiceOutput = Result<std::option::Option<protocol::api::linker::Services>>;

/// Service output as list
pub type ServiceOutputList = Result<std::option::Option<Vec<protocol::api::linker::Services>>>;

/// PlanFactory output
pub type PlanOutput = Result<std::option::Option<protocol::api::blueprint::Plan>>;

/// PlanFactory output as list
pub type PlanOutputList = Result<std::option::Option<Vec<protocol::api::blueprint::Plan>>>;

pub type EndPointOutputList = Result<Option<Vec<protocol::api::endpoints::EndPoints>>>;

pub type EndPointOutput = Result<Option<protocol::api::endpoints::EndPoints>>;

/// volume output
pub type VolumeOutput = Result<std::option::Option<protocol::api::volume::Volumes>>;

/// volume output as list
pub type VolumeOutputList = Result<std::option::Option<Vec<protocol::api::volume::Volumes>>>;
