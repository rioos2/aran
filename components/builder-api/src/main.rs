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
use rio_core::crypto::{init, default_rioconfig_key_path};
use rio_core::fs::rioconfig_config_path;
use common::ui::{Coloring, UI, NOCOLORING_ENVVAR, NONINTERACTIVE_ENVVAR};

use api::{command, Config, Error, Result};
use api::node::Servers;

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

lazy_static! {
    static  ref CFG_DEFAULT_FILE: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("api.toml").to_str().unwrap());
    static  ref SERVING_TLS_PFX:  PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("api-server.pfx").to_str().unwrap());
    static  ref SERVICEACCOUNT_PUBLIC_KEY:  PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("service-account.pub").to_str().unwrap());
    static  ref SETUP_COMPLETE_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join(".rioos_setup_complete").to_str().unwrap());
    static  ref MARKETPLACE_CACHE_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join("pullcache/marketplaces.yaml").to_str().unwrap());
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
            (@arg port: --port +takes_value "Listen port. [default: 7443]")
            (@arg watch_port: --watch_port +takes_value "Listen watch port. [default: 8443]")
            (@arg uiwatch_port: --watch_port +takes_value "Listen uiwatch port. [default: 9443]")
            (@arg watcher: --watcher +takes_value "Start Watch server. [default: false]")
            (@arg uiwatcher: --uiwatcher +takes_value "Start UIWatch server. [default: false]")

        )
        //For now we'll use the ./tools/localup.sh script
        (@subcommand setup =>
            (about: "Setup the api server")
        )

        (@subcommand sync =>
            (about: "Sync Rio.Marketplaces with api server")
        )

    )
}

fn exec_subcommand_if_called(ui: &mut UI, app_matches: &clap::ArgMatches) -> Result<()> {
    debug!("CLI matches: {:?}", app_matches);

    match app_matches.subcommand() {
        ("start", Some(m)) => sub_start_server(ui, m)?,
        ("setup", Some(m)) => sub_cli_setup(ui, m)?,
        ("sync", Some(m)) => sub_cli_sync(ui, m)?,

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

    //set which server to be start from command args
    let mut server = Servers::APISERVER;
    
    if watcher_from_args(&matches) {
        ui.begin(
        r#"
██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗    ██╗    ██╗ █████╗ ████████╗ ██████╗██╗  ██╗███████╗██████╗ 
██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██║    ██║██╔══██╗╚══██╔══╝██╔════╝██║  ██║██╔════╝██╔══██╗
██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ██║ █╗ ██║███████║   ██║   ██║     ███████║█████╗  ██████╔╝
██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██║███╗██║██╔══██║   ██║   ██║     ██╔══██║██╔══╝  ██╔══██╗
██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ╚███╔███╔╝██║  ██║   ██║   ╚██████╗██║  ██║███████╗██║  ██║
╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝     ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝    ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝                                                                                                                 
    "#,
    )?;
        server = Servers::WATCHER
    } else if uiwatcher_from_args(&matches) {
        ui.begin(
        r#"
██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗    ██╗   ██╗██╗██╗    ██╗ █████╗ ████████╗ ██████╗██╗  ██╗███████╗██████╗ 
██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██║   ██║██║██║    ██║██╔══██╗╚══██╔══╝██╔════╝██║  ██║██╔════╝██╔══██╗
██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ██║   ██║██║██║ █╗ ██║███████║   ██║   ██║     ███████║█████╗  ██████╔╝
██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██║   ██║██║██║███╗██║██╔══██║   ██║   ██║     ██╔══██║██╔══╝  ██╔══██╗
██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ╚██████╔╝██║╚███╔███╔╝██║  ██║   ██║   ╚██████╗██║  ██║███████╗██║  ██║
╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝     ╚═════╝ ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝    ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝                                                                                                             
   "#,
    )?;
        server = Servers::UIWATCHER
    } else {
        ui.begin(
        r#"
    ██████╗ ██╗ ██████╗     ██╗ ██████╗ ███████╗     █████╗ ██████╗  █████╗ ███╗   ██╗     █████╗ ██████╗ ██╗
    ██╔══██╗██║██╔═══██╗   ██╔╝██╔═══██╗██╔════╝    ██╔══██╗██╔══██╗██╔══██╗████╗  ██║    ██╔══██╗██╔══██╗██║
    ██████╔╝██║██║   ██║  ██╔╝ ██║   ██║███████╗    ███████║██████╔╝███████║██╔██╗ ██║    ███████║██████╔╝██║
    ██╔══██╗██║██║   ██║ ██╔╝  ██║   ██║╚════██║    ██╔══██║██╔══██╗██╔══██║██║╚██╗██║    ██╔══██║██╔═══╝ ██║
    ██║  ██║██║╚██████╔╝██╔╝   ╚██████╔╝███████║    ██║  ██║██║  ██║██║  ██║██║ ╚████║    ██║  ██║██║     ██║
    ╚═╝  ╚═╝╚═╝ ╚═════╝ ╚═╝     ╚═════╝ ╚══════╝    ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝    ╚═╝  ╚═╝╚═╝     ╚═╝                                                                                                        
    "#,
    )?;
        server = Servers::APISERVER
    }
    start(ui, config, server)
}

fn watcher_from_args(args: &clap::ArgMatches) -> bool {
    match args.value_of("watcher") {
        Some(flag) => bool::from_str(flag).unwrap(),
        None => false,
    }
}

fn uiwatcher_from_args(args: &clap::ArgMatches) -> bool {
    match args.value_of("uiwatcher") {
        Some(flag) => bool::from_str(flag).unwrap(),
        None => false,
    }
}

///
///
fn config_from_args(args: &clap::ArgMatches) -> Result<Config> {
    let mut config = match args.value_of("config") {
        Some(cfg_path) => try!(Config::from_file(cfg_path)),
        None => {
            let mut default_config = Config::default();

            if let Some(identity_pkcs12_file) = SERVING_TLS_PFX.to_str() {
                if SERVING_TLS_PFX.exists() {
                    default_config.http.port = 7443;
                    default_config.http.watch_port = 8443;
                    default_config.http.tls_pkcs12_file = Some(identity_pkcs12_file.to_string());
                }
            };

            if let Some(serviceaccount_public_key) = SERVICEACCOUNT_PUBLIC_KEY.to_str() {
                if SERVICEACCOUNT_PUBLIC_KEY.exists() {
                    default_config.http.serviceaccount_public_key = Some(serviceaccount_public_key.to_string());
                }
            }

            Config::from_file(CFG_DEFAULT_FILE.to_str().unwrap()).unwrap_or(default_config)
        }
    };

    if config.http.serviceaccount_public_key.is_none() {
        return Err(Error::MissingTLS("service account pub".to_string()));
    }

    if config.http.tls_pkcs12_file.is_none() {
        return Err(Error::MissingTLS("api server pfx".to_string()));
    }

    if let Some(port) = args.value_of("port") {
        if u16::from_str(port).map(|p| config.http.port = p).is_err() {
            return Err(Error::BadPort(port.to_string()));
        }
    }

    if let Some(watch_port) = args.value_of("watch_port") {
        if u16::from_str(watch_port)
            .map(|p| config.http.watch_port = p)
            .is_err()
        {
            return Err(Error::BadPort(watch_port.to_string()));
        }
    }

    if let Some(uiwatch_port) = args.value_of("uiwatch_port") {
        if u16::from_str(iowatch_port)
            .map(|p| config.http.uiwatch_port = p)
            .is_err()
        {
            return Err(Error::BadPort(uiwatch_port.to_string()));
        }
    }

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
