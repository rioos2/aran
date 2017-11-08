// Copyright (c) 2017 RioCorp Inc.
//
//! Securer variant which stores hidden gems in the local database.
//!

use error::Result;
use std::path::DatastoreConn;

use super::Securer;

/// Wraps a `DatastoreConn` representing the root of a local vault security.
pub struct LocalSecurer(DatastoreConn);

impl LocalSecurer {
    pub fn new(conn: &DatastoreConn) -> Result<LocalSecurer> {
        Ok(LocalSecurer(conn))
    }
}

impl Securer for LocalSecurer {
    fn seal() -> Result<()> {
        Ok(())
    }

    fn status() -> Result<()> {
        Ok(())
    }

    fn unseal() -> Result<()> {
        Ok(())
    }

    fn secure(&self, security_id: u64, security_req: &SecurityCreateReq) -> Result<()> {
        Ok(())
    }

    fn retrieve(&self, security_id: u64) -> Result<Vec<String>> {
        Ok(())
    }
}
