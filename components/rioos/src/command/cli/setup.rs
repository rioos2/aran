// Copyright (c) 2017 RioCorp Inc.


use std::path::Path;

use common::ui::UI;
use hcore::env;

use {AUTH_TOKEN_ENVVAR};
use config;
use error::Result;

pub fn start(ui: &mut UI, cache_path: &Path) -> Result<()> {
    println!("{:?}", cache_path);

    ui.br()?;
    ui.title("Rio/OS CLI Setup")?;
    ui.para("Welcome to rioos setup. Let's get started.")?;

    ui.heading("Login")?;
    ui.para(
        "For more information on authenticating using commandline, please read the \
          documentation at https://docs.rioos.sh/docs/identity-overview/",
    )?;
    if ask_default_auth_token(ui)? {
        ui.br()?;
        ui.para("Enter your userid.")?;
        let auth_token = prompt_auth_token(ui)?;
        write_cli_config_auth_token(&auth_token)?;
    } else {
        ui.para("Okay, maybe another time.")?;
    }
    ui.heading("CLI Setup Complete")?;
    ui.para("That's all for now. Thanks for using Rio/OS!")?;
    Ok(())
}


fn write_cli_config_auth_token(auth_token: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config::save(&config)
}

/*
fn create_origin(ui: &mut UI, origin: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::start(ui, &origin, cache_path);
    ui.br()?;
    result
}

fn prompt_origin(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.origin {
        Some(o) => {
            ui.para(&format!(
                "You already have a default origin set up as `{}', but feel \
                                free to change it if you wish.",
                &o
            ))?;
            Some(o)
        }
        None => env::var(ORIGIN_ENVVAR).or(env::var("USER")).ok(),
    };
    Ok(ui.prompt_ask(
        "Default origin name",
        default.as_ref().map(|x| &**x),
    )?)
}*/

fn ask_default_auth_token(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        "Set up a default GitHub access token?",
        Some(true),
    )?)
}

fn prompt_auth_token(ui: &mut UI) -> Result<String> {
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
    Ok(ui.prompt_ask(
        "GitHub access token",
        default.as_ref().map(|x| &**x),
    )?)
}
