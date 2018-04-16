// Copyright 2018 The Rio Advancement Inc

//! Contains core functionality for the Application's main server.
use std::sync::Arc;
use rio_net::server::NetIdent;
use config::Config;
use error::Result;
/* mod node;  don't remove this line, for channel/watch */
#[cfg(feature = "ssl")]
use super::node::{Node, Servers};
use common::ui::UI;


/// The main server for the Builder-API application. This should be run on the main thread.
#[cfg(feature = "ssl")]
pub struct Server {
    pub config: Arc<Config>,
}

#[cfg(feature = "ssl")]
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
    pub fn run(&mut self, ui: &mut UI, server: Servers) -> Result<()> {
        let cfg1 = self.config.clone();

        match server {
            Servers::APISERVER => {
                ui.begin(&format!(
                    "Rio/OS API listening on {}:{}",
                        self.config.http.listen,
                        self.config.http.port
                ))?;
            }
            Servers::STREAMER => { 
                ui.begin(&format!(
                    "Rio/OS Watch server listening on {}:{}",
                        self.config.http.listen,
                        self.config.http.watch_port
                ))?;
            }
            Servers::WEBSOCKET => { 
                ui.begin(&format!(
                    "Rio/OS Websocket server listening on {}:{}",
                        self.config.http.listen,
                        self.config.http.websocket_port
                ))?;
            }
        }
        
        ui.heading("Ready to go.")?;

        let node = Node::new(cfg1);

        ui.para("Ready to serve.")?;

        node.run(ui, server)?;
        Ok(())
    }
}

#[cfg(feature = "ssl")]
impl NetIdent for Server {}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
#[cfg(feature = "ssl")]
pub fn run(ui: &mut UI, config: Config, server: Servers) -> Result<()> {
    Server::new(config).run(ui, server)
}
