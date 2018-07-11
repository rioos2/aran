// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder deployment

extern crate chrono;
extern crate human_size;
extern crate petgraph;
extern crate postgres;
extern crate rand;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_builder_jobsbuilder as job;
extern crate rioos_telemetry as telemetry;
extern crate serde_json;

pub mod assembler;
mod builder;
pub mod error;
pub mod models;
pub mod replicas_expander;
pub mod stacks;

pub use self::error::{Error, Result};

//The plan category that applies to services.
const APPLICABLE_TO: &'static [&'static str] = &["blockchain_template", "containers"];

///The plan category that is eligible to be stand still
///The plan categories eligible to be standstill are
/// Blockchain networks - denoted by "blockchain"
/// Blockcahin apps     - denoted by "blockchain_template"
const APPLICABLE_TO_STAND_STILL: &'static [&'static str] = &["blockchain_template", "blockchain"];

// AssemblyFactory output
pub type AssemblyFactoryOutput = Result<std::option::Option<protocol::api::deploy::AssemblyFactory>>;

/// AssemblyFactory output as list
pub type AssemblyFactoryOutputList = Result<std::option::Option<Vec<protocol::api::deploy::AssemblyFactory>>>;

// StacksFactory output
pub type StacksFactoryOutput = Result<std::option::Option<protocol::api::deploy::StacksFactory>>;

/// StacksFactory output as list
pub type StacksFactoryOutputList = Result<std::option::Option<Vec<protocol::api::deploy::StacksFactory>>>;

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


/// ingress output
pub type IngressOutput = Result<std::option::Option<protocol::api::ingress::Ingress>>;
