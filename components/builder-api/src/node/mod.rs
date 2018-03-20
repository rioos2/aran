// Copyright 2018 The Rio Advancement Inc
//

//! Node that serves the api, runtimeguard, streamer.
//!

#![allow(unused_must_use)]

pub mod events;
pub mod runtime;
pub mod api_wirer;
pub mod streamer;

use std::path::PathBuf;
use std::sync::Arc;
use error::Result;

use rio_core::fs::rioconfig_config_path;
use rio_core::crypto::keys::read_key_in_bytes;

use config::Config;
use common::ui::UI;

use node::streamer::TLSPair;

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
    pub fn run(self, ui: &mut UI, streamer: bool) -> Result<()> {
        ui.title("Node run");
        //start the runtime guard.
        ui.begin("Runtime Guard");
        let rg = runtime::Runtime::new(self.config.clone());
        let api_sender = rg.channel();
        &rg.start()?;
        ui.end("Runtime Guard");

        if streamer {
            //start the runtime guard.
            ui.begin("Streamer");
            streamer::Streamer::new(self.config.http.watch_port, self.config.clone())
                .start(self.tls_as_option(self.config.http.tls_pkcs12_file.clone()))?;
            ui.end("Streamer");
        } else {
            //start the runtime guard.
            ui.heading("Api Wirer");
            api_wirer::Wirer::new(self.config.clone()).start(
                ui,
                api_sender,
            )?;
            ui.end("Api Wirer");
        }

        Ok(())
    }

    /// Returns the a tuple for tls usage with
    /// Option<(tls file location, bytes loaded from the name in the config toml file,
    ///        tls password if present or empty string)>
    fn tls_as_option(&self, tls_file: Option<String>) -> TLSPair {
        tls_file.clone().and_then(|t| {
            read_key_in_bytes(&PathBuf::from(
                &*rioconfig_config_path(None).join(t.clone()),
            )).map(|p| {
                (
                    t.clone(),
                    p,
                    self.config.http.tls_pkcs12_pwd.clone().unwrap_or(
                        "".to_string(),
                    ),
                )
            })
                .ok()
        })
    }
}
