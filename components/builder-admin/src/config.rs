// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//


//! Configuration for a Habitat Builder-Admin service

use std::io;
use std::net::{Ipv4Addr, IpAddr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

use rio_net::config::{PasswordCfg, PasswordAuth, RouterAddr, RouterCfg};
use rio_core::config::ConfigFile;

use error::Error;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    pub github: PasswordCfg,
    pub ui: UiCfg,
}

impl ConfigFile for Config {
    type Error = Error;
}

impl PasswordAuth for Config {}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr,
    pub port: u16,
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8080,
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
        port = 8080

        [ui]
        root = "/some/path"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.http.port, 8080);
        assert_eq!(config.ui.root, Some("/some/path".to_string()));
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
