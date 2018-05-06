// Copyright 2018 The Rio Advancement Inc
//
//! Securer variant which stores hidden gems in the local database.
//!
use std::sync::Arc;

use error::Result;
use db::data_store::DataStoreConn;
use protocol::api::secret::Secret;

use super::Securer;
use service::secret_ds::SecretDS;
use service::{SecretOutput, SecretOutputList};
use protocol::api::base::IdGet;

/// Wraps a `DataStoreConn` representing the root of a local vault security.
pub struct LocalSecurer(Arc<DataStoreConn>);

impl LocalSecurer {
    pub fn new(conn: Arc<DataStoreConn>) -> Result<Self> {
        Ok(LocalSecurer(conn))
    }
}

impl Securer for LocalSecurer {
    fn seal(&self) -> Result<()> {
        Ok(())
    }

    fn status(&self) -> Result<()> {
        Ok(())
    }

    fn unseal(&self) -> Result<()> {
        Ok(())
    }

    fn secure(&self, secret_create: &Secret) -> SecretOutput {
        SecretDS::create(&self.0, &secret_create)
    }

    fn retrieve_by(&self, id: &IdGet) -> SecretOutputList {
        SecretDS::list(&self.0, id)
    }

    fn retrieve(&self) -> SecretOutputList {
        SecretDS::list_blank(&self.0)
    }
}
