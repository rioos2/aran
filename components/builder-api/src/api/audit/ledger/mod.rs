// Copyright 2018 The Rio Advancement Inc
//

//! Contract for security retrieval of hidden secrets.
//!
//! As deployments are running, their secret is stored in a storge.

pub mod warehouse;

use error::Result;
use protocol::api::audit::{EnvelopeResponse, Envelope};
use protocol::api::base::IdGet;
use rio_net::http::middleware::BlockchainConn;
use rio_net::config::AuditBackend;

/// Envelope list
pub type EnvelopeOutputList = Result<Option<Vec<EnvelopeResponse>>>;

/// Currently implemented ledger backends

pub trait Ledger: Send {
    /// Store the envelop in the warehouse storage.
    fn record(&self, envl: &Envelope) -> Result<()>;

    /// Given a `account_id`, retrieves the events output for that accountfrom
    /// warehouse storage.
    fn retrieve_by(&self, id: &IdGet) -> EnvelopeOutputList;
}

/// Create appropriate Ledger variant based on configuration values.
pub fn from_config(config: &BlockchainConn) -> Result<Box<Ledger>> {
    match config.backend {
        AuditBackend::Exonum => Ok(Box::new(warehouse::Blockchain::new(config)?)),
    }
}
