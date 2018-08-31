// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use chrono::prelude::*;
use error::{Error, Result};
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::api::network;
use std::process::exit;

use db::data_store::DataStoreConn;
use postgres;
use serde_json;

use super::{NetworkOutput, NetworkOutputList};

pub struct NetworkDS;

impl NetworkDS {
    pub fn create(datastore: &DataStoreConn, net_create: &network::Network) -> NetworkOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_network_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(net_create.get_network_type() as String),
                &(net_create.get_subnet_ip() as String),
                &(net_create.get_netmask() as String),
                &(net_create.get_gateway() as String),
                &(net_create.get_used_bits() as Vec<i16>),
                &(serde_json::to_value(net_create.get_bridge_hosts()).unwrap()),
                &(serde_json::to_value(net_create.get_status()).unwrap()),
                &(serde_json::to_value(net_create.object_meta()).unwrap()),
                &(serde_json::to_value(net_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::NetworkCreate)?;
        if rows.len() > 0 {
            let network = row_to_network(&rows.get(0))?;
            return Ok(Some(network.clone()));
        }
        Ok(None)
    }

    pub fn list_blank(datastore: &DataStoreConn) -> NetworkOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_networks_v1()", &[])
            .map_err(Error::NetworkGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_network(&row)?)
            }
            return Ok(Some(response));
        }       
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, net_get: &IdGet) -> NetworkOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_network_v1($1)",
            &[&(net_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::NetworksGet)?;

        if rows.len() > 0 {
            let net = row_to_network(&rows.get(0))?;            
            return Ok(Some(net));
        }       
        
        Ok(None)
    }
    pub fn update(datastore: &DataStoreConn, net: &network::Network) -> NetworkOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_net_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(net.get_id().parse::<i64>().unwrap()),
                &(net.get_network_type() as String),
                &(net.get_subnet_ip() as String),
                &(net.get_netmask() as String),
                &(net.get_gateway() as String),
                &(net.get_used_bits() as Vec<i16>),
                &(serde_json::to_value(net.get_bridge_hosts()).unwrap()),
                &(serde_json::to_value(net.get_status()).unwrap()),
                &(serde_json::to_value(net.object_meta()).unwrap()),
            ],
        ).map_err(Error::NetUpdate)?;
        if rows.len() > 0 {
            let network = row_to_network(&rows.get(0))?;
            return Ok(Some(network));
        }
        Ok(None)
    }
}

fn row_to_network(row: &postgres::rows::Row) -> Result<network::Network> {
    let mut network = network::Network::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    network.set_id(id.to_string());
    network.set_status(serde_json::from_value(row.get("status")).unwrap());
    network.set_network_type(row.get("network_type"));
    network.set_subnet_ip(row.get("subnet_ip"));
    network.set_netmask(row.get("netmask"));
    network.set_gateway(row.get("gateway"));
    network.set_used_bits(row.get("used_bits"));
    network.set_bridge_hosts(serde_json::from_value(row.get("bridge_hosts")).unwrap());
    network.set_created_at(created_at.to_rfc3339());
    Ok(network)
}
