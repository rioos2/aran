// Copyright 2018 The Rio Advancement Inc

//! ~~~~ This where everything starts: main starting point of the Rio/OS Aran server.
#[macro_use]
extern crate clap;
extern crate env_logger;

extern crate rioos_aran_api as api;
extern crate rioos_common as common;
extern crate rioos_core as rio_core;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::path::PathBuf;
use std::fs::File;

use rio_core::config::ConfigFile;
use rio_core::env as renv;
use rio_core::crypto::{default_rioconfig_key_path, init};
use rio_core::fs::rioconfig_config_path;
use common::ui::{Coloring, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR, UI};

use api::Config;
use api::validator::ConfigValidator;
use api::{command, Error, Result};
use api::node::Servers;

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

lazy_static! {
    static ref CFG_DEFAULT_FILE: PathBuf = PathBuf::from(&*rioconfig_config_path(None).join("api.toml").to_str().unwrap());
    static ref SETUP_COMPLETE_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None).join(".rioos_setup_complete").to_str().unwrap());
    static ref MARKETPLACE_CACHE_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None).join("pullcache/marketplaces.yaml").to_str().unwrap());
}

fn main() {
    env_logger::init();
    let mut ui = ui();
    let matches = app().get_matches();

    if let Err(e) = exec_subcommand_if_called(&mut ui, &matches) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(RIOOSAran =>
        (version: VERSION)
        (about: "Rio/OS api-server")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand start =>
            (about: "Run the api server")
            (@arg config: -c --config +takes_value
                "Filepath to configuration file. [default: /var/lib/rioos/config/api.toml]")
            (@arg port: --port +takes_value "Listen port(https). [default: 7443]")
            (@arg streamer_port: --streamer_port +takes_value "Listen streamer port(http2). [default: 8443]")
            (@arg uistreamer_port: --uistreamer_port +takes_value "Listen uistreamer port(wss). [default: 9443]")
            (@arg streamer: --streamer +takes_value "Start http2 streamer server. [default: false]")
            (@arg uistreamer: --uistreamer +takes_value "Start websocket streamer server. [default: false]")

        )
        (@subcommand setup =>
            (about: "Setup api server")
        )

        (@subcommand sync =>
            (about: "Sync Rio.Marketplaces with api server")
        )

        (@subcommand migrate =>
            (about: "Run migration on database - rioosdb")
        )

    )
}

fn exec_subcommand_if_called(ui: &mut UI, app_matches: &clap::ArgMatches) -> Result<()> {
    debug!("CLI matches: {:?}", app_matches);

    match app_matches.subcommand() {
        ("start", Some(m)) => sub_start_server(ui, m)?,
        ("setup", Some(m)) => sub_cli_setup(ui, m)?,
        ("sync", Some(m)) => sub_cli_sync(ui, m)?,
        ("migrate", Some(_)) => sub_cli_migrate(ui)?,
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_cli_setup(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    init();
    let config = match config_for_setup(&matches) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    command::cli::setup::start(ui, &default_rioconfig_key_path(None), &config)
}

fn sub_cli_sync(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    init();

    let config = match config_for_setup(&matches) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    command::cli::sync::start(ui, &config)
}

fn sub_cli_migrate(ui: &mut UI) -> Result<()> {
    init();
    command::cli::migrate::start(ui)
}

fn sub_start_server(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    if File::open(&SETUP_COMPLETE_FILE.as_path()).is_err() {
        return Err(Error::SetupNotDone);
    }

    if File::open(&MARKETPLACE_CACHE_FILE.as_path()).is_err() {
        return Err(Error::SyncNotDone);
    }

    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    start(ui, config, servertype_from_args(&matches))
}

fn servertype_from_args(args: &clap::ArgMatches) -> Servers {
    if args.value_of("streamer").is_some() {
        return Servers::STREAMER;
    }

    match args.value_of("uistreamer") {
        Some(_flag) => return Servers::UISTREAMER,
        None => return Servers::APISERVER,
    }
}

///
///
fn config_from_args(args: &clap::ArgMatches) -> Result<Config> {
    let mut config = match args.value_of("config") {
        Some(cfg_path) => try!(Config::from_file(cfg_path)),
        None => {
            let mut default_config = Config::default();

            Config::from_file(CFG_DEFAULT_FILE.to_str().unwrap()).unwrap_or(default_config)
        }
    };

    if let Some(port) = args.value_of("port") {
        if u16::from_str(port).map(|p| config.https.port = p).is_err() {
            return Err(Error::BadPort(port.to_string()));
        }
    }

    if let Some(streamer_port) = args.value_of("streamer_port") {
        if u16::from_str(streamer_port)
            .map(|p| config.http2.port = p)
            .is_err()
        {
            return Err(Error::BadPort(streamer_port.to_string()));
        }
    }

    if let Some(uistreamer_port) = args.value_of("uistreamer_port") {
        if u16::from_str(uistreamer_port)
            .map(|p| config.http2.websocket = p)
            .is_err()
        {
            return Err(Error::BadPort(uistreamer_port.to_string()));
        }
    }

    config.valid()?;

    Ok(config)
}

fn config_for_setup(args: &clap::ArgMatches) -> Result<Config> {
    let config = match args.value_of("config") {
        Some(cfg_path) => try!(Config::from_file(cfg_path)),
        None => {
            let mut default_config = Config::default();
            Config::from_file(CFG_DEFAULT_FILE.to_str().unwrap()).unwrap_or(default_config)
        }
    };
    Ok(config)
}

/// Starts the aran-api server.
/// # Failures
/// * Fails if the postgresql dbr fails to be found - cannot bind to the port, etc.
fn start(ui: &mut UI, config: Config, server: Servers) -> Result<()> {
    api::server::run(ui, config, server)
}

fn ui() -> UI {
    let isatty = if renv::var(NONINTERACTIVE_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Some(false)
    } else {
        None
    };
    let coloring = if renv::var(NOCOLORING_ENVVAR)
        .map(|val| val == "true")
        .unwrap_or(false)
    {
        Coloring::Never
    } else {
        Coloring::Auto
    };
    UI::default_with(coloring, isatty)
}
