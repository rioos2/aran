// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv, netsrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use std::collections::BTreeMap;

pub struct NetworkDS;

impl NetworkDS {
    pub fn network_create(datastore: &DataStoreConn, net_create: &netsrv::Network) -> Result<Option<netsrv::Network>> {
        let conn = datastore.pool.get_shard(0)?;
        let object_meta = serde_json::to_string(net_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(net_create.get_type_meta()).unwrap();
        let status_str = serde_json::to_string(net_create.get_status()).unwrap();
        let parameter_str = serde_json::to_string(net_create.get_parameters()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_network_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(object_meta as String),
                &(type_meta as String),
                &(net_create.get_name() as String),
                &(net_create.get_host_ip() as String),
                &(net_create.get_storage_type() as String),
                &(parameter_str as String),
                &(status_str as String),
            ],
        ).map_err(Error::NetworkCreate)?;
        let network = row_to_network(&rows.get(0))?;
        return Ok(Some(network.clone()));
    }
}

fn row_to_network(row: &postgres::rows::Row) -> Result<netsrv::Network> {
    let mut network = netsrv::Network::new();
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

    network.set_id(id.to_string() as String);
    let parameters_obj: BTreeMap<String, String> = serde_json::from_str(&parameters).unwrap();
    network.set_paramaters(parameters_obj);
    let object_meta_obj: servicesrv::ObjectMetaData = serde_json::from_str(&object_meta).unwrap();
    network.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    network.set_type_meta(type_meta_obj);
    let status: netsrv::Status = serde_json::from_str(&status).unwrap();
    network.set_status(status);
    network.set_name(name);
    network.set_host_ip(host_ip);
    network.set_storage_type(storage_type);
    network.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ ASM: row_to_secret =>\n{:?}", network);
    debug!("◖☩ DONE: row_to_secret");
    Ok(network)
}
