// Copyright (c) 2017 RioCorp Inc.
//

//! CLI for Rio/OS API setup

use std::path::Path;

use common::ui::UI;
use rio_core::crypto::SigKeyPair;
use rio_core::env;
use rio_core::package::ident;
use rio_core::Error::InvalidOrigin;

use command;
use error::Result;

pub const CA_ENVVAR: &'static str = "RIO_CA";

pub fn start(ui: &mut UI, cache_path: &Path) -> Result<()> {
    let mut generated_ca = false;
    let mut generated_api = false;
    let mut generated_serviceaccount = false;

    ui.br()?;
    ui.title("Rio/OS Setup")?;
    ui.para("Welcome to rio/os setup. Let's get started.")?;

    ui.heading("Set up a PKI infrastructure")?;
    ui.para(
        "Every system in Rio/OS belongs to an origin, which indicates the person or \
               organization responsible for maintaining that package. Each origin also has \
               a key used to cryptographically sign packages in that origin.",
    )?;

    ui.para(
        "Selecting a default origin tells package building operations such as 'hab \
                  pkg build' what key should be used to sign the packages produced. If you \
                  do not set a default origin now, you will have to tell package building \
                  commands each time what origin to use.",
    )?;
    ui.para(
        "For more information on pki infrastructure and how they are used in building packages, \
               please consult the docs at https://www.rioos.sh/docs/identity/",
    )?;
    if ask_default_ca(ui)? {
        ui.br()?;
        ui.para(
            "Enter the name of your certificate authority (CA). The default name is ca.crt, ca.key \
                      publicly, we recommend that you select one that is not already in use \
                      on the Rio/OS build service found at https://bldr.rioos.sh/.",
        )?;

        let mut ca = prompt_ca(ui)?;

        while !ident::is_valid_ca_name(&ca) {
            ui.br()?;
            ui.fatal(&format!("{}", InvalidOrigin(ca)))?;
            ui.br()?;

            ca = prompt_ca(ui)?;
        }

        ui.br()?;
        if is_ca_in_cache(&ca, cache_path) {
            ui.para(&format!(
                "You already have an certificate autority (CA) for {} created and \
                                   installed. Great work!",
                &ca
            ))?;
        } else {
            ui.heading("Create certificate authority")?;
            ui.para(&format!(
                "It doesn't look like you have a certificate authority \
                                `{}'. Without it, you won't be able to securely \
                                connect your infrastructure.",
                &ca
            ))?;
            ui.para(
                "For more information on the use of certificate authority, please consult \
                          the documentation at \
                          https://docs.rioos.sh/docs/concepts-keys/#origin-keys",
            )?;

            if ask_create_ca(ui, &ca)? {
                create_ca(ui, &ca, cache_path)?;
                generated_ca = true;
            } else {
                ui.para(&format!(
                    "You might want to create a certificate authority with: \
                                       `rioos setup {}'",
                    &ca
                ))?;
            }

            ui.heading("Create api key pair")?;
            ui.para(&format!(
                "It doesn't look like you have a api key pair \
                                `{}'. Without it, you won't be able to securely \
                                connect to your api server.",
                &ca
            ))?;

            ui.para(
                "For more information on the use of api key pair, please consult \
                          the documentation at \
                          https://docs.rioos.sh/docs/concepts-keys/#origin-keys",
            )?;

            let api = "";

            if ask_create_api(ui, &api)? {
                create_api(ui, &api, cache_path)?;
                generated_api = true;
            } else {
                ui.para(&format!(
                    "You might want to create an api key later with: \
                                       `rioos setup {}'",
                    &api
                ))?;
            }

            ui.heading("Create service account key pair")?;
            ui.para(&format!(
                "It doesn't look like you have a service account key pair \
                                `{}'. Without it, you won't be able to securely \
                                connect to your api server.",
                &ca
            ))?;

            ui.para(
                "For more information on the use of service account key pairs, please consult \
                          the documentation at \
                          https://docs.rioos.sh/docs/concepts-keys/#origin-keys",
            )?;

            let service_account = "";

            if ask_create_serviceaccount(ui, &service_account)? {
                create_serviceaccount(ui, &service_account, cache_path)?;
                generated_serviceaccount = true;
            } else {
                ui.para(&format!(
                    "You might want to create a service account key later with: \
                                       `rioos setup {}'",
                    &service_account
                ))?;
            }
        }


    } else {
        ui.para("Okay, maybe another time.")?;
    }


    ui.heading("Rio/OS Setup Complete")?;
    ui.para(&format!(
        "That's all for now. Thanks for using Rio/OS! {} {} {}",
        generated_ca,
        generated_api,
        generated_serviceaccount
    ))?;
    Ok(())
}

fn ask_default_ca(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        "Set up a default certificate authority (CA)?",
        Some(true),
    )?)
}

//prompt if the certifying authority ca exists.
fn prompt_ca(ui: &mut UI) -> Result<String> {
    let default = env::var(CA_ENVVAR).or(env::var("ca")).ok();

    Ok(ui.prompt_ask(
        "Default certifying authority name",
        default.as_ref().map(|x| &**x),
    )?)
}

fn is_ca_in_cache(origin: &str, cache_path: &Path) -> bool {
    match SigKeyPair::get_latest_pair_for(origin, cache_path, None) {
        Ok(pair) => {
            match pair.secret() {
                Ok(_) => true,
                _ => false,
            }
        }
        _ => false,
    }
}


fn ask_create_ca(ui: &mut UI, ca: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!("Create a certifate authority `{}'?", ca),
        Some(true),
    )?)
}


fn create_ca(ui: &mut UI, origin: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::start(ui, &origin, cache_path);
    ui.br()?;
    result
}


fn ask_create_api(ui: &mut UI, api: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!("Create an api key pair `{}'?", api),
        Some(true),
    )?)
}

fn create_api(ui: &mut UI, api: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::start(ui, &api, cache_path);
    ui.br()?;
    result
}

fn ask_create_serviceaccount(ui: &mut UI, service_account: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a service account `{}'?",
            service_account
        ),
        Some(true),
    )?)
}

fn create_serviceaccount(ui: &mut UI, service_account: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::start(ui, &service_account, cache_path);
    ui.br()?;
    result
}
