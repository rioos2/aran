// Copyright (c) 2017 RioCorp Inc.

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate habitat_builder_api as api;
extern crate habitat_core as hab_core;
#[macro_use]
extern crate log;

use std::process;
use std::str::FromStr;

use hab_core::config::ConfigFile;
use api::{Config, Error, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/var/lib/rioos/api.toml";

fn main() {
    env_logger::init().unwrap();
    let matches = app().get_matches();
    debug!("CLI matches: {:?}", matches);
    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return exit_with(e, 1),
    };
    match start(config) {
        Ok(_) => std::process::exit(0),
        Err(e) => exit_with(e, 1),
    }
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(BuilderApi =>
        (version: VERSION)
        (about: "RIO/OS builder-api")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand start =>
            (about: "Run the builder-api server")
            (@arg config: -c --config +takes_value
                "Filepath to configuration file. [default: /var/lib/rioos/api.toml]")
            (@arg port: --port +takes_value "Listen port. [default: 9636]")
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let mut config = match args.value_of("config") {
        Some(cfg_path) => try!(Config::from_file(cfg_path)),
        None => Config::from_file(CFG_DEFAULT_PATH).unwrap_or(Config::default()),
    };
    if let Some(port) = args.value_of("port") {
        if u16::from_str(port).map(|p| config.http.port = p).is_err() {
            return Err(Error::BadPort(port.to_string()));
        }
    }

    Ok(config)
}

fn exit_with(err: Error, code: i32) {
    println!("{}", err);
    process::exit(code)
}

/// Starts the builder-api server.
///
/// # Failures
///
/// * Fails if the depot server fails to start - cannot bind to the port, etc.
fn start(config: Config) -> Result<()> {
    api::server::run(config)
}
