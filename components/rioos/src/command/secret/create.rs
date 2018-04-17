// Copyright 2018 The Rio Advancement Inc
//
use std::fs::File;
use protocol::api::secret::Secret;
use serde_yaml;
use common::ui::UI;
use error::Result;

use rcore::fs::open_from;
use api_client::Client;

pub fn start(ui: &mut UI, rio_client: Client, cache_path: &str, token: &str, email: &str) -> Result<()> {
    ui.begin(&format!("Constructing a secret for you..."))?;
    ui.br()?;
    let file = open_from(cache_path)?;
    let content: Secret = serde_yaml::from_reader(file)?;
    let origin = content.get_metadata().clone();
    rio_client.create_secret(content, &origin.get("origin").unwrap_or(&"rioos_system".to_string()), token, &email)?;
    ui.end("Your secret is ready")?;
    Ok(())
}
