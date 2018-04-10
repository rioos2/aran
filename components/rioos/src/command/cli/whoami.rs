// Copyright 2018 The Rio Advancement Inc

pub use error::{Error, Result};
use common::ui::UI;
use config;

pub fn start(ui: &mut UI) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;

    let config = config::load()?;
    ui.br()?;
    ui.para(&format!(
        "Currently logged in as {}({}) @ {}",
        &config.email.unwrap(),
        &config.account.unwrap(),
        &config.api_server.unwrap()
    ))?;
    Ok(())
}
