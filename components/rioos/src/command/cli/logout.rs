// Copyright (c) 2017 RioCorp Inc.


use common::ui::UI;
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
    //just blank out the auth token
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
