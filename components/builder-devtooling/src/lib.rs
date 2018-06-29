// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder network

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;

extern crate postgres;
extern crate serde_json;

pub mod error;
pub mod models;

pub use self::error::{Error, Result};

/// Build Config output
pub type BuildConfigOutput = Result<std::option::Option<protocol::api::devtool::BuildConfig>>;

/// build config output as list
pub type BuildConfigOutputList = Result<Option<Vec<protocol::api::devtool::BuildConfig>>>;

/// Build output
pub type BuildOutput = Result<std::option::Option<protocol::api::devtool::Build>>;

/// build output as list
pub type BuildOutputList = Result<Option<Vec<protocol::api::devtool::Build>>>;

/// ImageReferences output
pub type ImageReferencesOutput =
    Result<std::option::Option<protocol::api::devtool::ImageReferences>>;

/// image reference output as list
pub type ImageReferencesOutputList = Result<Option<Vec<protocol::api::devtool::ImageReferences>>>;

/// ImageMarks output
pub type ImageMarksOutput = Result<std::option::Option<protocol::api::devtool::ImageMarks>>;

/// image marks output as list
pub type ImageMarksOutputList = Result<Option<Vec<protocol::api::devtool::ImageMarks>>>;
