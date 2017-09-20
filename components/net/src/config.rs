// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub const DEFAULT_ROUTER_LISTEN_PORT: u16 = 5562;
pub const DEFAULT_ROUTER_HEARTBEAT_PORT: u16 = 5563;

/// URL to GitHub API endpoint
pub const DEFAULT_GITHUB_URL: &'static str = "https://api.github.com";
/// Default Client ID for providing a default value in development environments only. This is
/// associated to the habitat-sh GitHub account and is configured to re-direct and point to a local
/// builder-api.
///
pub const DEFAULT_PROMETHEUS_URL: &'static str = "https://api.github.com";

/// See https://github.com/settings/connections/applications/0c2f738a7d0bd300de10
pub const DEV_GITHUB_CLIENT_ID: &'static str = "0c2f738a7d0bd300de10";
/// Default Client Secret for development purposes only. See the `DEV_GITHUB_CLIENT_ID` for
/// additional comments.
pub const DEV_GITHUB_CLIENT_SECRET: &'static str = "438223113eeb6e7edf2d2f91a232b72de72b9bdf";


pub trait PasswordAuth {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasswordCfg {
    /// URL to GitHub API
    pub url: String,
    /// Client identifier used for GitHub API requests
    pub client_id: String,
    /// Client secret used for GitHub API requests
    pub client_secret: String,
}

impl Default for PasswordCfg {
    fn default() -> Self {
        PasswordCfg {
            url: DEFAULT_GITHUB_URL.to_string(),
            client_id: DEV_GITHUB_CLIENT_ID.to_string(),
            client_secret: DEV_GITHUB_CLIENT_SECRET.to_string(),
        }
    }
}

//Configuration structure for shield auth
pub trait ShieldAuth {
    fn github_url(&self) -> &str;
    fn github_client_id(&self) -> &str;
    fn github_client_secret(&self) -> &str;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShieldCfg {
    /// URL to GitHub API
    pub url: String,
    /// Client identifier used for GitHub API requests
    pub client_id: String,
    /// Client secret used for GitHub API requests
    pub client_secret: String,
}

impl Default for ShieldCfg {
    fn default() -> Self {
        ShieldCfg {
            url: DEFAULT_GITHUB_URL.to_string(),
            client_id: DEV_GITHUB_CLIENT_ID.to_string(),
            client_secret: DEV_GITHUB_CLIENT_SECRET.to_string(),
        }
    }
}

pub trait Prometheus {
    fn prometheus_url(&self) -> &str;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PrometheusCfg {
    /// URL to Prometheus API
    pub url: String,
}

impl Default for PrometheusCfg {
    fn default() -> Self {
        PrometheusCfg { url: DEFAULT_PROMETHEUS_URL.to_string() }
    }
}

/// Configuration structure for connecting to a Router
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct RouterAddr {
    /// Listening address of command and heartbeat socket
    pub host: IpAddr,
    /// Listening port of command socket
    pub port: u16,
    /// Listening port of heartbeat socket
    pub heartbeat: u16,
}

impl Default for RouterAddr {
    fn default() -> Self {
        RouterAddr {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: DEFAULT_ROUTER_LISTEN_PORT,
            heartbeat: DEFAULT_ROUTER_HEARTBEAT_PORT,
        }
    }
}

impl fmt::Display for RouterAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Apply to server configurations which connect to a cluster of Routers
pub trait RouterCfg {
    /// Return a list of router addresses
    fn route_addrs(&self) -> &Vec<RouterAddr>;
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

impl ToAddrString for RouterAddr {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.host, self.port)
    }
}
