// Copyright 2018 The Rio Advancement Inc
//
use std::path::Path;

use common::ui::UI;
use error::Result;
use protocol::api::{network, storage};
use serde_yaml;

use api_client::Client;
use rioos_core::fs::open_from;

pub fn start(
    ui: &mut UI,
    rio_client: Client,
    cache_path: &str,
    token: &str,
    email: &str,
) -> Result<()> {
    ui.begin(&format!("Constructing a cluster for you..."))?;
    ui.br()?;
    let file = open_from(Path::new(cache_path))?;
    let mut content: FileData = serde_yaml::from_reader(file)?;
    let network: network::Network =
        rio_client.create_network(content.network.clone(), token, email)?;
    content.datacenter.set_networks(vec![network.get_id()]);

    rio_client.create_datacenter(content.datacenter.clone(), token, email)?;
    ui.end("Your cluster is ready")?;
    Ok(())
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
struct FileData {
    network: network::Network,
    datacenter: storage::DataCenter,
}
