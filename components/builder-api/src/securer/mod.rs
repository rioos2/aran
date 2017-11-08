// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//

//! Contract for security retrieval of hidden secrets.
//!
//! As deployments are running, their secret is stored in a storge.

pub mod local;
pub mod vault;

use config::SecurerCfg;
use error::Result;
use std::path::PathBuf;

/// Currently implemented securer backends
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SecureBackend {
    Local,
    VAULT,
}

pub trait Securer: Send {
    /// Given a `job_id` and the path to the log output for that job,
    /// places the log in an archive for long-term storage.
    fn seal(&self) -> Result<()>;

    /// Given a `job_id`, retrieves the log output for that job from
    /// long-term storage.
    fn unseal(&self) -> Result<Vec<String>>;

    /// Given a `job_id` and the path to the log output for that job,
    /// places the log in an archive for long-term storage.
    fn status(&self) -> Result<()>;

    /// Given a `job_id` and the path to the log output for that job,
    /// places the log in an archive for long-term storage.
    fn secure(&self, security_id: u64, security_req: &PathBuf) -> Result<()>;

    /// Given a `job_id`, retrieves the log output for that job from
    /// long-term storage.
    fn retrieve(&self, security_id: u64) -> Result<Vec<String>>;
}

/// Create appropriate Securer variant based on configuration values.
pub fn from_config(config: &SecurerCfg, conn: &DataStoreConn) -> Result<Box<Securer>> {
    match config.backend {
        SecureBackend::Local => Ok(Box::new(local::LocalSecurer::new(conn)?)),
        SecureBackend::VAULT => Ok(Box::new(vault::VaultSecurer::new(config)?)),
    }
}
