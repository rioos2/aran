use error::{Error, Result};
use std::path::PathBuf;

use audit::config::LogsCfg;
use watch::config::StreamerCfg;

use api::audit::config::{AppStoresCfg, BlockchainCfg};
use api::deploy::config::ServicesCfg;
use api::security::config::SecurerCfg;
use rio_core::fs::rioconfig_config_path;

use auth::config::{IdentityCfg, PLUGIN_SERVICE_ACCOUNT};

use entitlement::config::LicensesCfg;
use telemetry::config::TelemetryCfg;

use http_gateway::config::prelude::*;

pub trait ConfigValidator {
    fn valid(&self) -> Result<()>;
}

/// Validate the presence of listener, port, and tls
impl ConfigValidator for HttpsCfg {
    fn valid(&self) -> Result<()> {
        if self.tls.is_none() {
            return Err(Error::MissingConfiguration(
                "Missing  in api.toml. [https] → tls".to_string(),
            ));
        }

        let tls_location = PathBuf::from(&*rioconfig_config_path(None)
            .join(self.tls.clone().unwrap())
            .to_str()
            .unwrap());

        if !tls_location.exists() {
            return Err(Error::MissingConfiguration(format!(
                "File Not Found at {}",
                tls_location.display()
            )));
        }

        Ok(())
    }
}

/// Validate the presence of listener, port, and tls
impl ConfigValidator for StreamerCfg {
    fn valid(&self) -> Result<()> {
        if self.tls.is_none() {
            return Err(Error::MissingConfiguration(
                "Missing  in api.toml. [http2] → tls".to_string(),
            ));
        }
        Ok(())
    }
}

/// Validate the presence of telemetry endpoint
impl ConfigValidator for TelemetryCfg {
    fn valid(&self) -> Result<()> {
        if self.endpoint.is_empty() {
            return Err(Error::MissingConfiguration(
                "Missing  in api.toml. [telemetry] → endpoint".to_string(),
            ));
        }

        Ok(())
    }
}

/// Validate the presence of backend
///  This is an enum, ideally we need to check it up using match.
impl ConfigValidator for SecurerCfg {
    fn valid(&self) -> Result<()> {
        Ok(())
    }
}

/// Validate the presence of modes altleast (password, token)
/// If service_account is enabled, then `pub` must exist
impl ConfigValidator for IdentityCfg {
    fn valid(&self) -> Result<()> {
        debug!("Validating identity");

        let mut s: Vec<&str> = vec![];

        if self.enabled.is_empty() {
            s.push("enabled");
        }

        self.enabled
            .clone()
            .into_iter()
            .map(|m| match m.as_str() {
                PLUGIN_SERVICE_ACCOUNT => {
                    if !(self.params.contains_key(&m)) {
                        s.push("params {service_account}");
                    }
                }
                &_ => {}
            })
            .collect::<()>();

        if s.is_empty() {
            return Ok(());
        }

        debug!("Error in validating identity.");

        Err(Error::MissingConfiguration(format!(
            "Missing  in api.toml. [identity] → {:?}",
            s
        )))
    }
}

/// Validate the presence of all the loadbalancer fields
impl ConfigValidator for ServicesCfg {
    fn valid(&self) -> Result<()> {
        debug!("Validating services");
        let mut s: Vec<&str> = vec![];

        if self.loadbalancer_imagein.is_empty() {
            s.push("loadbalancer_imagein");
        }
        if self.loadbalancer_imagename.is_empty() {
            s.push("loadbalancer_imagename");
        }
        if self.loadbalancer_cpu.is_empty() {
            s.push("loadbalancer_cpu");
        }

        if self.loadbalancer_mem.is_empty() {
            s.push("loadbalancer_mem");
        }

        if self.loadbalancer_mem.is_empty() {
            s.push("loadbalancer_mem");
        }

        if self.loadbalancer_disk.is_empty() {
            s.push("loadbalancer_disk");
        }

        if s.is_empty() {
            return Ok(());
        }

        debug!("Error in validating services.");

        Err(Error::MissingConfiguration(format!(
            "Missing  in api.toml. [services] → {:?}",
            s
        )))
    }
}

/// Validate the presence of so_file alteast
impl ConfigValidator for LicensesCfg {
    fn valid(&self) -> Result<()> {
        if self.so_file.is_empty() {
            return Err(Error::MissingConfiguration(
                "Missing  in api.toml. [licenses] → so_file".to_string(),
            ));
        }
        Ok(())
    }
}

/// Validate the presence of influx endpoint
impl ConfigValidator for LogsCfg {
    fn valid(&self) -> Result<()> {
        if self.influx_endpoint.is_empty() {
            return Err(Error::MissingConfiguration(
                "Missing  in api.toml.  [logs] → influx_endpoint".to_string(),
            ));
        }
        Ok(())
    }
}

/// Validate the presence of blockchain endpoint
impl ConfigValidator for BlockchainCfg {
    fn valid(&self) -> Result<()> {
        if self.endpoint.is_empty() {
            return Err(Error::MissingConfiguration(
                "Missing  in api.toml. [blockchain] → endpoint".to_string(),
            ));
        }
        Ok(())
    }
}

/// Validate the presence of appstore endpoint, email, token
impl ConfigValidator for AppStoresCfg {
    fn valid(&self) -> Result<()> {
        let mut s: Vec<&str> = vec![];

        if self.endpoint.is_empty() {
            s.push("endpoint");
        }
        if self.token.is_empty() {
            s.push("token");
        }
        if self.username.is_empty() {
            s.push("username");
        }

        if s.is_empty() {
            return Ok(());
        }

        Err(Error::MissingConfiguration(format!(
            "Missing  in api.toml. [appstores] → {:?}",
            s
        )))
    }
}
