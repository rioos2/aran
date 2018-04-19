// Copyright 2018 The Rio Advancement Inc

pub use error::{Error, Result};

use common::ui::UI;

use api_client::Client;
use config;
use rioos_core::env;
use protocol::api::session;
use AUTH_TOKEN_ENVVAR;

pub fn start(ui: &mut UI, client: Client) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;

    ui.heading("Signup")?;
    ui.para(
        "For more information on authenticating using commandline, please read the \
         documentation at https://docs.rioos.sh/docs/identity-overview/",
    )?;

    ui.br()?;
    ui.para("Enter your credentials.")?;

    let mut account = session::SessionCreate::new();
    account.set_first_name(prompt_firstname(ui)?);
    account.set_last_name(prompt_lastname(ui)?);
    account.set_email(ui.prompt_ask("Userid", None)?);
    account.set_password(ui.prompt_ask("password", None)?);
    account.set_phone(ui.prompt_ask("phone", None)?);
    account.set_company_name(prompt_company(ui)?);

    let account: session::Session = signup(ui, client, account.clone())?;

    write_cli_config_auth_token(
        &account.get_token(),
        &account.get_email(),
        &account.get_id(),
    )?;

    ui.heading("Create new account in rioos and Logged in.")?;
    ui.para("That's all for now. Thanks for using Rio/OS!")?;
    Ok(())
}

fn write_cli_config_auth_token(auth_token: &str, email: &str, account: &str) -> Result<()> {
    let mut config = config::load()?;
    config.auth_token = Some(auth_token.to_string());
    config.email = Some(email.to_string());
    config.account = Some(account.to_string());
    config::save(&config)
}

fn signup(ui: &mut UI, rio_client: Client, account: session::SessionCreate) -> Result<session::Session> {
    ui.br()?;
    Ok(rio_client.signup(account)?)
}

fn prompt_firstname(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.firstname {
        Some(o) => Some(o),
        None => env::var(AUTH_TOKEN_ENVVAR).ok(),
    };
    Ok(ui.prompt_ask("First Name", default.as_ref().map(|x| &**x))?)
}

fn prompt_lastname(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.lastname {
        Some(o) => Some(o),
        None => env::var(AUTH_TOKEN_ENVVAR).ok(),
    };
    Ok(ui.prompt_ask("Last Name", default.as_ref().map(|x| &**x))?)
}

fn prompt_company(ui: &mut UI) -> Result<String> {
    let config = config::load()?;
    let default = match config.company {
        Some(o) => Some(o),
        None => env::var(AUTH_TOKEN_ENVVAR).ok(),
    };
    Ok(ui.prompt_ask("Company", default.as_ref().map(|x| &**x))?)
}
