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

pub const ORIGIN_ENVVAR: &'static str = "RIO_ORIGIN";

pub fn start(ui: &mut UI, cache_path: &Path) -> Result<()> {
    let mut generated_origin = false;

    ui.br()?;
    ui.title("Rio/OS Setup")?;
    ui.para("Welcome to rio/os setup. Let's get started.")?;

    ui.heading("Set up a default origin")?;
    ui.para(
        "Every package in Rio/OS belongs to an origin, which indicates the person or \
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
        "For more information on origins and how they are used in building packages, \
               please consult the docs at https://www.rioos.sh/docs/create-packages-build/",
    )?;
    if ask_default_origin(ui)? {
        ui.br()?;
        ui.para(
            "Enter the name of your origin. If you plan to publish your packages \
                      publicly, we recommend that you select one that is not already in use \
                      on the Rio/OS build service found at https://bldr.rioos.sh/.",
        )?;
        ui.para(&format!(
            "Origins must begin with a lowercase letter or number. \
                Allowed characters include lowercase letters, numbers, _, -. \
                No more than 255 characters {}.", generated_origin
        ))?;
        let mut origin = prompt_origin(ui)?;

        while !ident::is_valid_origin_name(&origin) {
            ui.br()?;
            ui.fatal(&format!("{}", InvalidOrigin(origin)))?;
            ui.br()?;

            origin = prompt_origin(ui)?;
        }

        //write_cli_config_origin(&origin)?;

        ui.br()?;
        if is_origin_in_cache(&origin, cache_path) {
            ui.para(&format!(
                "You already have an origin key for {} created and \
                                   installed. Great work!",
                &origin
            ))?;
        } else {
            ui.heading("Create origin key pair")?;
            ui.para(&format!(
                "It doesn't look like you have a signing key for the origin \
                                `{}'. Without it, you won't be able to build new packages \
                                successfully.",
                &origin
            ))?;
            ui.para(
                "You can either create a new signing key now, or, if you are building \
                       packages for an origin that already exists, ask the owner to give \
                       you the signing key.",
            )?;
            ui.para(
                "For more information on the use of origin keys, please consult \
                          the documentation at \
                          https://www.rioos.sh/docs/concepts-keys/#origin-keys",
            )?;
            if ask_create_origin(ui, &origin)? {
                create_origin(ui, &origin, cache_path)?;
                generated_origin = true;
            } else {
                ui.para(&format!(
                    "You might want to create an origin key later with: \
                                       `hab origin key generate {}'",
                    &origin
                ))?;
            }
        }
    } else {
        ui.para("Okay, maybe another time.")?;
    }


    ui.heading("CLI Setup Complete")?;
    ui.para(&format!(
        "That's all for now. Thanks for using Rio/OS! {}", generated_origin
    ))?;
    Ok(())
}

fn ask_default_origin(ui: &mut UI) -> Result<bool> {
    Ok(ui.prompt_yes_no("Set up a default origin?", Some(true))?)
}

fn ask_create_origin(ui: &mut UI, origin: &str) -> Result<bool> {
    Ok(ui.prompt_yes_no(
        &format!("Create an origin key for `{}'?", origin),
        Some(true),
    )?)
}

/*fn write_cli_config_origin(origin: &str) -> Result<()> {
    /*let mut config = config::load()?;
    config.origin = Some(origin.to_string());
    config::save(&config)
    */
    Ok(())
}*/

fn is_origin_in_cache(origin: &str, cache_path: &Path) -> bool {
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

fn create_origin(ui: &mut UI, origin: &str, cache_path: &Path) -> Result<()> {
    let result = command::origin::key::generate::start(ui, &origin, cache_path);
    ui.br()?;
    result
}

fn prompt_origin(ui: &mut UI) -> Result<String> {
    /*let config = config::load()?;
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
    */
    let tet = env::var(ORIGIN_ENVVAR).or(env::var("USER"));
    ui.para(&format!(
        "Testing {:?}", tet))?;

    Ok(
        format!(
            "You already have a default origin set up as `{}', but feel \
                            free to change it if you wish.",
            "test"
        )
    )
}
