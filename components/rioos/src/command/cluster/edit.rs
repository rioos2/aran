// Copyright 2018 The Rio Advancement Inc
//
use api_client::Client;
use common::ui::UI;
use error::Result;
use protocol::api::{network, storage};
use serde_json;

pub fn edit_datacenter(
    ui: &mut UI,
    rio_client: Client,
    id: &str,
    token: &str,
    email: &str,
) -> Result<()> {
    let result: storage::DataCenter = rio_client.datacenter_get_by_id(&token, &email, &id)?;
    let data = ui.edit(&result)?;
    let datacenter: storage::DataCenter = serde_json::from_str(&data.to_string())?;
    rio_client.datacenter_update(&token, &email, datacenter)?;
    ui.end("Your datacenter updated successfully!.")?;
    Ok(())
}

pub fn edit_network(
    ui: &mut UI,
    rio_client: Client,
    id: &str,
    token: &str,
    email: &str,
) -> Result<()> {
    let result: network::Network = rio_client.network_get_by_id(&token, &email, &id)?;
    let data = ui.edit(&result)?;
    let network: network::Network = serde_json::from_str(&data.to_string())?;
    rio_client.network_update(&token, &email, network)?;
    ui.end("Your network updated successfully!.")?;
    Ok(())
}
