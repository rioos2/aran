// Copyright 2018 The Rio Advancement Inc
//
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::net::ToSocketAddrs;
use std::io;
use std::option::IntoIter;


/// host url  to get the audits
pub const DEFAULT_BLOCK_CHAIN_URL: &'static str = "http://localhost:7000";

/// host url  to get the rio marketplace
pub const DEFAULT_RIO_MARKETPLACES_URL: &'static str = "https://localhost:6443/api/v1";
/// a default username for marketplace
pub const DEV_RIO_COMPANY: &'static str = "dev@rio.companyadmin";
/// a default token for the marketplace
pub const TOKEN: &'static str = "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R";


///// Configuration for Secure vault.

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SecureBackend {
    Local,
    EnvKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurerCfg {
    pub backend: SecureBackend,
    pub endpoint: String,
    pub token: String,
}

impl Default for SecurerCfg {
    fn default() -> Self {
        SecurerCfg {
            backend: SecureBackend::Local,
            endpoint: "".to_string(),
            token: "".to_string(),
        }
    }
}

pub trait SecurerAuth {
    fn backend(&self) -> SecureBackend;
    fn endpoint(&self) -> &str;
    fn token(&self) -> &str;
}

/// Trait that feeds the configuration into the APIWirers.
/// This trait feed the configuration into the PasswordClient (via PasswordCLI)
pub trait PasswordAuth {}

/// This trait feed the service account public key credential configuration into the
/// Authenticated (Authenticated is invoked by all APIs (from APIWirers)
pub trait SystemAuth {
    fn serviceaccount_public_key(&self) -> Option<String>;
}


/// Public listening net address for HTTP requests, Watch requests.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpCfg {
    pub listen: IpAddr, // The listen ip address for http api/watch http2 api
    pub port: u16, // The http api server port
    pub watch_port: u16, // The http2 watch server port
    pub websocket_port: u16, // The websocket server port
    /// This file is used by both http api/watch server.
    pub tls_pkcs12_file: Option<String>, // The tls_pkcs12 is the pfx file that is used as security to start the server.
    pub tls_pkcs12_pwd: Option<String>, // The tls_pkcs12_pwd  is the pfx file password.
    pub serviceaccount_public_key: Option<String>,
}

impl Default for HttpCfg {
    fn default() -> Self {
        HttpCfg {
            listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 7443,
            watch_port: 8443,
            websocket_port: 9443,
            tls_pkcs12_file: None,
            tls_pkcs12_pwd: None,
            serviceaccount_public_key: None,
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

///// Configuration for Audits (blockchain)

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AuditBackend {
    Exonum,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BlockchainCfg {
    pub backend: AuditBackend,
    pub endpoint: String,
    pub enabled: bool,
    pub cache_dir: String,
}

impl Default for BlockchainCfg {
    fn default() -> Self {
        BlockchainCfg {
            backend: AuditBackend::Exonum,
            endpoint: DEFAULT_BLOCK_CHAIN_URL.to_string(),
            enabled: true,
            cache_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

pub trait Blockchain {
    fn backend(&self) -> AuditBackend;
    fn endpoint(&self) -> &str;
    fn enabled(&self) -> bool;
    fn cache_dir(&self) -> &str;
}

///// Configuration for rio marketplace

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MarketplacesCfg {
    pub endpoint: String,
    pub sync_on_startup: bool,
    pub username: String,
    pub token: String,
    pub cache_dir: String,
}

impl Default for MarketplacesCfg {
    fn default() -> Self {
        MarketplacesCfg {
            endpoint: DEFAULT_RIO_MARKETPLACES_URL.to_string(),
            sync_on_startup: false,
            username: DEV_RIO_COMPANY.to_string(),
            token: TOKEN.to_string(),
            cache_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

pub trait Marketplaces {
    fn endpoint(&self) -> &str;
    fn sync_on_startup(&self) -> bool;
    fn username(&self) -> &str;
    fn token(&self) -> &str;
    fn cache_dir(&self) -> &str;
}




/// Apply to a server configuration which belongs to a sharded service
pub trait Shards {
    fn shards(&self) -> &Vec<u32>;
}

/// Convert types into stringy socket addresses for ZeroMQ
pub trait ToAddrString {
    fn to_addr_string(&self) -> String;
}

impl ToAddrString for SocketAddr {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

impl ToAddrString for SocketAddrV4 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

impl ToAddrString for SocketAddrV6 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}
