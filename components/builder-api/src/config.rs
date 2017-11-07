// Copyright (c) 2017 RioCorp Inc.

//! Configuration for a Rio/OS API service

use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

use rio_net::config::{PasswordAuth, ShieldCfg, ShieldAuth, PrometheusCfg, Prometheus};
use rio_core::config::ConfigFile;

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    //  Console user interface
    pub ui: UiCfg,
    //  Where and how to shield rio
    pub shield: ShieldCfg,
    //  Where to pull and record metrics
    pub prometheus: PrometheusCfg,
    //  Where to store the hidden treasures
    pub vaults: VaultsCfg,
    //  Whether to log events for metrics
    pub events_enabled: bool,
    /// Where to record log events for metrics
    pub log_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpCfg::default(),
            ui: UiCfg::default(),
            shield: ShieldCfg::default(),
            prometheus: PrometheusCfg::default(),
            vaults: VaultsCfg::default(),
            events_enabled: false,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl PasswordAuth for Config {}


impl ShieldAuth for Config {
    fn github_url(&self) -> &str {
        &self.prometheus.url
    }

    fn github_client_id(&self) -> &str {
        &self.prometheus.url
    }

    fn github_client_secret(&self) -> &str {
        &self.prometheus.url
    }
}


impl Prometheus for Config {
    fn prometheus_url(&self) -> &str {
        &self.prometheus.url
    }
}


/// Public listening net address for HTTP requests
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr,
    pub port: u16,
    pub tls_pkcs12_file: Option<String>,
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 9636,
            tls_pkcs12_file: None,
        }
    }
}

impl ToSocketAddrs for HttpCfg {
    type Iter = IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<IntoIter<SocketAddr>> {
        match self.listen {
            IpAddr::V4(ref a) => (*a, self.port).to_socket_addrs(),
            IpAddr::V6(ref a) => (*a, self.port).to_socket_addrs(),
        }
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
