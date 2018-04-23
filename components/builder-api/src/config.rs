// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS API server

use api::audit::config::AuditBackend;
use audit::config::{Logs, LogsCfg, Vulnerability, VulnerabilityCfg};

use api::audit::config::{Blockchain, BlockchainCfg, Marketplaces, MarketplacesCfg};
use api::security::config::{SecureBackend, SecurerAuth, SecurerCfg};
use api::deploy::config::ServicesCfg;

use auth::config::{Identity, IdentityCfg};
use watch::config::{Streamer, StreamerCfg};
use entitlement::config::{License, LicensesCfg};
use telemetry::config::{Telemetry, TelemetryCfg};
use http_gateway::config::base::AuthenticationFlowCfg;

use error::Error;

use http_gateway::config::prelude::*;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    //  The base listener configuration for https
    pub https: HttpsCfg,
    //  The streamer (http2, websocket) configuration
    pub http2: StreamerCfg,
    //  The serving directory for files shown in browser.
    pub ui: UiCfg,
    //  Where to pull and record metrics
    pub telemetry: TelemetryCfg,
    //  The type of security to use. service_account to use.
    pub identity: IdentityCfg,
    //  Where to store the hidden treasures
    pub vaults: SecurerCfg,
    //  What information to use for creating services
    pub services: ServicesCfg,
    //  The information needed to load the license
    pub licenses: LicensesCfg,
    //  TO-DO: This will be removed as logs will be sent to blockchain (rocksdb) for doing analytics
    pub logs: LogsCfg,
    //  Blockchain API configuration.
    pub blockchain: BlockchainCfg,
    //  Marketplaces API configuration
    pub marketplaces: MarketplacesCfg,
    //  Security and vulnerabilty checker API
    pub vulnerability: VulnerabilityCfg,
}

// Set all the defaults fo the config
impl Default for Config {
    fn default() -> Self {
        Config {
            https: HttpsCfg::default(),
            http2: StreamerCfg::default(),
            ui: UiCfg::default(),
            telemetry: TelemetryCfg::default(),
            identity: IdentityCfg::default(),
            vaults: SecurerCfg::default(),
            services: ServicesCfg::default(),
            licenses: LicensesCfg::default(),
            logs: LogsCfg::default(),
            blockchain: BlockchainCfg::default(),
            marketplaces: MarketplacesCfg::default(),
            vulnerability: VulnerabilityCfg::default(),
        }
    }
}

/// A trait
impl AuthenticationFlowCfg for Config {
    //
    fn modes(&self) -> Vec<(String, String)> {
        vec![]
    }

    fn ready(&self) -> bool {
        false
    }

    fn unready_message(&self) -> Option<String> {
        None
    }
}

/// ConfigFile loader
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

    fn tls(&self) -> Option<String> {
        self.https.tls.clone()
    }

    fn tls_password(&self) -> Option<String> {
        self.https.tls_password.clone()
    }
}

/// Streamer configuration for Watcher
impl Streamer for Config {
    fn http2_port(&self) -> u16 {
        self.http2.port
    }

    fn websocket_port(&self) -> u16 {
        self.http2.websocket
    }

    fn tls(&self) -> Option<String> {
        self.http2.tls.clone()
    }

    fn tls_password(&self) -> Option<String> {
        self.http2.tls_password.clone()
    }
}

//A delegate, that returns the metrics (prometheus) config from the loaded prometheus config
impl Telemetry for Config {
    fn endpoint(&self) -> &str {
        &self.telemetry.endpoint
    }
}

