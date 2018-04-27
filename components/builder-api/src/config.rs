// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS API server
use std::collections::HashMap;
use std::path::PathBuf;

use api::audit::config::AuditBackend;
use audit::config::{Logs, LogsCfg, Vulnerability, VulnerabilityCfg};

use api::audit::config::{Blockchain, BlockchainCfg, Marketplaces, MarketplacesCfg};
use api::security::config::{SecureBackend, SecurerAuth, SecurerCfg};
use api::deploy::config::ServicesCfg;

use auth::config::{Identity, IdentityCfg};
use watch::config::{Streamer, StreamerCfg};
use entitlement::config::{License, LicensesCfg};
use telemetry::config::{Telemetry, TelemetryCfg};

use rio_core::config::ConfigFile;
use rio_core::crypto::keys::read_key_in_bytes;
use rio_core::fs::rioconfig_config_path;

use http_gateway::config::base::AuthenticationFlowCfg;
use validator::ConfigValidator;
use common::ui::UI;

use error::{Error, Result};

use http_gateway::config::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
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
    // Controller configuration
    pub controller: ControllerCfg,
    // Scheduler configuration
    pub scheduler: SchedulerCfg,
    // Vnc Server configuration
    pub vnc: VNCCfg,
}

/// dump the configuration
impl Config {
    pub fn dump(&self, ui: &mut UI) -> Result<()> {
        ui.begin("Configuration")?;
        ui.heading("[https]")?;
        ui.para(&format!("{}:{}", self.https.listen, self.https.port))?;
        ui.para(&format!(
            "{:?} {:?}",
            self.https.tls, self.https.tls_password
        ))?;
        ui.heading("[http2]")?;
        ui.para(&format!("{}:{}", self.http2.listener, self.http2.port))?;
        ui.para(&format!(
            "{:?} {:?}",
            self.http2.tls, self.http2.tls_password
        ))?;
        ui.heading("[telemetry]")?;
        ui.para(&self.telemetry.endpoint)?;
        ui.heading("[identity]")?;
        //ui.para(&format!("{:?}", &self.identity.enabled.into_iter().collect()))?;
        //ui.para(&format!("{:?}", &self.identity.params.into_iter().collect()))?;
        ui.heading("[vaults]")?;
        ui.para(&format!("{:?}", &self.vaults.backend))?;
        ui.heading("[services]")?;
        ui.para(&self.services.loadbalancer_imagein)?;
        ui.para(&self.services.loadbalancer_imagename)?;
        ui.para(&self.services.loadbalancer_cpu)?;
        ui.para(&self.services.loadbalancer_mem)?;
        ui.para(&self.services.loadbalancer_disk)?;
        ui.heading("[licenses]")?;
        ui.para(&self.licenses.so_file)?;
        ui.heading("[logs]")?;
        ui.para(&self.logs.influx_endpoint)?;
        ui.para(&self.logs.influx_prefix)?;
        ui.heading("[blockchain]")?;
        ui.para(&self.blockchain.endpoint)?;
        ui.heading("[marketplaces]")?;
        ui.para(&self.marketplaces.endpoint)?;
        ui.para(&self.marketplaces.username)?;
        ui.para(&self.marketplaces.token)?;
        ui.heading("[vulnerability]")?;
        ui.para(&self.vulnerability.anchore_endpoint)?;
        ui.para(&self.vulnerability.anchore_username)?;
        ui.para(&self.vulnerability.anchore_password)?;
        ui.end("Loaded.")?;

        Ok(())
    }

    /// Returns the a tuple for tls usage with
    /// Option<(tls file location, bytes loaded from the name in the config toml file,
    ///        tls password if present or empty string)>
    fn tlspair_as_bytes(tls: Option<String>, tls_password: Option<String>) -> TLSPair {
        tls.clone().and_then(|t| {
            read_key_in_bytes(&PathBuf::from(t.clone()))
                .map(|p| (t.clone(), p, tls_password.clone().unwrap_or("".to_string())))
                .ok()
        })
    }
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
            controller: ControllerCfg::default(),
            scheduler: SchedulerCfg::default(),
            vnc: VNCCfg::default(),
        }
    }
}

/// A trait
impl AuthenticationFlowCfg for Config {
    //
    fn modes(&self) -> Vec<(String, String)> {
        //self.identity.enabled
        vec![("service_account".to_string(), "testing.pub".to_string())]
    }

    fn ready(&self) -> bool {
        self.identity.valid().is_ok()
    }
}

impl ConfigValidator for Config {
    fn valid(&self) -> Result<()> {
        vec![
            self.https.valid(),
            self.http2.valid(),
            self.telemetry.valid(),
            self.identity.valid(),
            self.vaults.valid(),
            self.licenses.valid(),
            self.logs.valid(),
            self.blockchain.valid(),
            self.marketplaces.valid(),
        ].iter()
            .fold(Ok(()), |acc, x| match x {
                &Ok(()) => return acc,
                &Err(ref e) => {
                    if acc.is_ok() {
                        return acc;
                    }
                    Err(Error::MissingConfiguration(format!(
                        "{}\n{}",
                        e,
                        acc.unwrap_err()
                    )))
                }
            })
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

    fn tls_pair(&self) -> TLSPair {
        Config::tlspair_as_bytes(self.tls(), self.tls_password())
    }

    fn tls(&self) -> Option<String> {
        self.https
            .tls
            .clone()
            .map(|n| (&*rioconfig_config_path(None).join(n).to_str().unwrap()).to_string())
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

    fn http2_tls_pair(&self) -> TLSPair {
        Config::tlspair_as_bytes(self.http2_tls(), self.http2_tls_password())
    }
    
    fn http2_tls(&self) -> Option<String> {
        self.http2
            .tls
            .clone()
            .map(|n| (&*rioconfig_config_path(None).join(n).to_str().unwrap()).to_string())
    }

    fn http2_tls_password(&self) -> Option<String> {
        self.http2.tls_password.clone()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ControllerCfg {
    pub url: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct SchedulerCfg {
    pub url: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct VNCCfg {
    pub url: Option<String>,
}

//A delegate, that returns the metrics (prometheus) config from the loaded prometheus config
impl Telemetry for Config {
    fn endpoint(&self) -> &str {
        &self.telemetry.endpoint
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
        enabled = ["password", "token", "service_account", "passticket"]
        params = { service_account = "service_account.pub" }
        
        [marketplaces]
        endpoint = "https://marketplaces.rioos.xyz:6443/api/v1"
        username = "rioosdolphin@rio.company"
        token = "srXrg7a1T3Th3kmU1cz5-2dtpkX9DaUSXoD5R"

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
        influx_endpoint = "http://localhost:8086"
        influx_prefix = "rioos_logs"

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

        assert_eq!(config.identity.enabled, vec!["password", "token"]);

        assert_eq!(config.vaults.backend, "Local");

        assert_eq!(config.licenses.so_file, "ShaferFilechck.so");
        assert_eq!(config.licenses.activation_code, "");

        assert_eq!(config.logs.influx_endpoint, "http://localhost:8086");
        assert_eq!(config.logs.influx_prefix, "rioos_logs");

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

        assert_eq!(config.identity.enabled, vec!["password", "token"]);

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
