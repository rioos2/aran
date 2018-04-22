use std::env;

///host url to check the vulnerability of the container
pub const DEFAULT_ANCHORE_URL: &'static str = "http://localhost:8228/v1";
/// host url  to get the audits
pub const DEFAULT_BLOCK_CHAIN_URL: &'static str = "http://localhost:7000";
/// Default Influx Host url to access the log of virtual machine and container
pub const DEFAULT_LOGS_URL: &'static str = "http://localhost:8086";
/// host url  to get the rio marketplace
pub const DEFAULT_RIO_MARKETPLACES_URL: &'static str = "https://localhost:6443/api/v1";
/// a default username for marketplace
pub const DEV_RIO_COMPANY: &'static str = "dev@rio.companyadmin";
/// a default token for the marketplace
pub const TOKEN: &'static str = "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R";
/// a default username for anchore or anybody else who wish to use the name admin
pub const DEFAULT_USERNAME_ADMIN: &'static str = "admin";

///// Configuration for Logs

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct LogsCfg {
    pub url: String,
    pub prefix: String,
}

impl Default for LogsCfg {
    fn default() -> Self {
        LogsCfg {
            url: DEFAULT_LOGS_URL.to_string(),
            prefix: "rioos_logs".to_string(),
        }
    }
}

pub trait Logs {
    /// URL to Influx API
    fn endpoint(&self) -> &str;
    /// Includes the prefix of the database,table,path in influx
    fn prefix(&self) -> &str;
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

///// Configuration for security vulnerability

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct VulnerabilityCfg {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Default for VulnerabilityCfg {
    fn default() -> Self {
        VulnerabilityCfg {
            anchore_endpoint: DEFAULT_ANCHORE_URL.to_string(),
            anchore_username: DEFAULT_USERNAME_ADMIN.to_string(),
            anchore_password: DEFAULT_USERNAME_ADMIN.to_string(),
        }
    }
}

pub trait Vulnerability {
    fn anchore_endpoint(&self) -> &str;
    fn anchore_username(&self) -> &str;
    fn anchore_password(&self) -> &str;
}



#[derive(Clone, Debug)]
pub struct BlockchainConn {
    pub backend: AuditBackend,
    pub url: String,
}

#[allow(unused_variables)]
impl BlockchainConn {
    pub fn new<T: Blockchain>(config: &T) -> Self {
        BlockchainConn {
            backend: config.backend(),
            url: config.endpoint().to_string(),
        }
    }
}
