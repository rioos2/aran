// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv, netsrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct NetworkDS;

impl NetworkDS {
    pub fn network_create(datastore: &DataStoreConn, net_create: &netsrv::Network) -> Result<Option<netsrv::Network>> {
        let conn = datastore.pool.get_shard(0)?;
        let status_str = serde_json::to_string(net_create.get_status()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_network_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(net_create.get_name() as String),
                &(net_create.get_network_type() as String),
                &(net_create.get_subnet_ip() as String),
                &(net_create.get_netmask() as String),
                &(net_create.get_gateway() as String),
                &(status_str as String),
            ],
        ).map_err(Error::NetworkCreate)?;
        let network = row_to_network(&rows.get(0))?;
        return Ok(Some(network.clone()));
    }

    pub fn network_list(datastore: &DataStoreConn) -> Result<Option<netsrv::NetworkGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_networks_v1()", &[]).map_err(
            Error::NetworkGetResponse,
        )?;

        let mut response = netsrv::NetworkGetResponse::new();

        let mut network_collection = Vec::new();
        for row in rows {
            network_collection.push(row_to_network(&row)?)
        }
        response.set_network_collection(
            network_collection,
            "NetworkList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }
}

fn row_to_network(row: &postgres::rows::Row) -> Result<netsrv::Network> {
    let mut network = netsrv::Network::new();
    debug!("◖☩ START: row_to_secret");
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let network_type: String = row.get("network_type");
    let subnet_ip: String = row.get("subnet_ip");
    let netmask: String = row.get("netmask");
    let gateway: String = row.get("gateway");
    let status: String = row.get("status");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let obj_meta = servicesrv::ObjectMetaData::new();
    let mut type_meta = asmsrv::TypeMeta::new();


    network.set_id(id.to_string() as String);
    let status: asmsrv::Status = serde_json::from_str(&status).unwrap();
    network.set_status(status);
    network.set_name(name);
    network.set_network_type(network_type);
    network.set_subnet_ip(subnet_ip);
    network.set_netmask(netmask);
    network.set_gateway(gateway);
    network.set_created_at(created_at.to_rfc3339());
    network.set_object_meta(obj_meta);
    type_meta.set_kind("Networks".to_string());
    type_meta.set_api_version("v1".to_string());
    network.set_type_meta(type_meta);
    debug!("◖☩ ASM: row_to_secret =>\n{:?}", network);
    debug!("◖☩ DONE: row_to_secret");
    Ok(network)
}