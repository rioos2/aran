// Copyright 2018 The Rio Advancement Inc

use common::ui::UI;
use config;
pub use error::{Error, Result};

pub fn start(ui: &mut UI) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;
    let config = config::load()?;
    ui.br()?;
    if config.email.is_some() && !config.clone().email.unwrap().is_empty() {
        ui.para(&format!(
            "Currently logged in as {}",
            &config.email.unwrap(),
        ))?;
        ui.para(&format!(
            "Authorization Token: {}",
            &config.auth_token.unwrap()
        ))?;
        ui.para(&format!("Account Id: {}", &config.account.unwrap()))?;
        ui.para(&format!("API Gateway: {}", &config.api_server.unwrap()))?;
    } else {
        ui.para(&format!("☛ Not Logged in"))?;
    }
    Ok(())
}
