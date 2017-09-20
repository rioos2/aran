// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use std::collections::BTreeMap;


pub struct ServiceAccountDS;

impl ServiceAccountDS {
    pub fn secret_create(datastore: &DataStoreConn, secret_create: &servicesrv::Secret) -> Result<Option<servicesrv::Secret>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(secret_create.get_data()).unwrap();
        let object_meta = serde_json::to_string(secret_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(secret_create.get_type_meta()).unwrap();
        debug!("◖☩ START: secret_create ");
        let rows = &conn.query(
            "SELECT * FROM insert_secret_v1($1,$2,$3)",
            &[
                &(spec_str as String),
                &(object_meta as String),
                &(type_meta as String),
            ],
        ).map_err(Error::SecretCreate)?;
        debug!(">● ROWS: secret_create =>\n{:?}", &rows);
        let secret = row_to_secret(&rows.get(0))?;
        debug!("◖☩ DONE:secret_create ");
        return Ok(Some(secret.clone()));
    }
    pub fn secret_show(datastore: &DataStoreConn, get_secret: &servicesrv::SecretGet) -> Result<Option<servicesrv::Secret>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: secret_show {:?}", get_secret.get_id());
        let secret_id = get_secret.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_secret_v1($1)", &[&secret_id])
            .map_err(Error::SecretGet)?;
        debug!(">● ROWS: secret_show =>\n{:?}", &rows);
        for row in rows {
            let secret = row_to_secret(&row)?;
            return Ok(Some(secret));
        }
        Ok(None)
    }
}


fn row_to_secret(row: &postgres::rows::Row) -> Result<servicesrv::Secret> {
    let mut secret = servicesrv::Secret::new();
    debug!("◖☩ START: row_to_secret");
    let id: i64 = row.get("id");
    let data: String = row.get("data");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    secret.set_id(id.to_string() as String);
    let data_obj: BTreeMap<String, String> = serde_json::from_str(&data).unwrap();
    secret.set_data(data_obj);
    let object_meta_obj: asmsrv::ObjectMeta = serde_json::from_str(&object_meta).unwrap();
    secret.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    secret.set_type_meta(type_meta_obj);
    secret.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ ASM: row_to_secret =>\n{:?}", secret);
    debug!("◖☩ DONE: row_to_secret");
    Ok(secret)
}
