// Copyright (c) 2017 RioCorp Inc.

//! ~~~~ This where everything starts: main starting point of the Rio/OS Aran server.

#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]

extern crate lazy_static;
extern crate rioos_aran_api as api;
extern crate rioos_core as rio_core;
extern crate rioos_common as common;

#[macro_use]
extern crate log;

use std::str::FromStr;
use std::path::PathBuf;

use rio_core::config::ConfigFile;
use rio_core::env as renv;
use rio_core::crypto::{init, default_cache_key_path};
use rio_core::fs::cache_config_path;
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};

use api::{command, Config, Error, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

lazy_static! {
    /// The default filesystem root path to base all commands from. This is lazily generated on
    /// first call and reflects on the presence and value of the environment variable keyed as
    /// `FS_ROOT_ENVVAR`.
    static  ref CFG_DEFAULT_FILE: PathBuf =  PathBuf::from(&*cache_config_path(None).join("api.toml").to_str().unwrap());
    static  ref SERVING_TLS_PFX:  PathBuf =  PathBuf::from(&*cache_config_path(None).join("serving-rioos-apiserver.pfx").to_str().unwrap());
}

fn main() {
    env_logger::init().unwrap();
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
            (@arg port: --port +takes_value "Listen port. [default: 9636]")

        )
        /*
        TO-DO: Don't remove this code. We don't have ability in openssl crate today to
        sign certificates using CSR.
        For now we'll use the ./tools/localup.sh script
        (@subcommand setup =>
            (about: "Setup the api server")
        )*/

    )
}

fn exec_subcommand_if_called(ui: &mut UI, app_matches: &clap::ArgMatches) -> Result<()> {
    debug!("CLI matches: {:?}", app_matches);

    match app_matches.subcommand() {
        ("start", Some(m)) => sub_start_server(ui, m)?,
        ("setup", Some(_)) => sub_cli_setup(ui)?,
        _ => unreachable!(),
    };
    Ok(())
}


fn sub_cli_setup(ui: &mut UI) -> Result<()> {
    init();

    command::cli::setup::start(ui, &default_cache_key_path(None))
}

fn sub_start_server(ui: &mut UI, matches: &clap::ArgMatches) -> Result<()> {
    ui.begin(
        r#"
    ██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗     █████╗ ██████╗  █████╗ ███╗   ██╗
    ██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██╔══██╗██╔══██╗██╔══██╗████╗  ██║
    ██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ███████║██████╔╝███████║██╔██╗ ██║
    ██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██╔══██║██╔══██╗██╔══██║██║╚██╗██║
    ██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ██║  ██║██║  ██║██║  ██║██║ ╚████║
    "#,
    )?;

    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };
    start(ui, config)
}

///
///
fn config_from_args(args: &clap::ArgMatches) -> Result<Config> {
    let mut config = match args.value_of("config") {
        Some(cfg_path) => try!(Config::from_file(cfg_path)),
        None => {
            /// Override with the default tls config if the pkcs12 file named
            /// serving-rioos-api-server.pfx exists
            let mut default_config = Config::default();

            if let Some(identity_pkcs12_file) = SERVING_TLS_PFX.to_str() {
                if SERVING_TLS_PFX.exists() {
                    default_config.http.port = 7443;
                    default_config.http.tls_pkcs12_file = Some(identity_pkcs12_file.to_string());
                }
            };

            Config::from_file(CFG_DEFAULT_FILE.to_str().unwrap()).unwrap_or(default_config)
        }
    };

    if let Some(port) = args.value_of("port") {
        if u16::from_str(port).map(|p| config.http.port = p).is_err() {
            return Err(Error::BadPort(port.to_string()));
        }
    }

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
