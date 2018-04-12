// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::storage;
use protocol::api::base::{IdGet, StatusUpdate, MetaFields};

use postgres;
use db::data_store::DataStoreConn;

use serde_json;

use super::{StorageConnectorOutput, StorageConnectorOutputList, DatacenterOutput, DatacenterOutputList, StoragePoolOutput, StoragePoolOutputList};

pub struct StorageDS;

impl StorageDS {
    pub fn storage_create(datastore: &DataStoreConn, storage_create: &storage::Storage) -> StorageConnectorOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_storage_v1($1,$2,$3,$4,$5,$6,$7,$8)",
            &[
                &(storage_create.get_host_ip() as String),
                &(storage_create.get_storage_type() as String),
                &(serde_json::to_value(storage_create.get_parameters()).unwrap()),
                &(serde_json::to_value(storage_create.get_storage_info()).unwrap()),
                &(serde_json::to_value(storage_create.get_node_info()).unwrap()),
                &(serde_json::to_value(storage_create.get_status()).unwrap()),
                &(serde_json::to_value(storage_create.object_meta()).unwrap()),
                &(serde_json::to_value(storage_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::StorageCreate)?;
        if rows.len() > 0 {
            let storage = row_to_storage(&rows.get(0))?;
            return Ok(Some(storage));
        }
        Ok(None)
    }

    pub fn storage_list(datastore: &DataStoreConn) -> StorageConnectorOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_storages_v1()", &[]).map_err(
            Error::StorageGetResponse,
        )?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_storage(&row)?)
            }

            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn storage_get_by_ip(datastore: &DataStoreConn, get_storage: &IdGet) -> StorageConnectorOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_storages_by_ip_v1($1)",
            &[&(get_storage.get_id() as String)],
        ).map_err(Error::StorageGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_storage(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn storage_show(datastore: &DataStoreConn, get_storage: &IdGet) -> StorageConnectorOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_storage_v1($1)",
            &[&(get_storage.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::StorageGet)?;
        if rows.len() > 0 {
            let storage = row_to_storage(&rows.get(0))?;
            return Ok(Some(storage));
        }
        Ok(None)
    }

    pub fn storage_status_update(datastore: &DataStoreConn, upd: &StatusUpdate) -> StorageConnectorOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_storage_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::StorageSetStatus)?;
        if rows.len() > 0 {
            for row in rows {
                let storage = row_to_storage(&row)?;
                return Ok(Some(storage));
            }
        }
        Ok(None)
    }

    pub fn storage_update(datastore: &DataStoreConn, storage_create: &storage::Storage) -> StorageConnectorOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_storage_v1($1,$2,$3,$4,$5,$6,$7,$8)",
            &[
                &(storage_create.get_id().parse::<i64>().unwrap()),
                &(storage_create.get_host_ip() as String),
                &(storage_create.get_storage_type() as String),
                &(serde_json::to_value(storage_create.get_parameters()).unwrap()),
                &(serde_json::to_value(storage_create.get_storage_info()).unwrap()),
                &(serde_json::to_value(storage_create.get_node_info()).unwrap()),
                &(serde_json::to_value(storage_create.get_status()).unwrap()),
                &(serde_json::to_value(storage_create.object_meta()).unwrap()),
            ],
        ).map_err(Error::StorageCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let storage = row_to_storage(&row)?;
                return Ok(Some(storage));
            }
        }
        Ok(None)
    }

    pub fn data_center_create(datastore: &DataStoreConn, dc_create: &storage::DataCenter) -> DatacenterOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_dc_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(dc_create.get_nodes() as Vec<String>),
                &(dc_create.get_networks() as Vec<String>),
                &(dc_create.get_enabled() as bool),
                &(dc_create.get_storage() as String),
                &(serde_json::to_value(dc_create.get_advanced_settings()).unwrap()),
                &(dc_create.get_flag() as String),
                &(dc_create.get_currency() as String),
                &(serde_json::to_value(dc_create.get_status()).unwrap()),
                &(serde_json::to_value(dc_create.object_meta()).unwrap()),
                &(serde_json::to_value(dc_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::DcCreate)?;
        if rows.len() > 0 {
            let dc = row_to_dc(&rows.get(0))?;
            return Ok(Some(dc));
        }
        Ok(None)
    }

    pub fn data_center_list(datastore: &DataStoreConn) -> DatacenterOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_data_centers_v1()", &[])
            .map_err(Error::DcGetResponse)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_dc(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn data_center_show(datastore: &DataStoreConn, get_dc: &IdGet) -> DatacenterOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_data_center_v1($1)",
            &[&(get_dc.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::StorageGet)?;
        if rows.len() > 0 {
            for row in rows {
                let dc = row_to_dc(&row)?;
                return Ok(Some(dc));
            }
        }
        Ok(None)
    }


    pub fn datacenter_update(datastore: &DataStoreConn, dc: &storage::DataCenter) -> DatacenterOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_datacenter_by_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(dc.get_id().parse::<i64>().unwrap()),
                &(dc.get_nodes() as Vec<String>),
                &(dc.get_networks() as Vec<String>),
                &(dc.get_enabled() as bool),
                &(dc.get_storage() as String),
                &(serde_json::to_value(dc.get_advanced_settings()).unwrap()),
                &(dc.get_flag() as String),
                &(dc.get_currency() as String),
                &(serde_json::to_value(dc.get_status()).unwrap()),
                &(serde_json::to_value(dc.object_meta()).unwrap()),
            ],
        ).map_err(Error::DatacenterUpdate)?;
        if rows.len() > 0 {
            let dc = row_to_dc(&rows.get(0))?;
            return Ok(Some(dc));
        }
        Ok(None)
    }

    pub fn storage_pool_create(datastore: &DataStoreConn, storage_create: &storage::StoragePool) -> StoragePoolOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_storage_pool_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(storage_create.get_connector_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(storage_create.get_parameters()).unwrap()),
                &(serde_json::to_value(storage_create.get_remote_storage_disks()).unwrap()),
                &(serde_json::to_value(storage_create.get_storage_info()).unwrap()),
                &(serde_json::to_value(storage_create.get_status()).unwrap()),
                &(serde_json::to_value(storage_create.object_meta()).unwrap()),
                &(serde_json::to_value(storage_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::StoragePoolCreate)?;
        if rows.len() > 0 {
            let storage = row_to_storage_pool(&rows.get(0))?;
            return Ok(Some(storage));
        }
        Ok(None)
    }

    pub fn storage_pool_list_all(datastore: &DataStoreConn) -> StoragePoolOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_storage_pool_all_v1()", &[])
            .map_err(Error::StoragePoolGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_storage_pool(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn storage_pool_list(datastore: &DataStoreConn, get_storage: &IdGet) -> StoragePoolOutputList {
        let conn = datastore.pool.get_shard(0)?;
        let connector_id = get_storage.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_storage_pool_v1($1)", &[&connector_id])
            .map_err(Error::StoragePoolGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_storage_pool(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn storage_pool_show(datastore: &DataStoreConn, get_storage: &IdGet) -> StoragePoolOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_storage_pool_by_id_v1($1)",
            &[&(get_storage.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::StoragePoolGetResponse)?;

        if rows.len() > 0 {
            let storage = row_to_storage_pool(&rows.get(0))?;
            return Ok(Some(storage));
        }
        Ok(None)
    }


    pub fn storage_pool_status_update(datastore: &DataStoreConn, upd: &StatusUpdate) -> StoragePoolOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_storage_pool_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::StoragePoolSetStatus)?;
        for row in rows {
            if rows.len() > 0 {
                let storagepool = row_to_storage_pool(&row)?;
                return Ok(Some(storagepool));
            }
        }
        Ok(None)
    }
}

fn row_to_storage(row: &postgres::rows::Row) -> Result<storage::Storage> {
    let mut storage = storage::Storage::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    storage.set_id(id.to_string());
    storage.set_host_ip(row.get("host_ip"));
    storage.set_storage_type(row.get("storage_type"));
    storage.set_paramaters(serde_json::from_value(row.get("parameters")).unwrap());
    storage.set_storage_info(serde_json::from_value(row.get("storage_info")).unwrap());
    storage.set_node_info(serde_json::from_value(row.get("node_info")).unwrap());
    storage.set_status(serde_json::from_value(row.get("status")).unwrap());
    storage.set_created_at(created_at.to_rfc3339());
    Ok(storage)
}

fn row_to_dc(row: &postgres::rows::Row) -> Result<storage::DataCenter> {
    let mut dc = storage::DataCenter::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let networks: Vec<String> = row.get("networks");
    let enabled: bool = row.get("enabled");
    let nodes: Vec<String> = row.get("nodes");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    dc.set_id(id.to_string());
    dc.set_networks(networks);
    dc.set_storage(row.get("storage"));
    dc.set_flag(row.get("flag"));
    dc.set_enabled(enabled);
    dc.set_currency(row.get("currency"));
    dc.set_nodes(nodes);
    dc.set_advanced_settings(
        serde_json::from_value(row.get("advanced_settings")).unwrap(),
    );
    dc.set_status(serde_json::from_value(row.get("status")).unwrap());
    dc.set_created_at(created_at.to_rfc3339());
    Ok(dc)
}

fn row_to_storage_pool(row: &postgres::rows::Row) -> Result<storage::StoragePool> {
    let mut storage = storage::StoragePool::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let connector_id: i64 = row.get("connector_id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    storage.set_id(id.to_string());
    storage.set_connector_id(connector_id.to_string());
    storage.set_paramaters(serde_json::from_value(row.get("parameters")).unwrap());
    storage.set_remote_storage_disks(
        serde_json::from_value(row.get("remote_storage_disks")).unwrap(),
    );
    storage.set_storage_info(serde_json::from_value(row.get("storage_info")).unwrap());
    storage.set_status(serde_json::from_value(row.get("status")).unwrap());
    storage.set_created_at(created_at.to_rfc3339());
    Ok(storage)
}
