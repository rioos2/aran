// Copyright 2018 The Rio Advancement Inc
//
use std::fs::File;
use protocol::api::{network, storage};
use serde_yaml;
use common::ui::UI;
use error::Result;
use api_client::Client;

pub fn edit_datacenter(ui: &mut UI, rio_client: Client, id: &str, token: &str, email: &str) -> Result<()> {
    let data = ui.edit(&["data"])?;
    println!("{:?}", data);
    Ok(())
}

pub fn edit_network(ui: &mut UI, rio_client: Client, id: &str, token: &str, email: &str) -> Result<()> {
    let data = ui.edit(&["data"])?;
    println!("{:?}", data);
    Ok(())
}
