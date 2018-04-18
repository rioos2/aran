// Copyright 2018 The Rio Advancement Inc

//! Libraries  module used by builder scaling

extern crate chrono;
extern crate rioos_builder_apimachinery as protocol;
extern crate rioos_builder_db as db;
extern crate rioos_net as rio_net;
extern crate rioos_telemetry as telemetry;

extern crate postgres;
extern crate serde_json;

pub mod horizontalscaling_ds;
pub mod verticalscaling_ds;
pub mod error;
pub mod scaling;

pub use self::error::{Error, Result};

//// The public types of outputs

pub type HorizontalScalingOutputList = Result<Option<Vec<protocol::api::scale::HorizontalScaling>>>;

pub type HorizontalScalingOutput = Result<Option<protocol::api::scale::HorizontalScaling>>;

pub type VerticalScalingOutputList = Result<Option<Vec<protocol::api::scale::VerticalScaling>>>;

pub type VerticalScalingOutput = Result<Option<protocol::api::scale::VerticalScaling>>;
