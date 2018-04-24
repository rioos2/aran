// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS API service

use rio_net::config::{AuditBackend, Blockchain, Marketplaces, PasswordAuth, SecureBackend, SecurerAuth, SystemAuth};
use rio_net::config::{BlockchainCfg, HttpCfg, MarketplacesCfg, SecurerCfg};

use entitlement::config::{License, LicensesCfg};
use telemetry::config::{Telemetry, TelemetryCfg};
use audit::config::{Anchore, AnchoreCfg, Influx, LogsCfg};

use rio_core::config::ConfigFile;

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub http: HttpCfg,
    //  Console user interface
    pub ui: UiCfg,
    //  Where to pull and record metrics
    pub prometheus: TelemetryCfg,
    //  Where to store the hidden treasures
    pub vaults: SecurerCfg,
    //  What information to use for creating services
    pub services: ServicesCfg,
    //  The information needed to load the license
    pub licenses: LicensesCfg,
    //  The information for posting in a separate logs db (influx)
    //  TO-DO: This will be moved to blockchain (rocksdb) as doing analytics will be easy.
    pub logs: LogsCfg,
    //  Blockchain API configuration.
    pub blockchain: BlockchainCfg,
    //  Marketplaces API configuration
    pub marketplaces: MarketplacesCfg,
    //  Security and vulnerabilty checker API
    pub anchore: AnchoreCfg,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            http: HttpCfg::default(),
            ui: UiCfg::default(),
            prometheus: TelemetryCfg::default(),
            vaults: SecurerCfg::default(),
            services: ServicesCfg::default(),
            licenses: LicensesCfg::default(),
            logs: LogsCfg::default(),
            blockchain: BlockchainCfg::default(),
            marketplaces: MarketplacesCfg::default(),
            anchore: AnchoreCfg::default(),
        }
    }
}

/// ConfigFile loader
impl ConfigFile for Config {
    type Error = Error;
}

/// Path to UI files to host over HTTP. If not set the UI will be disabled.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct UiCfg {
    pub url: Option<String>,
    pub root: Option<String>,
}

//A delegate, that returns the metrics (prometheus) config from the loaded prometheus config
impl Telemetry for Config {
    fn endpoint(&self) -> &str {
        &self.prometheus.url
    }
}

//A delegate, that returns the securer auth config from the loaded securer auth config
impl SecurerAuth for Config {
    fn backend(&self) -> SecureBackend {
        self.vaults.backend.clone()
    }
    fn endpoint(&self) -> &str {
        &self.vaults.endpoint
    }
    fn token(&self) -> &str {
        &self.vaults.token
    }
}

//Returns the stub services config
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServicesCfg {
    pub loadbalancer_imagein: String,
    pub loadbalancer_imagename: String,
    pub loadbalancer_cpu: String,
    pub loadbalancer_mem: String,
    pub loadbalancer_disk: String,
}

impl Default for ServicesCfg {
    fn default() -> Self {
        ServicesCfg {
            loadbalancer_imagein: "container".to_string(),
            loadbalancer_imagename: "registry.rioos.xyz:5000/rioos/loadbalancer".to_string(),
            loadbalancer_cpu: "1".to_string(),
            loadbalancer_mem: "1024 MiB".to_string(),
            loadbalancer_disk: "1 GiB".to_string(),
        }
    }
}

//A delegate, that returns the blockchain config from the loaded blockchain config
impl Blockchain for Config {
    fn backend(&self) -> AuditBackend {
        self.blockchain.backend.clone()
    }

    fn endpoint(&self) -> &str {
        &self.blockchain.endpoint
    }
    fn enabled(&self) -> bool {
        self.blockchain.enabled
    }

    fn cache_dir(&self) -> &str {
        &self.blockchain.cache_dir
    }
}

//A delegate, that returns the marketplaces config from the loaded marketplace config
impl Marketplaces for Config {
    fn endpoint(&self) -> &str {
        &self.marketplaces.endpoint
    }
    fn sync_on_startup(&self) -> bool {
        self.marketplaces.sync_on_startup
    }
    fn username(&self) -> &str {
        &self.marketplaces.username
    }
    fn token(&self) -> &str {
        &self.marketplaces.token
    }
    fn cache_dir(&self) -> &str {
        &self.marketplaces.cache_dir
    }
}

//A delegate, that returns the influx config from the loaded influx config
impl Influx for Config {
    fn endpoint(&self) -> &str {
        &self.logs.url
    }
    fn prefix(&self) -> &str {
        &self.logs.prefix
    }
}

//A delegate, that returns the anchore config from the loaded anchore config
impl Anchore for Config {
    fn endpoint(&self) -> &str {
        &self.anchore.url
    }
    fn username(&self) -> &str {
        &self.anchore.username
    }
    fn password(&self) -> &str {
        &self.anchore.password
    }
}

//A delegate, that returns the license config from the loaded licenseconfig
impl License for Config {
    fn so_file(&self) -> &str {
        &self.licenses.so_file
    }
    fn activation_code(&self) -> Option<String> {
        self.licenses.activation_code.clone()
    }
}

///// Authentication delegate configuration.

impl PasswordAuth for Config {}

impl SystemAuth for Config {
    fn serviceaccount_public_key(&self) -> Option<String> {
        self.http.serviceaccount_public_key.clone()
    }
}

/*// Memory pool configuration parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    /// Maximum number of uncommited transactions.
    pub tx_pool_capacity: usize,
    /// Sets the maximum number of messages that can be buffered on the event loop's
    /// notification channel before a send will fail.
    pub events_pool_capacity: EventsPoolCapacity,
}

impl Default for MemoryPoolConfig {
    fn default() -> MemoryPoolConfig {
        MemoryPoolConfig {
            tx_pool_capacity: 100_000,
            events_pool_capacity: EventsPoolCapacity::default(),
        }
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_file() {
        let content = r#"
        [http]
        listen = "0:0:0:0:0:0:0:1"
        port = 7443

        [ui]
        root = "/some/path"

        [[targets]]
        platform = "windows"
        architecture = "x86_64"
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.http.port, 7443);
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
