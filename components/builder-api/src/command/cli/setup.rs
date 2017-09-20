// Copyright (c) 2017 RioCorp Inc.
//

//! CLI for Rio/OS API setup

use std::path::Path;

use common::ui::UI;
use rio_core::crypto::SigKeyPair;

use command;
use error::Result;

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
        "For more information on pki infrastructure and how they are used in building packages, \
               please consult the docs at https://www.rioos.sh/docs/identity/",
    )?;

    let ca = "ca";

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

        let api = "api-server";

        ui.heading("Create api key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a api key pair \
                                `{}'. Without it, you won't be able to securely \
                                connect to your api server.",
            &api
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

        let service_account = "service-account";

        ui.heading("Create service account key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a service account key pair \
                                `{}'. Without it, you won't be able to securely \
                                connect to your api server.",
            &service_account
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



    ui.heading("Rio/OS Setup Complete")?;
    ui.para(&format!(
        "That's all for now. Thanks for using Rio/OS! {} {} {}",
        generated_ca,
        generated_api,
        generated_serviceaccount
    ))?;
    Ok(())
}

fn is_ca_in_cache(origin: &str, cache_path: &Path) -> bool {
    match SigKeyPair::get_pair_for(origin, cache_path) {
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

//redundant (create_api and create_serviceaccount)
fn create_api(ui: &mut UI, api: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::signed(ui, &api, cache_path);
    ui.br()?;
    result
}

//redundant (ask_create_api and ask_create_serviceaccount)
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
    let result = command::origin::key::generate::signed(ui, &service_account, cache_path);
    ui.br()?;
    result
}
