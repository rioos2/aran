// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio.Marketplaces API service

use std::collections::HashMap;
use std::path::PathBuf;

use auth::config::{flow_modes, AuthenticationFlowCfg, Identity, IdentityCfg};

use rio_core::config::ConfigFile;
use rio_core::crypto::keys::read_key_in_bytes;
use rio_core::fs::rioconfig_config_path;

use common::ui::UI;

use error::{Error, Result};

use http_gateway::config::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub https: HttpsCfg,
    //  Console user interface
    pub ui: UiCfg,
    //  The type of security to use. service_account to use.
    pub identity: IdentityCfg,
}

/// dump the configuration
impl Config {
    pub fn dump(&self, ui: &mut UI) -> Result<()> {
        ui.begin("Configuration")?;
        ui.heading("[https]")?;
        ui.para(
            &format!("{}:{}", self.https.listen, self.https.port),
        )?;
        ui.para(&format!(
            "{:?} {:?}",
            self.https.tls,
            self.https.tls_password
        ))?;
        ui.heading("[identity]")?;
        //ui.para(&format!("{:?}", &self.identity.enabled.into_iter().collect()))?;
        //ui.para(&format!("{:?}", &self.identity.params.into_iter().collect()))?;
        ui.end("Loaded.")?;

        Ok(())
    }

    /// Returns the a tuple for tls usage with
    /// Option<(tls file location, bytes loaded from the name in the config toml file,
    ///        tls password if present or empty string)>
    fn tlspair_as_bytes(tls: Option<String>, tls_password: Option<String>) -> TLSPair {
        tls.clone().and_then(|t| {
            read_key_in_bytes(&PathBuf::from(t.clone()))
                .map(|p| {
                    (t.clone(), p, tls_password.clone().unwrap_or("".to_string()))
                })
                .ok()
        })
    }
}

pub trait ConfigValidator {
    fn valid(&self) -> Result<()>;
}


impl ConfigValidator for Config {
    fn valid(&self) -> Result<()> {
        vec![self.https.valid()].iter().fold(
            Ok(()),
            |acc, x| match x {
                &Ok(()) => return acc,
                &Err(ref e) => {
                    if acc.is_ok() {
                        return Err(Error::MissingConfiguration(format!("{}", e)));
                    }
                    Err(Error::MissingConfiguration(
                        format!("{}\n{}", e, acc.unwrap_err()),
                    ))
                }
            },
        )
    }
}


/// Validate the presence of listener, port, and tls
impl ConfigValidator for HttpsCfg {
    fn valid(&self) -> Result<()> {
        if self.tls.is_none() {
            return Err(Error::MissingConfiguration(
                "Missing  in api.toml. [https] → tls".to_string(),
            ));
        }

        let tls_location = PathBuf::from(&*rioconfig_config_path(None)
            .join(self.tls.clone().unwrap())
            .to_str()
            .unwrap());

        if !tls_location.exists() {
            return Err(Error::MissingConfiguration(
                format!("File Not Found at {}", tls_location.display()),
            ));
        }

        Ok(())
    }
}


impl AuthenticationFlowCfg for Config {
    fn modes(&self) -> (Vec<String>, HashMap<String, String>) {
        flow_modes(self, rioconfig_config_path(None))
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            https: HttpsCfg::default(),
            identity: IdentityCfg::default(),
            ui: UiCfg::default(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

/// GatewayCfg for HttpGateway
impl GatewayCfg for Config {
    fn listen_addr(&self) -> &IpAddr {
        &self.https.listen
    }

    fn listen_port(&self) -> u16 {
        self.https.port
    }

    fn tls_pair(&self) -> TLSPair {
        Config::tlspair_as_bytes(self.tls(), self.tls_password())
    }

    fn tls(&self) -> Option<String> {
        self.https.tls.clone().map(|n| {
            (&*rioconfig_config_path(None).join(n).to_str().unwrap()).to_string()
        })
    }

    fn tls_password(&self) -> Option<String> {
        self.https.tls_password.clone()
    }
}

//A delegate, that returns the identity the loaded identity config
impl Identity for Config {
    fn enabled(&self) -> Vec<String> {
        self.identity.enabled.clone()
    }

    fn params(&self) -> HashMap<String, String> {
        self.identity.params.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        [https]
        listen = "0.0.0.0"
        port = 6443
        tls = "api-server.pfx"
        tls_password = "TEAMRIOADVANCEMENT123"

        [ui]
        root = "/some/path"

        [identity]
        enabled = ["password"]
        params = {}

        [[targets]]
        platform = "windows"
        architecture = "x86_64"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.https.listen), "::1");
        assert_eq!(config.https.port, 6443);
    }

    #[test]
    fn config_from_file_defaults() {
        let content = r#"
        [https]
        port = 9000
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(config.https.port, 9000);
    }
}
