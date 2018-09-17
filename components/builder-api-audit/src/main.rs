// Copyright 2018 The Rio Advancement Inc

//! ~~~~ This where everything starts: main starting point of the Rio/OS Audit Blockchain server.

#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

extern crate rioos_aran_blockchain as blockchain;
extern crate rioos_common as common;
extern crate rioos_core as rio_core;

use std::path::PathBuf;

#[macro_use]
extern crate log;

extern crate exonum;

use rio_core::config::ConfigFile;
use rio_core::env as renv;
use rio_core::fs::rioconfig_config_path;

use common::ui::{Coloring, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR, UI};

use blockchain::{server, Config, NodeInternalConfig, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

lazy_static! {
    static ref CFG_DEFAULT_FILE: PathBuf = PathBuf::from(&*rioconfig_config_path(None)
        .join("blockchain.toml")
        .to_str()
        .unwrap());
}

fn main() {
    exonum::helpers::init_logger().unwrap();

    let mut ui = ui();
    let matches = app().get_matches();

    if let Err(e) = exec_subcommand_if_called(&mut ui, &matches) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(RIOOSAudit =>
        (version: VERSION)
        (about: "Rio/OS blockchain-server")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand start =>
            (about: "Run the audit blockchain server")
            (@arg config: -c --config +takes_value
                "Filepath to configuration file. [default: $RIOOS_HOME/config/blockchain.toml]")
        )
    )
}

fn exec_subcommand_if_called(ui: &mut UI, app_matches: &clap::ArgMatches) -> Result<()> {
    debug!("CLI matches: {:?}", app_matches);

    match app_matches.subcommand() {
        ("start", Some(m)) => sub_start_server(ui, m)?,
        _ => unreachable!(),
    };
    Ok(())
}

//&default_rioconfig_key_path(None)
fn sub_start_server(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    ui.begin(
        r#"
██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗    ██████╗ ██╗      ██████╗  ██████╗██╗  ██╗ ██████╗██╗  ██╗ █████╗ ██╗███╗   ██╗
██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██╔══██╗██║     ██╔═══██╗██╔════╝██║ ██╔╝██╔════╝██║  ██║██╔══██╗██║████╗  ██║
██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ██████╔╝██║     ██║   ██║██║     █████╔╝ ██║     ███████║███████║██║██╔██╗ ██║
██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██╔══██╗██║     ██║   ██║██║     ██╔═██╗ ██║     ██╔══██║██╔══██║██║██║╚██╗██║
██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ██████╔╝███████╗╚██████╔╝╚██████╗██║  ██╗╚██████╗██║  ██║██║  ██║██║██║ ╚████║
╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝    ╚═════╝ ╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝
"#,
    )?;

    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };
    start(ui, config)
}

/// Load configuration of the node from the
/// --config parameter is present, if the file can't be loaded send an error.
/// --config parameter is not present, then load the default_config file from
///                   $RIOOS_HOME/config/blockchain.toml (or)
///                    load the built in defaults.
fn config_from_args(args: &clap::ArgMatches) -> Result<Config> {
    Ok(match args.value_of("config") {
        Some(cfg_path) => {
            try!(NodeInternalConfig::from_file(cfg_path).and_then(|n| Ok(Config { node: n })))
        }
        None => {
            let mut default_config = Config::default();

            match NodeInternalConfig::from_file(CFG_DEFAULT_FILE.to_str().unwrap()) {
                Ok(conf) => Config { node: conf },
                Err(err) => {                    
                    default_config 
                }
            }

            /*NodeInternalConfig::from_file(CFG_DEFAULT_FILE.to_str().unwrap())
                .and_then(|n| Ok(Config { node: n }))
                .unwrap_or(default_config) // panic shouldn't happen*/
        }
    })
}

/// Starts the aran-blockchain server.
/// # Failures
/// * Fails if the postgresql dbr fails to be found - cannot bind to the port, etc.
fn start(ui: &mut UI, config: Config) -> Result<()> {
    server::run(ui, config)
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
