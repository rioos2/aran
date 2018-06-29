use std::env;

/// host url  to get the audits
pub const DEFAULT_BLOCK_CHAIN_URL: &'static str = "http://localhost:7000";
/// Default Influx Host url to access the log of virtual machine and container
pub const DEFAULT_LOGS_URL: &'static str = "http://localhost:8086";
/// host url  to get the rio appstore
pub const DEFAULT_RIO_APPSTORE_URL: &'static str = "https://localhost:6443/api/v1";
/// a default username for rio appstore
pub const DEV_RIO_COMPANY: &'static str = "dev@rio.company";
/// a default token for the marketplace
pub const TOKEN: &'static str = "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R";

const USERNAME: &'static str = "postmaster@ojamail.megambox.com";

const PASSWORD: &'static str = "b311ed99d8d544b10ca001bd5fdbcbe1";

const SENDER: &'static str = "info@rio.company";

const DOMAIN: &'static str = "smtp.mailgun.org:587";

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

///// Configuration for rio appstore

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct AppStoresCfg {
    pub endpoint: String,
    pub sync_on_startup: bool,
    pub username: String,
    pub token: String,
    pub cache_dir: String,
}

impl Default for AppStoresCfg {
    fn default() -> Self {
        AppStoresCfg {
            endpoint: DEFAULT_RIO_APPSTORE_URL.to_string(),
            sync_on_startup: false,
            username: DEV_RIO_COMPANY.to_string(),
            token: TOKEN.to_string(),
            cache_dir: env::temp_dir().to_string_lossy().into_owned(),
        }
    }
}

pub trait AppStores {
    fn endpoint(&self) -> &str;
    fn sync_on_startup(&self) -> bool;
    fn username(&self) -> &str;
    fn token(&self) -> &str;
    fn cache_dir(&self) -> &str;
}

///// Configuration for audits blockchain

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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct MailerCfg {
    pub enabled: bool,
    pub username: String,
    pub password: String,
    pub domain: String,
    pub sender: String,
}

impl Default for MailerCfg {
    fn default() -> Self {
        MailerCfg {
            enabled: true,
            username: USERNAME.to_string(),
            password: PASSWORD.to_string(),
            domain: DOMAIN.to_string(),
            sender: SENDER.to_string(),
        }
    }
}

pub trait Mailer {
    fn enabled(&self) -> bool;
    fn username(&self) -> &str;
    fn password(&self) -> &str;
    fn domain(&self) -> &str;
    fn sender(&self) -> &str;
}

#[allow(unused_variables)]
impl MailerCfg {
    pub fn new<T: Mailer>(config: &T) -> Self {
        MailerCfg {
            enabled: config.enabled(),
            username: config.username().to_string(),
            password: config.password().to_string(),
            domain: config.domain().to_string(),
            sender: config.sender().to_string(),
        }
    }
}
