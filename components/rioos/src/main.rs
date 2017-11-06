// Copyright (c) 2017 RioCorp Inc.
//

#![recursion_limit="128"]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
extern crate env_logger;
extern crate rioos;
extern crate rioos_core as rcore;
extern crate rioos_common as common;
extern crate handlebars;
#[macro_use]
extern crate log;
extern crate base64;

use std::env;
use std::ffi::OsString;
use std::io;
use std::thread;

use clap::{ArgMatches, Shell};
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};
use rcore::crypto::init; //TO-DO: NOT NEEDED
use rcore::env as henv;

use rioos::{cli, command, config, AUTH_TOKEN_ENVVAR, AUTH_EMAIL_ENVVAR, ORIGIN_ENVVAR, API_SERVER_ENVVAR};
use rioos::error::{Error, Result};



fn main() {
    env_logger::init().unwrap();
    let mut ui = ui();

    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    let (args, remaining_args) = raw_parse_args();
    debug!("clap cli args: {:?}", &args);
    debug!("remaining cli args: {:?}", &remaining_args);

    // We build the command tree in a separate thread to eliminate
    // possible stack overflow crashes at runtime. OSX, for instance,
    // will crash with our large tree. This is a known issue:
    // https://github.com/kbknapp/clap-rs/issues/86
    let child = thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            return cli::get()
                .get_matches_from_safe_borrow(&mut args.iter())
                .unwrap_or_else(|e| { e.exit(); });
        })
        .unwrap();
    let app_matches = child.join().unwrap();
    match app_matches.subcommand() {
        ("cli", Some(matches)) => {
            match matches.subcommand() {
                ("init", Some(m)) => sub_cli_login(ui, m)?,
                ("list", Some(m)) => sub_cli_login(ui, m)?,
                ("completers", Some(m)) => sub_cli_completers(m)?,
                ("new", Some(m)) => sub_cli_new(ui, m)?,
                ("whoami",Some(m)) => sub_cli_whoami(ui,m)?,
                _ => unreachable!(),
            }
        }
        ("auth", Some(matches)) => {
            match matches.subcommand() {
                ("login", Some(m)) => sub_cli_login(ui, m)?,
                ("logout", Some(m)) => sub_cli_logout(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("digitalcloud", Some(matches)) => {
            match matches.subcommand() {
                ("deploy", Some(m)) => sub_digicloud_deploy(ui, m)?,
                ("list", Some(m)) => sub_digicloud_list(ui, m)?,
                ("describe", Some(m)) => sub_digicloud_decribe(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("nodes", Some(matches)) => {
            match matches.subcommand() {
                ("list", Some(m)) => sub_node_list(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("images", Some(matches)) => {
            match matches.subcommand() {
                ("list", Some(m)) => sub_images_list(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("datacenters", Some(matches)) => {
            match matches.subcommand() {
                ("list", Some(m)) => sub_datacenters_list(ui, m)?,
                ("get", Some(m)) => sub_datacenetrs_get(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("origin", Some(matches)) => {
            match matches.subcommand() {
                ("list", Some(m)) => sub_origin_list(ui, m)?,
                ("get", Some(m)) => sub_origin_get(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("jobs", Some(matches)) => {
            match matches.subcommand() {
                ("list", Some(m)) => sub_job_list(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("networks", Some(matches)) => {
            match matches.subcommand() {
                ("list", Some(m)) => sub_network_list(ui, m)?,
                _ => unreachable!(),
            }
        }

        ("login", Some(m)) => sub_cli_login(ui, m)?,
        ("logout", Some(m)) => sub_cli_logout(ui, m)?,
        ("init", Some(m)) => sub_cli_login(ui, m)?,
        ("list", Some(m)) => sub_cli_login(ui, m)?,
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_cli_login(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    init();

    command::cli::login::start(ui, &api_server_param_or_env(&m)?)
}

fn sub_cli_logout(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    init();

    command::cli::logout::start(ui, &api_server_param_or_env(&m)?)

}


fn sub_cli_new(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    init();

    command::cli::new::start(ui, &api_server_param_or_env(&m)?)

}
fn sub_cli_whoami(ui: &mut UI,m: &ArgMatches) -> Result<()> {
    init();

    command::cli::whoami::start(ui,&api_server_param_or_env(&m)?)

}


fn sub_cli_completers(m: &ArgMatches) -> Result<()> {
    let shell = m.value_of("SHELL").expect(
        "Missing Shell; A shell is required",
    );
    cli::get().gen_completions_to("rioos", shell.parse::<Shell>().unwrap(), &mut io::stdout());
    Ok(())
}


fn sub_digicloud_deploy(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    let config_file = m.value_of("CONFIG").map(|v| v.into());

    command::digicloud::deploy::start(
        ui,
        auth_token_param_or_env(&m)?,
        //api_server_param_or_env(&m)?,
        config_file,
    )
}


fn sub_digicloud_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    command::digicloud::list::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
    )
}
fn sub_node_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    command::node::list::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
    )
}
fn sub_images_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    command::image::list::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
    )
}

fn sub_datacenters_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    command::datacenter::list::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
    )
}
fn sub_origin_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    command::origin::list::start(ui,&api_server_param_or_env(&m)?,
    auth_token_param_or_env(&m)?,
    auth_email_param_or_env(&m)?,
)
}
fn sub_job_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    command::job::list::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
    )
}
fn sub_network_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {

    command::network::list::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
    )
}

fn sub_digicloud_decribe(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let config_file = m.value_of("DIGICLOUD_NAME").map(|v| v.into());

    command::digicloud::describe::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
        config_file.unwrap(),
    )
}

fn sub_datacenetrs_get(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let config_file = m.value_of("DATACENTER_ID").map(|v| v.into());

    command::datacenter::get::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
        config_file.unwrap(),
    )
}

fn sub_origin_get(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let config_file = m.value_of("ORG_IDENT").map(|v| v.into());

    command::origin::get::start(
        ui,
        &api_server_param_or_env(&m)?,
        auth_token_param_or_env(&m)?,
        auth_email_param_or_env(&m)?,
        config_file.unwrap(),
    )
}

fn ui() -> UI {
    let isatty = if henv::var(NONINTERACTIVE_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Some(false)
    } else {
        None
    };
    let coloring = if henv::var(NOCOLORING_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Coloring::Never
    } else {
        Coloring::Auto
    };
    UI::default_with(coloring, isatty)
}

/// Parse the raw program arguments and split off any arguments that will skip clap's parsing.
///
/// **Note** with the current version of clap there is no clean way to ignore arguments after a
/// certain point, especially if those arguments look like further options and flags.
fn raw_parse_args() -> (Vec<OsString>, Vec<OsString>) {
    let mut args = env::args();
    match (
        args.nth(1).unwrap_or_default().as_str(),
        args.next().unwrap_or_default().as_str(),
    ) {
        ("pkg", "exec") => {
            if args.by_ref().count() > 2 {
                return (
                    env::args_os().take(5).collect(),
                    env::args_os().skip(5).collect(),
                );
            } else {
                (env::args_os().collect(), Vec::new())
            }
        }
        _ => (env::args_os().collect(), Vec::new()),
    }
}

/// Check to see if the user has passed in an AUTH_TOKEN param. If not, check the
/// RIOOS_AUTH_TOKEN env var. If not, check the /rioos/etc/cli.yoml config if there is an
/// auth_token set. If that's empty too, then error.
fn auth_token_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("AUTH_TOKEN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(AUTH_TOKEN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = config::load()?;
                    match config.auth_token {
                        Some(v) => Ok(v),
                        None => return Err(Error::ArgumentError("No auth token specified")),
                    }
                }
            }
        }
    }
}



fn auth_email_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("EMAIL_TOKEN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(AUTH_EMAIL_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = config::load()?;
                    match config.email {
                        Some(v) => Ok(v),
                        None => return Err(Error::ArgumentError("No auth email specified")),
                    }
                }
            }
        }
    }
}

/// Check to see if the user has passed in an ORIGIN param.  If not, check the RIOOS_ORIGIN env
/// var. If not, check the /rioos/etc/cli.toml config if there is an origin. If that's empty too,
/// then error.
fn origin_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("ORIGIN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(ORIGIN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = config::load()?;
                    match config.origin {
                        Some(v) => Ok(v),
                        None => return Err(Error::ArgumentError("No origin specified")),
                    }
                }
            }
        }
    }
}

/// Check to see if the user has passed in an API_SERVER_ENVVAR param.  If not, check the RIOOS_API_SERVER env
/// var. If not, check the /rioos/etc/cli.toml config if there is an origin. If that's empty too,
/// then error.
fn api_server_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("API_SERVER") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(API_SERVER_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = config::load()?;
                    match config.api_server {
                        Some(v) => Ok(v),
                        None => return Err(Error::ArgumentError("No api_server specified")),
                    }
                }
            }
        }
    }
}
