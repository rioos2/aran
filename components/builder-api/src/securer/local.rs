// Copyright (c) 2017 RioCorp Inc.
//
//! Securer variant which stores hidden gems in the local database.
//!

use error::{Result, Error};
use db::data_store::DataStoreConn;
use protocol::servicesrv::Secret;
use super::Securer;
use service::service_account_ds::ServiceAccountDS;

/// Wraps a `DatastoreConn` representing the root of a local vault security.
pub struct LocalSecurer(DataStoreConn);

impl LocalSecurer {
    pub fn new(conn: &DataStoreConn) -> Result<Self> {
        Ok(LocalSecurer(conn.clone()))
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

    fn secure(&self, secret_create: &Secret) -> Result<Option<Secret>> {
        let secret = ServiceAccountDS::secret_create(&self.0, &secret_create)
            .map_err(Error::Secret)?;
        Ok(secret)
    }

    fn retrieve(&self, security_id: u64) -> Result<Vec<String>> {
        let data = vec![];
        Ok(data)
    }
}
