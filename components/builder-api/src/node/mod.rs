// Copyright 2018 The Rio Advancement Inc
//

//! Node that serves the api, runtimeguard, streamer.
//!

#![allow(unused_must_use)]

pub mod api_wirer;
pub mod events;
pub mod internal;
pub mod runtime;
pub mod streamer;
pub mod websocket;


use auth::rbac::license;

use common::ui::UI;
use config::Config;
use db::data_store::DataStoreConn;
use entitlement::softwarekeys::licensor::NativeSDK;
use error::{Error, Result};
use lib_load;
use protocol::cache::ExpanderSender;
use rio_core::fs::rioconfig_license_path;
use std::sync::Arc;
use watch::config::Streamer;

pub enum Servers {
    APISERVER,
    // The http2 port used by controlmanager, scheduler
    STREAMER,
    // The websocket port used by UI
    UISTREAMER,
}

#[derive(Debug)]
pub struct Node {
    config: Arc<Config>,
}

impl Node {
    // Creates node for the given api and node configuration.
    pub fn new(config: Arc<Config>) -> Self {
        Node { config: config.clone() }
    }

    // A generic implementation that launches a `Node`
    // for aran api handlers.
    pub fn run(self, ui: &mut UI) -> Result<()> {
        ui.title("Starting node");

        ui.begin("→ Runtime Guard");

        let ods = DataStoreConn::new().ok();
        let ds = match ods {
            Some(ds) => Box::new(ds),
            None => {
                return Err(Error::Api(
                    "Failed to wire the api middleware, \ndatabase isn't ready."
                        .to_string(),
                ))
            }
        };

        let rg = runtime::Runtime::new(self.config.clone(), self.create_licensor(ds.clone())?);

        let api_sender = rg.channel();

        ui.end("✓ Runtime Guard");

        ui.begin("→ Api Gateway");
        &rg.start()?;

        api_wirer::ApiSrv::new(self.config.clone()).start(
            api_sender,
            ds.clone(),
        )?;
        ui.end("✓ Api Gateway");

        ui.begin("→ Streamer");
        streamer::Streamer::new(self.config.http2.port, self.config.clone())
            .start((*self.config).http2_tls_pair(), ds.clone())?;
        ui.end("✓ Streamer");

        ui.begin("→ UIStreamer");
        websocket::Websocket::new(self.config.http2.websocket, self.config.clone())
            .start((*self.config).http2_tls_pair(), ds.clone())?;
        ui.end("✓ UIStreamer");

        Ok(())
    }
    /*This function creates the native API context with software key.
      Needs a cache with access to database
      The Native.so file is loaded and provided as input */
    fn create_licensor(&self, ds: Box<DataStoreConn>) -> Result<NativeSDK> {
        let mut license = license::LicensesFascade::new(ds.clone());
        license.with_cache();
        let so_file = self.config.licenses.so_file.clone();

        let lib = lib_load::Library::new(&rioconfig_license_path(None).join(so_file))?;

        let mut sdk = NativeSDK::new(lib, license);
        sdk.initialize_api_context()?;
        Ok(sdk)

    }
}
