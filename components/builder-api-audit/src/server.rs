// Copyright 2018 The Rio Advancement Inc

//! Contains core functionality for the Application's main server.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use std::path::PathBuf;

use exonum::node::{Node, NodeConfig};
use exonum::storage::{RocksDB, RocksDBOptions};
use rio_core::fs::rioconfig_blockchain_path;

use api::audit::Habitat;

use config::Config;
use error::Result;

use common::ui::UI;

lazy_static! {
    static ref ROCKSDB_DATA_PATH: PathBuf = PathBuf::from(&*rioconfig_blockchain_path(None));
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
    /// * HTTPS blockchain server could not start
    pub fn run(&mut self, ui: &mut UI) -> Result<()> {
        let cfg: NodeConfig = self.config.clone().node.clone().into();

        let api_address = cfg.api.public_api_address.unwrap_or(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            7000,
        ));

        ui.begin(&format!(
            "Rio/OS Blockchain listening on http://{}:{}",
            api_address.ip(),
            api_address.port()
        ))?;

        ui.heading("Ready to go.")?;

        let mut options = RocksDBOptions::default();
        options.create_if_missing(true);

        let node = Node::new(
            Box::new(RocksDB::open(ROCKSDB_DATA_PATH.as_path(), &options).unwrap()),
            vec![Box::new(Habitat)],
            cfg,
        );

        ui.para("Starting a single node...")?;
        ui.para("Blockchain is ready for transactions!")?;
        node.run().unwrap();

        ui.para("Ready to serve.")?;

        Ok(())
    }
}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(ui: &mut UI, config: Config) -> Result<()> {
    Server::new(config).run(ui)
}
