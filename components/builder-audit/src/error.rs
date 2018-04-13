// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder scaling
use std::result;
use influx_db_client;

pub type Result<T> = result::Result<T, influx_db_client::error::Error>;
