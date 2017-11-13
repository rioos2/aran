// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//

//! Contract for security retrieval of hidden secrets.
//!
//! As deployments are running, their secret is stored in a storge.

pub mod local;
pub mod vault;
use db::data_store::DataStoreConn;
use error::Result;
use protocol::servicesrv::Secret;
use rio_net::http::middleware::SecurerConn;
use rio_net::config::SecureBackend;

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
    fn secure(&self, security_req: &Secret) -> Result<Option<Secret>>;

    /// Given a `job_id`, retrieves the log output for that job from
    /// long-term storage.
    fn retrieve(&self) -> Result<Option<Secret>>;
}

/// Create appropriate Securer variant based on configuration values.
pub fn from_config(config: &SecurerConn, conn: &DataStoreConn) -> Result<Box<Securer>> {
    match config.backend {
        SecureBackend::Local => Ok(Box::new(local::LocalSecurer::new(conn)?)),
        SecureBackend::EnvKey => Ok(Box::new(vault::EnvKeySecurer::new(config)?)),
    }
}
