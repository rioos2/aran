// Copyright 2018 The Rio Advancement Inc

//! ~~~~ This where everything starts: main starting point of the Rio.Marketplace server.
#[macro_use]
extern crate clap;
extern crate env_logger;

extern crate rioos_common as common;
extern crate rioos_core as rio_core;
extern crate rioos_marketplace_api as api;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

use rio_core::config::ConfigFile;
use rio_core::env as renv;
use rio_core::fs::rioconfig_config_path;
use common::ui::{Coloring, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR, UI};

use api::{command, Config, Error, Result};

lazy_static! {
    static  ref CFG_DEFAULT_FILE: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("marketplace.toml").to_str().unwrap());
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
        (version: api::VERSION)
        (about: "Rio.Marketplace api-server")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand start =>
            (about: "Run the rio.marketplaces server")
            (@arg config: -c --config +takes_value
                "Filepath to configuration file. [default: /var/lib/rioos/config/marketplaces.toml]")
            (@arg port: --port +takes_value "Listen port. [default: 6443]")

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
        ("migrate", Some(_)) => sub_cli_migrate(ui)?,
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_start_server(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    ui.begin(
        r#"
    ██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗    ███╗   ███╗ █████╗ ██████╗ ██╗  ██╗███████╗████████╗██████╗ ██╗      █████╗  ██████╗███████╗
    ██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ████╗ ████║██╔══██╗██╔══██╗██║ ██╔╝██╔════╝╚══██╔══╝██╔══██╗██║     ██╔══██╗██╔════╝██╔════╝
    ██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ██╔████╔██║███████║██████╔╝█████╔╝ █████╗     ██║   ██████╔╝██║     ███████║██║     █████╗
    ██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██║╚██╔╝██║██╔══██║██╔══██╗██╔═██╗ ██╔══╝     ██║   ██╔═══╝ ██║     ██╔══██║██║     ██╔══╝
    ██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ██║ ╚═╝ ██║██║  ██║██║  ██║██║  ██╗███████╗   ██║   ██║     ███████╗██║  ██║╚██████╗███████╗
    ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝    ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝   ╚═╝   ╚═╝     ╚══════╝╚═╝  ╚═╝ ╚═════╝╚══════╝

    "#,
    )?;

    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };
    sleep(Duration::new(2, 0));
    start(ui, config)
}

fn sub_cli_migrate(ui: &mut UI) -> Result<()> {
    command::cli::migrate::start(ui)
}

///
///
fn config_from_args(args: &clap::ArgMatches) -> Result<Config> {
    let mut config = load_config(args)?;

    if let Some(port) = args.value_of("port") {
        if u16::from_str(port).map(|p| config.https.port = p).is_err() {
            return Err(Error::BadPort(port.to_string()));
        }
    }

    Ok(config)
}

fn load_config(args: &clap::ArgMatches) -> Result<Config> {
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
fn start(ui: &mut UI, config: Config) -> Result<()> {
    api::server::run(ui, config)
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
