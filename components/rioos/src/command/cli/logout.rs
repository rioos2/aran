// Copyright (c) 2017 RioCorp Inc.


use std::path::Path;

use common::ui::UI;
use rioos_core::env;

use AUTH_TOKEN_ENVVAR;
use config;
use error::Result;

pub fn start(ui: &mut UI, api: &str) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;

    ui.heading("Logout")?;
    ui.para(
        "For more information on authenticating using commandline, please read the \
          documentation at https://docs.rioos.sh/docs/identity-overview/",
    )?;

    ui.br()?;
    //api call
    //logout(ui, api)
    write_cli_config_auth_token("")?;

    ui.heading("Logged out.")?;
    ui.para("That's all for now. Thanks for using Rio/OS!")?;
    Ok(())
}


fn write_cli_config_auth_token(auth_token: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config::save(&config)
}


fn logout(ui: &mut UI, api: &str) -> Result<()> {
    //let result = command::origin::key::generate::start(ui, &origin, cache_path);
    //ui.br()?;
    //result
    Ok(())
}
