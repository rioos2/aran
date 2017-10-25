// Copyright (c) 2017 RioCorp Inc.


use std::path::Path;

use common::ui::UI;
use rioos_core::env;

use AUTH_TOKEN_ENVVAR;
use config;
use error::Result;

pub fn start(ui: &mut UI, cache_path: &Path) -> Result<()> {
    println!("{:?}", cache_path);

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

    let auth_token = login(ui, &userid, cache_path)?;

    write_cli_config_auth_token(&auth_token)?;

    ui.heading("Logged in.")?;
    ui.para("That's all for now. Thanks for using Rio/OS!")?;
    Ok(())
}


fn write_cli_config_auth_token(auth_token: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config::save(&config)
}


fn login(ui: &mut UI, origin: &str, cache_path: &Path) -> Result<String> {
    //    let result = command::origin::key::generate::start(ui, &origin, cache_path);
    ui.br()?;
    //    result
    Ok(" test ".to_string())
}

fn prompt_userid(ui: &mut UI) -> Result<String> {
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
