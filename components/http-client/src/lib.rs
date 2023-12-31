// Copyright 2018 The Rio Advancement Inc
//

extern crate base64;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate rioos_core as rio_core;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod api_client;
pub mod error;
pub mod proxy;

pub use api_client::ApiClient;
pub use error::{Error, Result};
