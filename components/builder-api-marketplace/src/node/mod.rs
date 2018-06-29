// Copyright 2018 The Rio Advancement Inc
//

//! Node that serves the api.
//!

pub mod api_wirer;

use std::sync::Arc;

use config::Config;
use common::ui::UI;
use error::Result;
use db::data_store::*;
use error::Error;

// Node that contains handler (`RuntimeHandler`)`.
#[derive(Debug)]
pub struct Node {
    config: Arc<Config>,
}

impl Node {
    // Creates node for the given api and node configuration.
    pub fn new(config: Arc<Config>) -> Self {
        Node {
            config: config.clone(),
        }
    }
    // A generic implementation that launches `Node` and optionally creates threads
    // for aran api handlers.
    // Aran api v1 prefix is `/api/v1`
    pub fn run(self, _ui: &mut UI) -> Result<()> {
        let ods = DataStoreConn::new().ok();
        let ds = match ods {
            Some(ds) => Box::new(ds),           
            None => {
                return Err(Error::Api("Failed to wire the api middleware, \ndatabase isn't ready.".to_string()))
            }
        };
        api_wirer::ApiSrv::new(self.config.clone()).start(ds.clone())
    }
}
