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

use std::path::PathBuf;
use std::sync::Arc;
use error::Result;

use rio_core::fs::rioconfig_config_path;
use rio_core::crypto::keys::read_key_in_bytes;

use config::Config;
use common::ui::UI;
use node::streamer::TLSPair;

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
        Node {
            config: config.clone(),
        }
    }

    // A generic implementation that launches a `Node`
    // for aran api handlers.
    pub fn run(self, ui: &mut UI, server: Servers) -> Result<()> {
        ui.title("Node run");
        //start the runtime guard.
        ui.begin("Runtime Guard");
        let rg = runtime::Runtime::new(self.config.clone());
        let api_sender = rg.channel();

        ui.end("Runtime Guard");

        match server {
            Servers::APISERVER => {
                ui.heading("Api Wirer");
                api_wirer::Wirer::new(self.config.clone()).start(ui, api_sender, rg)?;
                ui.end("Api Wirer");
            }
            Servers::STREAMER => {
                ui.begin("Streamer");
                streamer::Streamer::new(self.config.http2.port, self.config.clone()).start(self.read_tls_as_bytes(
                    self.config.http2.tls.clone(),
                    self.config.http2.tls_password.clone(),
                ))?;
                ui.end("Streamer");
            }
            Servers::UISTREAMER => {
                //start the uiwatcher(websocket) server.
                ui.begin("UIStreamer");
                websocket::Websocket::new(self.config.http2.websocket, self.config.clone()).start(self.read_tls_as_bytes(
                    self.config.http2.tls.clone(),
                    self.config.http2.tls_password.clone(),
                ))?;
                ui.end("UIStreamer");
            }
        }

        Ok(())
    }

    /// Returns the a tuple for tls usage with
    /// Option<(tls file location, bytes loaded from the name in the config toml file,
    ///        tls password if present or empty string)>
    fn read_tls_as_bytes(&self, tls: Option<String>, tls_password: Option<String>) -> TLSPair {
        tls.clone().and_then(|t| {
            read_key_in_bytes(&PathBuf::from(&*rioconfig_config_path(None).join(t.clone())))
                .map(|p| (t.clone(), p, tls_password.clone().unwrap_or("".to_string())))
                .ok()
        })
    }
}
