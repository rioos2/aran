// Copyright 2018 The Rio Advancement Inc

//! Contains core functionality for the Application's main server.
use std::sync::Arc;
use config::Config;
use error::Result;

use super::node::{Node, Servers};
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
    pub fn run(&mut self, ui: &mut UI, server: Servers) -> Result<()> {
        let cfg1 = self.config.clone();

        match server {
            Servers::APISERVER => {
                ui.begin(
        r#"
    ██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗     █████╗ ██████╗  █████╗ ███╗   ██╗     █████╗ ██████╗ ██╗
    ██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██╔══██╗██╔══██╗██╔══██╗████╗  ██║    ██╔══██╗██╔══██╗██║
    ██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ███████║██████╔╝███████║██╔██╗ ██║    ███████║██████╔╝██║
    ██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██╔══██║██╔══██╗██╔══██║██║╚██╗██║    ██╔══██║██╔═══╝ ██║
    ██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ██║  ██║██║  ██║██║  ██║██║ ╚████║    ██║  ██║██║     ██║
    ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝    ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝    ╚═╝  ╚═╝╚═╝     ╚═╝                                                                                                        
    "#,
    )?;
                ui.begin(&format!("Rio/OS API listening on {}:{}", self.config.https.listen, self.config.https.port))?;
            }
            Servers::STREAMER => {
                ui.begin(
                    r#"
                                                                                                                    
██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗    ███████╗████████╗██████╗ ███████╗ █████╗ ███╗   ███╗███████╗██████╗ 
██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔══██╗████╗ ████║██╔════╝██╔══██╗
██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ███████╗   ██║   ██████╔╝█████╗  ███████║██╔████╔██║█████╗  ██████╔╝
██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ╚════██║   ██║   ██╔══██╗██╔══╝  ██╔══██║██║╚██╔╝██║██╔══╝  ██╔══██╗
██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ███████║   ██║   ██║  ██║███████╗██║  ██║██║ ╚═╝ ██║███████╗██║  ██║
╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝    ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝
   "#,
                )?;
                ui.begin(&format!("Rio/OS STREAMER listening on {}:{}", self.config.http2.listener, self.config.http2.port))?;
            }
            Servers::UISTREAMER => {
                ui.begin(
        r#"
                                                                                                                                
██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗    ██╗   ██╗██╗███████╗████████╗██████╗ ███████╗ █████╗ ███╗   ███╗███████╗██████╗ 
██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██║   ██║██║██╔════╝╚══██╔══╝██╔══██╗██╔════╝██╔══██╗████╗ ████║██╔════╝██╔══██╗
██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ██║   ██║██║███████╗   ██║   ██████╔╝█████╗  ███████║██╔████╔██║█████╗  ██████╔╝
██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██║   ██║██║╚════██║   ██║   ██╔══██╗██╔══╝  ██╔══██║██║╚██╔╝██║██╔══╝  ██╔══██╗
██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ╚██████╔╝██║███████║   ██║   ██║  ██║███████╗██║  ██║██║ ╚═╝ ██║███████╗██║  ██║
╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝     ╚═════╝ ╚═╝╚══════╝   ╚═╝   ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝ 
"#,
    )?;

                ui.begin(&format!("Rio/OS UISTREAMER listening on {}:{}", self.config.http2.listener, self.config.http2.websocket))?;
            }
        }

        ui.heading("Ready to go.")?;

        let node = Node::new(cfg1);

        ui.para("Ready to serve.")?;

        node.run(ui, server)?;
        Ok(())
    }
}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(ui: &mut UI, config: Config, server: Servers) -> Result<()> {
    Server::new(config).run(ui, server)
}
