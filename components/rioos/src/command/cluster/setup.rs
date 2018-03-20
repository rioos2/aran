// Copyright 2018 The Rio Advancement Inc
//
use std::fs::File;
use protocol::api::{network, storage};
use serde_yaml;
use common::ui::UI;
use error::Result;
use api_client::Client;

pub fn start(ui: &mut UI, rio_client: Client, cache_path: &str, token: &str, email: &str) -> Result<()> {
    ui.begin(&format!("Constructing a cluster for you..."))?;
    ui.br()?;
    let file = File::open(cache_path)?;
    let content: FileData = serde_yaml::from_reader(file)?;
    rio_client.create_network(content.network, &token, &email)?;
    rio_client.create_datacenter(
        content.datacenter,
        token,
        email,
    )?;
    ui.end("Your cluster is ready")?;
    Ok(())
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
struct FileData {
    network: network::Network,
    datacenter: storage::DataCenter,
}
