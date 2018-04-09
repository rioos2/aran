// Copyright 2018 The Rio Advancement Inc
//

//! CLI for Rio/OS API setup

use std::path::{Path, PathBuf};
use std::fs::File;
use std::str;
use base64;

use failure::SyncFailure;
use handlebars::Handlebars;
use config::Config;

use common::ui::UI;

use rio_core::crypto::{ROOT_CA, SigKeyPair};
use rio_core::fs::{write_to_file, read_from_file, rioconfig_config_path};

use command;
use error::Result;

lazy_static! {
    static  ref RIOCONFIG_TEMPLATE: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("template/rioconfig.hbs").to_str().unwrap());
}

///Sets up the Rio/OS infrastructure to connect via PKI.
pub fn start(ui: &mut UI, cache_path: &Path, config: &Config) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS Setup")?;
    ui.para("Welcome to rio/os setup. Let's get started.")?;

    ui.heading("Set up a PKI infrastructure")?;
    ui.para(
        "Every system in Rio/OS belongs to an certificate authority, which indicates the systems \
         identity responsible in the connected infrastructure. Each system has \
         a key used in cryptographically transporting data across other Rio/OS systems.",
    )?;

    ui.para(
        "For more information on pki infrastructure and how they are used in connecting your infrastructure, \
         please consult the docs at https://bit.ly/rioos_sh_admin_guide",
    )?;

    let server_ca = "server-ca";

    if is_server_ca_in_cache(&server_ca, cache_path) {
        ui.para(&format!(
            "You already have an certificate autority (CA) for {} created and \
             installed. Great work!",
            &server_ca
        ))?;
    } else {
        ui.heading("Create certificate authority")?;
        ui.para(&format!(
            "It doesn't look like you have a certificate authority \
             `{}'. Without it, you won't be able to securely \
             connect your infrastructure.",
            &server_ca
        ))?;
        ui.para(
            "For more information on the use of certificate authority, please consult \
             the documentation at \
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_server_ca(ui, &server_ca)? {
            create_server_ca(ui, &server_ca, cache_path)?;
        } else {
            ui.para(&format!(
                "You might want to create a server certificate authority with: \
                 `rioos setup '"
            ))?;
        }

        let client_cli = "client-cli";

        if ask_create_client_cli(ui, &client_cli)? {
            create_client_cli(ui, &client_cli, cache_path)?;
        } else {
            ui.para(&format!(
                "You might want to create a client certificate authority with: \
                 `rioos setup '"
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
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_api(ui, &api)? {
            create_api(ui, &api, cache_path)?;
        } else {
            ui.para(&format!(
                "You might want to create an api key later with: \
                 `rioos setup '"
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
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_serviceaccount(ui, &service_account)? {
            create_serviceaccount(ui, &service_account, cache_path)?;
        } else {
            ui.para(&format!(
                "You might want to create a service account key later with: \
                 `rioos setup '"
            ))?;
        }

        let controller = "client-controller";

        ui.heading("Create client-controller key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a client-controller key pair \
             `{}'. Without it, you won't be able to securely \
             connect to your api server.",
            &controller
        ))?;

        ui.para(
            "For more information on the use of client-controller key pair, please consult \
             the documentation at \
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_controller(ui, &controller)? {
            create_controller(ui, &controller, cache_path, config)?;
        } else {
            ui.para(&format!(
                "You might want to create an client-controller key later with: \
                 `rioos setup '"
            ))?;
        }

        let nodelet = "client-nodelet";

        ui.heading("Create client-nodelet key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a client-nodelet key pair \
             `{}'. Without it, you won't be able to securely \
             connect to your api server.",
            &nodelet
        ))?;

        ui.para(
            "For more information on the use of client-nodelet key pair, please consult \
             the documentation at \
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_nodelet(ui, &nodelet)? {
            create_nodelet(ui, &nodelet, cache_path, config)?;
        } else {
            ui.para(&format!(
                "You might want to create an client-nodelet key later with: \
                 `rioos setup '"
            ))?;
        }

        let storelet = "client-storlet";

        ui.heading("Create client-storelet key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a client-storelet key pair \
             `{}'. Without it, you won't be able to securely \
             connect to your api server.",
            &storelet
        ))?;

        ui.para(
            "For more information on the use of client-storelet key pair, please consult \
             the documentation at \
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_storelet(ui, &storelet)? {
            create_storelet(ui, &storelet, cache_path, config)?;
        } else {
            ui.para(&format!(
                "You might want to create an client-storelet key later with: \
                 `rioos setup '"
            ))?;
        }

        let scheduler = "client-scheduler";

        ui.heading("Create client-scheduler key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a client-scheduler key pair \
             `{}'. Without it, you won't be able to securely \
             connect to your api server.",
            &scheduler
        ))?;

        ui.para(
            "For more information on the use of client-scheduler key pair, please consult \
             the documentation at \
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_scheduler(ui, &scheduler)? {
            create_scheduler(ui, &scheduler, cache_path, config)?;
        } else {
            ui.para(&format!(
                "You might want to create an client-scheduler key later with: \
                 `rioos setup '"
            ))?;
        }


        let gulp = "client-gulp";

        ui.heading("Create client-gulp key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a client-gulp key pair \
             `{}'. Without it, you won't be able to securely \
             connect to your api server.",
            &gulp
        ))?;

        ui.para(
            "For more information on the use of client-gulp key pair, please consult \
             the documentation at \
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_gulp(ui, &gulp)? {
            create_gulp(ui, &gulp, cache_path, config)?;
        } else {
            ui.para(&format!(
                "You might want to create an client-gulp key later with: \
                 `rioos setup '"
            ))?;
        }

        let prometheus = "client-prometheus";

        ui.heading("Create client-prometheus key pair")?;
        ui.para(&format!(
            "It doesn't look like you have a client-prometheus key pair \
             `{}'. Without it, you won't be able to securely \
             connect to your api server.",
            &prometheus
        ))?;

        ui.para(
            "For more information on the use of client-prometheus key pair, please consult \
             the documentation at \
             https://bit.ly/rioos_sh_admin_guide",
        )?;

        if ask_create_prometheus(ui, &prometheus)? {
            create_prometheus(ui, &prometheus, cache_path)?;
        } else {
            ui.para(&format!(
                "You might want to create an client-prometheus key later with: \
                 `rioos setup '"
            ))?;
        }
    }

    create_rioos_setup_file(cache_path, ".rioos_setup_complete")?;

    ui.heading("Rio/OS Setup Complete")?;
    ui.para(
        &format!("That's all for now. Thanks for using Rio/OS!"),
    )?;
    Ok(())
}

