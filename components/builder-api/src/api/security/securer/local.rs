// Copyright 2018 The Rio Advancement Inc
//
//! Securer variant which stores hidden gems in the local database.
//!
use db::data_store::DataStoreConn;
use error::Result;
use protocol::api::secret::Secret;

use super::Securer;
use protocol::api::base::IdGet;
use service::models::secret;
use service::{SecretOutput, SecretOutputList};

/// Wraps a `DataStoreConn` representing the root of a local vault security.
pub struct LocalSecurer(Box<DataStoreConn>);

impl LocalSecurer {
    pub fn new(conn: Box<DataStoreConn>) -> Result<Self> {
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
        secret::DataStore::create(&self.0, &secret_create)
    }

    fn retrieve_by(&self, id: &IdGet) -> SecretOutputList {
        secret::DataStore::list(&self.0, id)
    }

    fn retrieve(&self) -> SecretOutputList {
        secret::DataStore::list_blank(&self.0)
    }
}
