// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv, storagesrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use std::collections::BTreeMap;

pub struct StorageDS;

impl StorageDS {
    pub fn storage_create(datastore: &DataStoreConn, storage_create: &storagesrv::Storage) -> Result<Option<storagesrv::Storage>> {
        let conn = datastore.pool.get_shard(0)?;
        let object_meta = serde_json::to_string(storage_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(storage_create.get_type_meta()).unwrap();
        let status_str = serde_json::to_string(storage_create.get_status()).unwrap();
        let parameter_str = serde_json::to_string(storage_create.get_parameters()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_storage_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(object_meta as String),
                &(type_meta as String),
                &(storage_create.get_name() as String),
                &(storage_create.get_host_ip() as String),
                &(storage_create.get_storage_type() as String),
                &(parameter_str as String),
                &(status_str as String),
            ],
        ).map_err(Error::StorageCreate)?;
        let storage = row_to_storage(&rows.get(0))?;
        return Ok(Some(storage.clone()));
    }
}

fn row_to_storage(row: &postgres::rows::Row) -> Result<storagesrv::Storage> {
    let mut storage = storagesrv::Storage::new();
    debug!("◖☩ START: row_to_secret");
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let storage_type: String = row.get("storage_type");
    let host_ip: String = row.get("host_ip");
    let parameters: String = row.get("parameters");
    let status: String = row.get("status");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    storage.set_id(id.to_string() as String);
    let parameters_obj: BTreeMap<String, String> = serde_json::from_str(&parameters).unwrap();
    storage.set_paramaters(parameters_obj);
    let object_meta_obj: servicesrv::ObjectMetaData = serde_json::from_str(&object_meta).unwrap();
    storage.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    storage.set_type_meta(type_meta_obj);
    let status: storagesrv::Status = serde_json::from_str(&status).unwrap();
    storage.set_status(status);
    storage.set_name(name);
    storage.set_host_ip(host_ip);
    storage.set_storage_type(storage_type);
    storage.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ ASM: row_to_secret =>\n{:?}", storage);
    debug!("◖☩ DONE: row_to_secret");
    Ok(storage)
}
