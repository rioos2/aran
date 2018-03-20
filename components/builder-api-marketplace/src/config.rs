// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS API service

use rio_net::config::{HttpCfg, SystemAuth, PasswordAuth};
use rio_core::config::ConfigFile;

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    //  Console user interface
    pub ui: UiCfg,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpCfg::default(),
            ui: UiCfg::default(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl PasswordAuth for Config {}

/// This isn't needed for rio.marketplace. 
/// TO-DO : Remove later.
impl SystemAuth for Config {
    fn serviceaccount_public_key(&self) -> Option<String> {
        Some("".to_string())
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct UiCfg {
    /// Path to UI files to host over HTTP. If not set the UI will be disabled.
    pub root: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        [http]
        listen = "0:0:0:0:0:0:0:1"
        port = 9636

        [ui]
        root = "/some/path"

        [[targets]]
        platform = "windows"
        architecture = "x86_64"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.http.port, 9636);
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        [http]
        port = 9000
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.http.port, 9000);
    }
}
