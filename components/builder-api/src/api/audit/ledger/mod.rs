// Copyright 2018 The Rio Advancement Inc
//

//! Contract for security retrieval of hidden secrets.
//!
//! As deployments are running, their secret is stored in a storge.

pub mod warehouse;

use api::audit::config::{AuditBackend, BlockchainConn};
use error::Result;
use protocol::api::audit::{Envelope, EnvelopeResponse};
use protocol::api::base::IdGet;

/// Envelope list
pub type EnvelopeOutputList = Result<Option<Vec<EnvelopeResponse>>>;

/// Currently implemented ledger backends

pub trait Ledger: Send {
    /// Store the envelop in the warehouse storage.
    fn record_event(&self, envl: &Envelope) -> Result<()>;

    /// Store the envelop in the warehouse storage.
    fn record_audit(&self, envl: &Envelope) -> Result<()>;

    ///  retrieves the audits output for that accountfrom
    /// warehouse storage.
    fn retrieve_audits(&self) -> EnvelopeOutputList;

    /// Given a `account_id`, retrieves the events output for that accountfrom
    /// warehouse storage.
    fn retrieve_events(&self, id: &IdGet) -> EnvelopeOutputList;
}

/// Create appropriate Ledger variant based on configuration values.
pub fn from_config(config: &BlockchainConn) -> Result<Box<Ledger>> {
    match config.backend {
        AuditBackend::Exonum => Ok(Box::new(warehouse::Blockchain::new(config)?)),
    }
}
