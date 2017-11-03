// Copyright (c) 2017 RioCorp Inc.


pub use error::{Error, Result};

use common::ui::UI;
use rioos_core::env;

use api_client::{self, Client};
use AUTH_TOKEN_ENVVAR;
use {PRODUCT, VERSION};
use config;



pub fn start(ui: &mut UI, url: &str) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;

    ui.heading("Login")?;
    ui.para(
        "For more information on authenticating using commandline, please read the \
          documentation at https://docs.rioos.sh/docs/identity-overview/",
    )?;

    ui.br()?;
    ui.para("Enter your credentials.")?;
    let userid = ui.prompt_ask("Userid", None)?;
    let password = ui.prompt_ask("Password", None)?;

    let auth_token = signup(ui, url, &userid, &password)?;

    write_cli_config_auth_token(&auth_token, &userid)?;

    ui.heading("Create new account in rioos and Logged in.")?;
    ui.para("That's all for now. Thanks for using Rio/OS!")?;
    Ok(())
}


fn write_cli_config_auth_token(auth_token: &str, email: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config.email = Some(email.to_string());
    config::save(&config)
}


fn signup(ui: &mut UI, url: &str, userid: &str, password: &str) -> Result<String> {
    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    ui.br()?;

    let result = rio_client.signup(userid, password)?;

    Ok(result)
}
