// Copyright 2018 The Rio Advancement Inc
//
use common::ui::UI;
use error::Result;
use api_client::Client;
use protocol::api::network;
use serde_json;



pub fn edit_datacenter(ui: &mut UI, _rio_client: Client, _id: &str, _token: &str, _email: &str) -> Result<()> {
    let data = ui.edit(&["data"])?;
    println!("{:?}", data);
    Ok(())
}

pub fn edit_network(ui: &mut UI, rio_client: Client, id: &str, token: &str, email: &str) -> Result<()> {
    let result: network::Network = rio_client.network_get_by_id(&token, &email, &id)?;
    let data = ui.edit(&result)?;
    let p: network::Network = serde_json::from_str(&data.to_string()).unwrap();
    rio_client.network_update(&token, &email, p)?;
    ui.end("Your network successfully updated!")?;
    Ok(())
}
