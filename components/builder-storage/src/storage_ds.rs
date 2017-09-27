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

    pub fn storage_list(datastore: &DataStoreConn) -> Result<Option<storagesrv::StorageGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_storages_v1()", &[]).map_err(
            Error::StorageGetResponse,
        )?;

        let mut response = storagesrv::StorageGetResponse::new();

        let mut storage_collection = Vec::new();
        for row in rows {
            storage_collection.push(row_to_storage(&row)?)
        }
        response.set_storage_collection(
            storage_collection,
            "StorageList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }

    pub fn storage_show(datastore: &DataStoreConn, get_storage: &asmsrv::IdGet) -> Result<Option<storagesrv::Storage>> {
        let conn = datastore.pool.get_shard(0)?;
        let storage_id = get_storage.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_storage_v1($1)", &[&storage_id])
            .map_err(Error::StorageGet)?;
        for row in rows {
            let storage = row_to_storage(&row)?;
            return Ok(Some(storage));
        }
        Ok(None)
    }

    pub fn storage_status_update(datastore: &DataStoreConn, storage_create: &storagesrv::Storage) -> Result<Option<storagesrv::Storage>> {
        let conn = datastore.pool.get_shard(0)?;
        let storage_id = storage_create.get_id().parse::<i64>().unwrap();
        let status_str = serde_json::to_string(storage_create.get_status()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM set_storage_status_v1($1, $2)",
            &[&storage_id, &(status_str as String)],
        ).map_err(Error::StorageSetStatus)?;
        for row in rows {
            let storage = row_to_storage(&row)?;
            return Ok(Some(storage));
        }
        Ok(None)
    }

    pub fn storage_update(datastore: &DataStoreConn, storage_create: &storagesrv::Storage) -> Result<Option<storagesrv::Storage>> {
        let conn = datastore.pool.get_shard(0)?;
        let storage_id = storage_create.get_id().parse::<i64>().unwrap();
        let object_meta = serde_json::to_string(storage_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(storage_create.get_type_meta()).unwrap();
        let parameter_str = serde_json::to_string(storage_create.get_parameters()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM update_storage_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &storage_id,
                &(object_meta as String),
                &(type_meta as String),
                &(storage_create.get_name() as String),
                &(storage_create.get_host_ip() as String),
                &(storage_create.get_storage_type() as String),
                &(parameter_str as String),
            ],
        ).map_err(Error::StorageCreate)?;
        let storage = row_to_storage(&rows.get(0))?;
        return Ok(Some(storage.clone()));
    }

    pub fn data_center_create(datastore: &DataStoreConn, dc_create: &storagesrv::DataCenter) -> Result<Option<storagesrv::DataCenter>> {
        let conn = datastore.pool.get_shard(0)?;
        let object_meta = serde_json::to_string(dc_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(dc_create.get_type_meta()).unwrap();
        let status_str = serde_json::to_string(dc_create.get_status()).unwrap();
        let adv_str = serde_json::to_string(dc_create.get_advanced_settings()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_dc_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(object_meta as String),
                &(type_meta as String),
                &(dc_create.get_name() as String),
                &(dc_create.get_nodes() as Vec<String>),
                &(dc_create.get_networks() as Vec<String>),
                &(dc_create.get_storage() as String),
                &(adv_str as String),
                &(dc_create.get_flag() as String),
                &(dc_create.get_currency() as String),
                &(status_str as String),
            ],
        ).map_err(Error::DcCreate)?;
        let dc = row_to_dc(&rows.get(0))?;
        return Ok(Some(dc.clone()));
    }
}

fn row_to_storage(row: &postgres::rows::Row) -> Result<storagesrv::Storage> {
    let mut storage = storagesrv::Storage::new();
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
    Ok(storage)
}


fn row_to_dc(row: &postgres::rows::Row) -> Result<storagesrv::DataCenter> {
    let mut dc = storagesrv::DataCenter::new();
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let storage: String = row.get("storage");
    let flag: String = row.get("flag");
    let currency: String = row.get("currency");
    let networks: Vec<String> = row.get("networks");
    let nodes: Vec<String> = row.get("nodes");
    let advanced_settings: String = row.get("advanced_settings");
    let status: String = row.get("status");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    dc.set_id(id.to_string() as String);
    let adv_obj: BTreeMap<String, String> = serde_json::from_str(&advanced_settings).unwrap();
    dc.set_advanced_settings(adv_obj);
    let object_meta_obj: servicesrv::ObjectMetaData = serde_json::from_str(&object_meta).unwrap();
    dc.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    dc.set_type_meta(type_meta_obj);
    let status: storagesrv::DcStatus = serde_json::from_str(&status).unwrap();
    dc.set_status(status);
    dc.set_name(name);
    dc.set_networks(networks);
    dc.set_storage(storage);
    dc.set_flag(flag);
    dc.set_currency(currency);
    dc.set_nodes(nodes);
    dc.set_created_at(created_at.to_rfc3339());
    Ok(dc)
}
