// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder authorization
#[macro_use]
extern crate log;
extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_builder_session as session;

extern crate postgres;
extern crate serde;
extern crate serde_json;

pub mod error;
pub mod models;
pub mod invites;

pub use self::error::{Error, Result};

/// Teams output
pub type TeamsOutput = Result<std::option::Option<protocol::api::authorize::Teams>>;
/// Teams output as list
pub type TeamsOutputList = Result<Option<Vec<protocol::api::authorize::Teams>>>;

pub type PolicyOutputList = Result<Option<Vec<protocol::api::authorize::Policies>>>;

/// permission output
pub type PermissionsOutput = Result<std::option::Option<protocol::api::authorize::Permissions>>;
/// permission output as list
pub type PermissionsOutputList = Result<Option<Vec<protocol::api::authorize::Permissions>>>;

/// Invitations output
pub type InvitationsOutput = Result<std::option::Option<protocol::api::invitations::Invitations>>;
/// Invitations output as list
pub type InvitationsOutputList = Result<Option<Vec<protocol::api::invitations::Invitations>>>;

/// TeamMembers output
pub type TeamMembersOutput = Result<std::option::Option<protocol::api::authorize::TeamMembers>>;
/// TeamMembers output as list
pub type TeamMembersOutputList = Result<Option<Vec<protocol::api::authorize::TeamMembers>>>;

/// PolicyMembers output
pub type PolicyMembersOutput = Result<std::option::Option<protocol::api::authorize::PolicyMembers>>;
/// PolicyMembers output as list
pub type PolicyMembersOutputList = Result<Option<Vec<protocol::api::authorize::PolicyMembers>>>;
