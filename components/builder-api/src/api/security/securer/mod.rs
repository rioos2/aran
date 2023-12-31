// Copyright 2018 The Rio Advancement Inc
//

//! Contract for security retrieval of hidden secrets.
//!
//! As deployments are running, their secret is stored in a storge.

pub mod local;
pub mod vault;
pub mod parse;

use db::data_store::DataStoreConn;
use error::Result;
use protocol::api::secret::Secret;
use rio_net::http::middleware::SecurerConn;
use rio_net::config::SecureBackend;
use service::{SecretOutput, SecretOutputList};
use protocol::api::base::IdGet;

/// Currently implemented securer backends

pub trait Securer: Send {
    /// Given a `job_id` and the path to the log output for that job,
    /// places the log in an archive for long-term storage.
    fn seal(&self) -> Result<()>;

    /// Given a `job_id`, retrieves the log output for that job from
    /// long-term storage.
    fn unseal(&self) -> Result<()>;

    /// Given a `job_id` and the path to the log output for that job,
    /// places the log in an archive for long-term storage.
    fn status(&self) -> Result<()>;

    /// Given a `job_id` and the path to the log output for that job,
    /// places the log in an archive for long-term storage.
    fn secure(&self, security_req: &Secret) -> SecretOutput;

    /// Given a `job_id`, retrieves the log output for that job from
    /// long-term storage.
    fn retrieve_by(&self, id: &IdGet) -> SecretOutputList;

    /// Given a `job_id`, retrieves the log output for that job from
    /// long-term storage.

    fn retrieve(&self) -> SecretOutputList;
}

/// Create appropriate Securer variant based on configuration values.
pub fn from_config(config: &SecurerConn, conn: Box<DataStoreConn>) -> Result<Box<Securer>> {
    match config.backend {
        SecureBackend::Local => Ok(Box::new(local::LocalSecurer::new(conn)?)),
        SecureBackend::EnvKey => Ok(Box::new(vault::EnvKeySecurer::new(config)?)),
    }
}