fn is_server_ca_in_cache(server_ca: &str, cache_path: &Path) -> bool {
    match SigKeyPair::get_pair_for(server_ca, cache_path) {
        Ok(pair) => {
            match pair.secret() {
                Ok(_) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn ask_create_server_ca(ui: &mut UI, ca: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a server certificate authority `{}'?",
            ca
        ),
        Some(true),
    )?)
}

fn create_server_ca(ui: &mut UI, server_ca: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::start(ui, &server_ca, cache_path);
    ui.br()?;
    result
}

fn ask_create_client_cli(ui: &mut UI, ca: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a client certificate authority `{}'?",
            ca
        ),
        Some(true),
    )?)
}

fn create_client_cli(ui: &mut UI, client_cli: &str, cache_path: &Path) -> Result<()> {
    command::origin::key::generate::signed_with_x509(ui, &client_cli, cache_path)?;
    ui.br()?;
    Ok(())
}

fn ask_create_api(ui: &mut UI, api: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!("Create an api key pair `{}'?", api),
        Some(true),
    )?)
}

//redundant (create_api and create_serviceaccount)
fn create_api(ui: &mut UI, api: &str, cache_path: &Path) -> Result<()> {
    command::origin::key::generate::signed_with_pfx(ui, &api, cache_path)?;
    ui.br()?;
    Ok(())
}

