// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::secretsrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use std::collections::BTreeMap;


pub struct SecretDS;

impl SecretDS {
    pub fn secret_create(datastore: &DataStoreConn, secret_create: &secretsrv::Secret) -> Result<Option<secretsrv::Secret>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(secret_create.get_data()).unwrap();
        debug!("◖☩ START: secret_create ");
        let rows = &conn.query(
            "SELECT * FROM insert_secret_v1($1)",
            &[&(spec_str as String)],
        ).map_err(Error::SecretCreate)?;
        debug!(">● ROWS: secret_create =>\n{:?}", &rows);
        let node = row_to_secret(&rows.get(0))?;
        debug!("◖☩ DONE:secret_create ");
        return Ok(Some(node.clone()));
    }
}


fn row_to_secret(row: &postgres::rows::Row) -> Result<secretsrv::Secret> {
    let mut secret = secretsrv::Secret::new();
    debug!("◖☩ START: row_to_secret");
    let data: String = row.get("data");
    let data_obj: BTreeMap<String, String> = serde_json::from_str(&data).unwrap();
    secret.set_data(data_obj);
    debug!("◖☩ ASM: row_to_secret =>\n{:?}", secret);
    debug!("◖☩ DONE: row_to_secret");
    Ok(secret)
}
