// Copyright 2018 The Rio Advancement Inc
//
use common::ui::UI;
use error::Result;
use api_client::Client;

pub fn edit_datacenter(ui: &mut UI, _rio_client: Client, _id: &str, _token: &str, _email: &str) -> Result<()> {
    let data = ui.edit(&["data"])?;
    println!("{:?}", data);
    Ok(())
}

pub fn edit_network(ui: &mut UI, _rio_client: Client, _id: &str, _token: &str, _email: &str) -> Result<()> {
    let data = ui.edit(&["data"])?;
    println!("{:?}", data);
    Ok(())
}