fn ask_create_controller(ui: &mut UI, controller: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a client-controller key pair `{}'?",
            controller
        ),
        Some(true),
    )?)
}

//redundant (create_api and create_serviceaccount)
fn create_controller(ui: &mut UI, controller: &str, cache_path: &Path, config: &Config) -> Result<()> {
    let result = command::origin::key::generate::signed(ui, &controller, cache_path)?;
    ui.br()?;
    create_rioconfig(&result, cache_path, "controller.rioconfig", config)?;
    Ok(())
}

fn ask_create_nodelet(ui: &mut UI, nodelet: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a client-nodelet key pair `{}'?",
            nodelet
        ),
        Some(true),
    )?)
}

//redundant (create_api and create_serviceaccount)
fn create_nodelet(ui: &mut UI, nodelet: &str, cache_path: &Path, config: &Config) -> Result<()> {
    let result = command::origin::key::generate::signed(ui, &nodelet, cache_path)?;
    ui.br()?;
    create_rioconfig(&result, cache_path, "nodelet.rioconfig", config)?;
    Ok(())
}

fn ask_create_storelet(ui: &mut UI, storelet: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a client-storelet key pair `{}'?",
            storelet
        ),
        Some(true),
    )?)
}

//redundant (create_api and create_serviceaccount)
fn create_storelet(ui: &mut UI, storelet: &str, cache_path: &Path, config: &Config) -> Result<()> {
    let result = command::origin::key::generate::signed(ui, &storelet, cache_path)?;
    ui.br()?;
    create_rioconfig(&result, cache_path, "storlet.rioconfig", config)?;
    Ok(())
}

fn ask_create_scheduler(ui: &mut UI, scheduler: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a client-scheduler key pair `{}'?",
            scheduler
        ),
        Some(true),
    )?)
}

//redundant (create_api and create_serviceaccount)
fn create_scheduler(ui: &mut UI, scheduler: &str, cache_path: &Path, config: &Config) -> Result<()> {
    let result = command::origin::key::generate::signed(ui, &scheduler, cache_path)?;
    ui.br()?;
    create_rioconfig(&result, cache_path, "scheduler.rioconfig", config)?;
    Ok(())
}

fn ask_create_gulp(ui: &mut UI, gulp: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!("Create a client-agent key pair `{}'?", gulp),
        Some(true),
    )?)
}

//redundant (create_api and create_serviceaccount)
fn create_gulp(ui: &mut UI, gulp: &str, cache_path: &Path, config: &Config) -> Result<()> {
    let result = command::origin::key::generate::signed(ui, &gulp, cache_path)?;
    ui.br()?;
    create_rioconfig(&result, cache_path, "gulp.rioconfig", config)?;
    Ok(())
}


fn ask_create_prometheus(ui: &mut UI, prometheus: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!(
            "Create a client-prometheus key pair `{}'?",
            prometheus
        ),
        Some(true),
    )?)
}

//redundant (create_api and create_serviceaccount)
fn create_prometheus(ui: &mut UI, prometheus: &str, cache_path: &Path) -> Result<()> {
    command::origin::key::generate::signed_with_x509(ui, &prometheus, cache_path)?;
    ui.br()?;
    Ok(())
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
    command::origin::key::generate::signed(ui, &service_account, cache_path)?;
    ui.br()?;
    Ok(())
}

fn create_rioconfig(result: &SigKeyPair, cache_path: &Path, name: &str, config: &Config) -> Result<()> {
    let server_ca = SigKeyPair::get_pair_for(ROOT_CA, cache_path)?;

    let json = json!({
        "key":  base64::encode(&result.secret()?),
        "cert": base64::encode(&result.public()?),
        "server_ca": base64::encode(&server_ca.public()?),
        "ip":config.http.listen,
        "port": config.http.port,
    });
    let r = Handlebars::new()
        .render_template(&read_from_file(&RIOCONFIG_TEMPLATE)?, &json)
        .map_err(SyncFailure::new);
    let s = r.unwrap()
        .lines()
        .filter(|l| *l != "")
        .collect::<Vec<_>>()
        .join("\n") + "\n";
    write_to_file(&cache_path.join(name), &s)?;
    Ok(())
}

fn create_rioos_setup_file(cache_path: &Path, file_name: &str) -> Result<()> {
    try!(File::create(cache_path.join(file_name)));
    Ok(())
}
