// Copyright 2018 The Rio Advancement Inc

pub use error::{Error, Result};

use common::ui::UI;

use api_client::Client;
use config;
use protocol::api::session;

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
    account.set_first_name(ui.prompt_ask("First Name", None)?);
    account.set_last_name(ui.prompt_ask("Last Name", None)?);
    account.set_email(ui.prompt_ask("Userid", None)?);
    account.set_password(ui.prompt_ask("Password", None)?);
    account.set_phone(ui.prompt_ask("phone", None)?);
    account.set_company_name(ui.prompt_ask("Company", None)?);
    account.set_registration_ip_address(ui.prompt_ask("Registration IP", None)?);

    let auth_token = signup(ui, client, account.clone())?;

    write_cli_config_auth_token(&auth_token, &account.get_email())?;

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

fn signup(ui: &mut UI, rio_client: Client, account: session::SessionCreate) -> Result<String> {
    ui.br()?;
    Ok(rio_client.signup(account)?)
}
