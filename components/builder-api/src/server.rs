// Copyright 2018 The Rio Advancement Inc

//! Contains core functionality for the Application's main server.
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use config::Config;
use error::{Error, Result};

use super::node::Node;
use common::ui::UI;
use rio_core::crypto::default_rioconfig_key_path;
use validator::ConfigValidator;

lazy_static! {
    static ref SETUP_COMPLETE_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None)
        .join(".rioos_setup_complete")
        .to_str()
        .unwrap());
    static ref MARKETPLACE_CACHE_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None)
        .join("pullcache/appstores.yaml")
        .to_str()
        .unwrap());
}

/// The main server for the Builder-API application. This should be run on the main thread.
pub struct Server {
    pub config: Arc<Config>,
}

impl Server {
    /// Create a new `Server`
    pub fn new(config: Config) -> Self {
        Server {
            config: Arc::new(config),
        }
    }

    /// Runs the main server and starts and manages all supporting threads. This function will
    /// block the calling thread.
    ///
    /// # Errors
    ///
    /// * HTTPS server could not start
    pub fn run(&mut self, ui: &mut UI) -> Result<()> {
        let cfg1 = self.config.clone();

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
        ui.para(&format!(
            "Rio/OS {} listening on {}:{}",
            "API", self.config.https.listen, self.config.https.port
        ))?;

        ui.para(&format!(
            "Rio/OS {} listening on {}:{}",
            "Watch", self.config.http2.listener, self.config.http2.port
        ))?;

        ui.para(&format!(
            "Rio/OS {} listening on {}:{}",
            "Websocket", self.config.http2.listener, self.config.http2.websocket
        ))?;

        let node = Node::new(cfg1);

        node.run(ui)?;
        Ok(())
    }
}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(ui: &mut UI, config: Config) -> Result<()> {
    config.valid()?;

    config.dump(ui)?;

    if File::open(&SETUP_COMPLETE_FILE.as_path()).is_err() {
        return Err(Error::SetupNotDone);
    }

    if File::open(&MARKETPLACE_CACHE_FILE.as_path()).is_err() {
        return Err(Error::SyncNotDone);
    }

    Server::new(config).run(ui)
}
