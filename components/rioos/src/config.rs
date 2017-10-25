// Copyright (c) 2017 RioCorp Inc.
//

#[macro_use]
extern crate lazy_static;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use rioos_core::config::ConfigFile;
use rioos_core::fs::rioconfig_etc_path;
use rioos_core::fs::am_i_root;
use toml;
use error::{Error, Result};

lazy_static! {
    static  ref CLICFG_DEFAULT_FILE: PathBuf =  PathBuf::from(&*rioconfig_etc_path(None).join("cli.toml").to_str().unwrap());
}

//used if the user is not root. eg: /home/rajthilak/rioos/etc/cli.toml
const CLI_CONFIG_PATH: &'static str = "rioos/etc/cli.toml";


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub api_server: Option<String>,
    pub auth_token: Option<String>,
    pub origin: Option<String>,
}

impl ConfigFile for Config {
    type Error = Error;
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_server: Some("localhost:9046".to_string()),
            auth_token: None,
            origin: None,
        }
    }
}

pub fn load() -> Result<Config> {
    let cli_config_path = cli_config_path();
    if cli_config_path.exists() {
        debug!("Loading CLI config from {}", cli_config_path.display());
        Ok(Config::from_file(&cli_config_path)?)
    } else {
        debug!("No CLI config found, loading defaults");
        Ok(Config::default())
    }
}

pub fn save(config: &Config) -> Result<()> {
    let config_path = cli_config_path();
    let parent_path = match config_path.parent() {
        Some(p) => p,
        None => {
            return Err(Error::FileNotFound(
                config_path.to_string_lossy().into_owned(),
            ))
        }
    };
    fs::create_dir_all(&parent_path)?;
    let raw = toml::ser::to_string(config)?;
    debug!("Raw config toml:\n---\n{}\n---", &raw);
    let mut file = File::create(&config_path)?;
    file.write_all(raw.as_bytes())?;
    Ok(())
}


fn cli_config_path() -> PathBuf {
    if !am_i_root() {
        if let Some(home) = env::home_dir() {
            return home.join(format!(".{}", CLI_CONFIG_PATH));
        }
    }
    PathBuf::from(CLICFG_DEFAULT_FILE.to_str().unwrap())
}
