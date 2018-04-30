// Copyright 2018 The Rio Advancement Inc

//! Contains core functionality for the Application's main server.

use std::sync::Arc;

use config::Config;
use error::Result;
/* mod node;  don't remove this line, for channel/watch */

use super::node::Node;
use common::ui::UI;
use config::ConfigValidator;

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
            "Rio.Marketplaces listening on {}:{}",
            self.config.https.listen,
            self.config.https.port
        ))?;
        ui.heading("Ready to go.")?;

        let node = Node::new(cfg1);

        ui.para("Ready to serve.")?;

        node.run(ui)?;
        Ok(())
    }
}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(ui: &mut UI, config: Config) -> Result<()> {
    config.valid()?;

    config.dump(ui)?;

    Server::new(config).run(ui)
}
