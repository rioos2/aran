// Copyright (c) 2017 RioCorp Inc.


pub use error::{Error, Result};

use common::ui::UI;
use rioos_core::env;

use api_client::Client;
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
    let userid = prompt_userid(ui)?;
    let password = prompt_password(ui)?;

    let auth_token = login(ui, url, &userid, &password)?;

    write_cli_config_auth_token(&auth_token, &userid)?;

    ui.heading("Logged in.")?;
    ui.para("That's all for now. Thanks for using Rio/OS!")?;
    Ok(())
}


fn write_cli_config_auth_token(auth_token: &str, email: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config.email = Some(email.to_string());
    config::save(&config)
}


fn login(ui: &mut UI, url: &str, userid: &str, password: &str) -> Result<String> {
    let rio_client = Client::new(url, PRODUCT, VERSION, None)?;

    ui.br()?;

    let result = rio_client.login(userid, password)?;

    Ok(result)
}

fn prompt_userid(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.email {
        Some(o) => {
            ui.para(
                "You already have a default auth token set up, but feel free to change it \
                       if you wish.",
            )?;
            Some(o)
        }
        None => env::var(AUTH_TOKEN_ENVVAR).ok(),
    };
    Ok(ui.prompt_ask("Userid", default.as_ref().map(|x| &**x))?)
}


fn prompt_password(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.auth_token {
        Some(o) => {
            ui.para(
                "You already have a default auth token set up, but feel free to change it \
                       if you wish.",
            )?;
            Some(o)
        }
        None => env::var(AUTH_TOKEN_ENVVAR).ok(),
    };
    Ok(ui.prompt_ask("Password", default.as_ref().map(|x| &**x))?)
}
