// Copyright 2018 The Rio Advancement Inc

use common::ui::UI;
use config;
use api_client::Client;
use error::Result;

pub fn start(ui: &mut UI, rio_client: Client, token: String, email: String) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;

    ui.heading("Logout")?;
    ui.para(
        "For more information on authenticating using commandline, please read the \
         documentation at https://bit.ly/rioos_sh_usersguide",
    )?;

    ui.br()?;
    rio_client.logout(&token, &email)?;

    //just blank out the auth token
    write_cli_config_auth_token("")?;
    write_cli_config_email("")?;
    write_cli_config_account("")?;

    ui.heading("Logged out.")?;
    ui.para("That's all for now. Thanks for using Rio/OS!")?;
    Ok(())
}

fn write_cli_config_auth_token(auth_token: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config::save(&config)
}

fn write_cli_config_email(email: &str) -> Result<()> {
    let mut config = config::load()?;
    config.email = Some(email.to_string());
    config::save(&config)
}
fn write_cli_config_account(account: &str) -> Result<()> {
    let mut config = config::load()?;
    config.account = Some(account.to_string());
    config::save(&config)
}
