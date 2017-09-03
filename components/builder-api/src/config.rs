// Copyright (c) 2017 RioCorp Inc.

//! Configuration for a Rio/OS API service

use std::env;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::option::IntoIter;

use rio_net::config::{PasswordCfg, ShieldCfg, PasswordAuth, ShieldAuth, RouterAddr, RouterCfg};
use rio_core::config::ConfigFile;

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    /// List of net addresses for routing servers to connect to
    pub routers: Vec<RouterAddr>,
    //
    pub ui: UiCfg,
    //
    pub github: PasswordCfg,
    //RIO Shield
    pub shield: ShieldCfg,
    //
    // Whether to log events for funnel metrics
    pub events_enabled: bool,
    /// Where to record log events for funnel metrics
    pub log_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpCfg::default(),
            routers: vec![RouterAddr::default()],
            ui: UiCfg::default(),
            github: PasswordCfg::default(),
            shield: ShieldCfg::default(),
            events_enabled: false,
            log_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

impl ConfigFile for Config {
    type Error = Error;
}

impl PasswordAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}

impl ShieldAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}


impl RouterCfg for Config {
    fn route_addrs(&self) -> &Vec<RouterAddr> {
        &self.routers
    }
}

/// Public listening net address for HTTP requests
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
            port: 9636,
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

        [[routers]]
        host = "172.18.0.2"
        port = 9632
        heartbeat = 9001

        [github]
        url = "https://api.github.com"
        client_id = "0c2f738a7d0bd300de10"
        client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.http.port, 9636);
        assert_eq!(&format!("{}", config.routers[0]), "172.18.0.2:9632");
        assert_eq!(config.github.url, "https://api.github.com");
        assert_eq!(config.github.client_id, "0c2f738a7d0bd300de10");
        assert_eq!(
            config.github.client_secret,
            "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
        );
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

/*

CACertAndKeyBaseName = "ca"
CACertName           = "ca.crt"
CAKeyName            = "ca.key"

APIServerCertAndKeyBaseName = "apiserver"
APIServerCertName           = "apiserver.crt"
APIServerKeyName            = "apiserver.key"

APIServerKubeletClientCertAndKeyBaseName = "apiserver-kubelet-client"
APIServerKubeletClientCertName           = "apiserver-kubelet-client.crt"
APIServerKubeletClientKeyName            = "apiserver-kubelet-client.key"

ServiceAccountKeyBaseName    = "sa"
ServiceAccountPublicKeyName  = "sa.pub"
ServiceAccountPrivateKeyName = "sa.key"


fn generate_server_cert() -> Result<X509> {
    let (ca_cert, ca_key) = X509Generator::new().generate().unwrap();

    let (server_cert, server_key) = X509Generator::new().generate().unwrap();
    let request = server_cert.generate_signing_request().unwrap();
    let signed_server_cert = ca_cert.sign(&request, &ca_key).unwrap();
}

*/
