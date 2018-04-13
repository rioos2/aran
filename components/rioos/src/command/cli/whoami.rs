// Copyright 2018 The Rio Advancement Inc

pub use error::{Error, Result};
use common::ui::UI;
use config;

pub fn start(ui: &mut UI) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;
    let config = config::load()?;
    ui.br()?;
    if config.email.is_some() && !config.clone().email.unwrap().is_empty() {
        ui.para(&format!(
            "Currently logged in as {} with {} in {}",
            &config.email.unwrap(),
            &config.account.unwrap(),
            &config.api_server.unwrap()
        ))?;
    } else {
        ui.para(&format!("Currently No One Logged"))?;
    }
    Ok(())
}
