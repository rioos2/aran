// Copyright 2018 The Rio Advancement Inc

//! Contains core functionality for the Application's main server.

use std::sync::Arc;
use rio_net::server::NetIdent;
use config::Config;
use error::Result;
/* mod node;  don't remove this line, for channel/watch */
use super::node::Node;
use common::ui::UI;

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
    pub fn run(&mut self, ui: &mut UI) -> Result<()> {
        let cfg1 = self.config.clone();

        ui.begin(&format!(
            "Rio/OS Marketplace listening on {}:{}",
            self.config.http.listen,
            self.config.http.port
        ))?;
        ui.heading("Ready to go.")?;

        let node = Node::new(cfg1);

        ui.para("Ready to serve.")?;

        node.run(ui)?;
        Ok(())
    }
}

impl NetIdent for Server {}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(ui: &mut UI, config: Config) -> Result<()> {
    Server::new(config).run(ui)
}
