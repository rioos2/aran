// Copyright 2018 The Rio Advancement Inc

//! Contains core functionality for the Application's main server.
use std::path::PathBuf;
use std::fs::File;

use std::sync::Arc;
use rio_net::server::NetIdent;
use config::Config;
use error::{Result, Error};
/* mod node;  don't remove this line, for channel/watch */
use super::node::Node;
use rio_core::crypto::default_rioconfig_key_path;
use common::ui::UI;

lazy_static! {
    static  ref SETUP_COMPLETE_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join(".rioos_setup_complete").to_str().unwrap());
    static  ref MARKETPLACE_CACHE_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join("pullcache/marketplaces.yaml").to_str().unwrap());
}

/// The main server for the Builder-API application. This should be run on the main thread.
pub struct Server {
    pub config: Arc<Config>,
}

impl Server {
    /// Create a new `Server`
    pub fn new(config: Config) -> Self {
        Server { config: Arc::new(config) }
    }

    /// Runs the main server and starts and manages all supporting threads. This function will
    /// block the calling thread.
    ///
    /// # Errors
    ///
    /// * HTTPS server could not start
    pub fn run(&mut self, ui: &mut UI, streamer: bool) -> Result<()> {
        let cfg1 = self.config.clone();

        if File::open(&SETUP_COMPLETE_FILE.as_path()).is_err() {
            return Err(Error::SetupNotDone);
        }

        if File::open(&MARKETPLACE_CACHE_FILE.as_path()).is_err() {
            return Err(Error::SyncNotDone);
        }

        ui.begin(&format!(
            "Rio/OS API listening on {}:{}",
            self.config.http.listen,
            self.config.http.port
        ))?;
        ui.heading("Ready to go.")?;

        let node = Node::new(cfg1);

        ui.para("Ready to serve.")?;

        node.run(ui, streamer)?;
        Ok(())
    }
}

impl NetIdent for Server {}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(ui: &mut UI, config: Config, streamer: bool) -> Result<()> {
    Server::new(config).run(ui, streamer)
}