//A delegate, that returns the identity the loaded identity config
impl Identity for Config {
    fn service_account(&self) -> Option<String> {
        self.identity.service_account.clone()
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
impl Logs for Config {
    fn influx_endpoint(&self) -> &str {
        &self.logs.influx_endpoint
    }
    fn influx_prefix(&self) -> &str {
        &self.logs.influx_prefix
    }
}

//A delegate, that returns the vulnerability provider
// Supported providers are anchore
impl Vulnerability for Config {
    fn anchore_endpoint(&self) -> &str {
        &self.vulnerability.anchore_endpoint
    }
    fn anchore_username(&self) -> &str {
        &self.vulnerability.anchore_username
    }
    fn anchore_password(&self) -> &str {
        &self.vulnerability.anchore_password
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

/*
TO-DO: 
Use the bleow configuration for the events channel
Rename the tx_pool_capacity to events_pool_capacity

// Memory pool configuration parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryPoolConfig {
    /// Maximum number of uncommited transactions.
    pub     tx_pool_capacity: usize,
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
        [https]
        listen = "0.0.0.0"
        port = 7443        
        tls = "api-server.pfx"
        tls_password = "TEAMRIOADVANCEMENT123"

        [http2]
        port      = 8443
        websocket = 9443        
        tls = "api-server.pfx"
        tls_password = "TEAMRIOADVANCEMENT123"

        [identity]
        service_account = "service-account.pub"

        [marketplaces]
        username = "rioosdolphin@rio.company"
        token = "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R"
        endpoint = "https://marketplaces.rioos.xyz:6443/api/v1"

        [blockchain]
        endpoint = "http://localhost:7000"

        [ui]
        root = "/public"

        [telemetry]
        endpoint = "http://localhost:9090/api/v1"

        [vaults]
        backend = "Local"

        [licenses]
        so_file = "ShaferFilechck.so"
        activation_code = ""

        [logs]
        endpoint = "http://localhost:8086"
        prefix = "rioos_logs"

        [vulnerability]
        anchore_endpoint = "http://localhost:8228/v1"
        anchore_username = ""
        anchore_password = ""       
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.https.port, 7443);

        assert_eq!(config.https.tls, "api-server.pfx");
        assert_eq!(config.https.tls_password, "TEAMRIOADVANCEMENT123");

        assert_eq!(config.http2.port, 8443);
        assert_eq!(config.http2.websocker, 9443);

        assert_eq!(config.marketplaces.username, "rioosdolphin@rio.company");
        assert_eq!(
            config.marketplaces.token,
            "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R"
        );
        assert_eq!(
            config.marketplaces.endpoint,
            "https://marketplces.rioos.xyz:6443/api/v1"
        );

        assert_eq!(config.blockchain.endpoint, "http://localhost:7000");

        assert_eq!(config.ui.root, "/public");

        assert_eq!(config.telemetry.endpoint, "http://localhost:9090/api/v1");

        assert_eq!(config.identity.serviceaccount, "service-account.pub");

        assert_eq!(config.vaults.backend, "Local");

        assert_eq!(config.licenses.so_file, "ShaferFilechck.so");
        assert_eq!(config.licenses.activation_code, "");

        assert_eq!(config.logs.endpoint, "http://localhost:8086");
        assert_eq!(config.logs.prefix, "rioos_logs");

        assert_eq!(
            config.vulnerability.anchore_endpoint,
            "http://localhost:8086"
        );
        assert_eq!(config.vulnerability.anchore_username, "");
        assert_eq!(config.vulnerability.anchore_password, "");
    }

    #[test]
    fn config_from_file() {
        let content = r#"        
        [https]
        listen = "0.0.0.0"
        port = 7443        

        [marketplaces]
        username = "rioosdolphin@rio.company"
        token = "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R"
        endpoint = "https://marketplaces.rioos.xyz:6443/api/v1"            
        "#;

        let config = Config::from_raw(&content).unwrap();
        assert_eq!(&format!("{}", config.http.listen), "::1");
        assert_eq!(config.https.port, 7443);

        assert_eq!(config.https.tls, "api-server.pfx");
        assert_eq!(config.https.tls_password, "TEAMRIOADVANCEMENT123");

        assert_eq!(config.http2.port, 8443);
        assert_eq!(config.http2.websocket, 9443);
        assert_eq!(config.http2.tls, "api-server.pfx");
        assert_eq!(config.http2.tls_password, "TEAMRIOADVANCEMENT123");

        assert_eq!(config.marketplaces.username, "rioosdolphin@rio.company");
        assert_eq!(
            config.marketplaces.token,
            "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R"
        );
        assert_eq!(
            config.marketplaces.endpoint,
            "https://marketplces.rioos.xyz:6443/api/v1"
        );

        assert_eq!(config.blockchain.endpoint, "http://localhost:7000");

        assert_eq!(config.ui.root, "/public");

        assert_eq!(config.telemetry.endpoint, "http://localhost:9090/api/v1");

        assert_eq!(config.identity.service_account, "service-account.pub");

        assert_eq!(config.vaults.backend, "Local");

        assert_eq!(config.licenses.so_file, "ShaferFilechck.so");
        assert_eq!(config.licenses.activation_code, "");

        assert_eq!(config.logs.endpoint, "http://localhost:8086");
        assert_eq!(config.logs.prefix, "rioos_logs");

        assert_eq!(
            config.vulnerability.anchore_endpoint,
            "http://localhost:8086"
        );
        assert_eq!(config.vulnerability.anchore_username, "");
        assert_eq!(config.vulnerability.anchore_password, "");
    }

}
