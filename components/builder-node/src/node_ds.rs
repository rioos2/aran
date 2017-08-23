// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::nodesrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct NodeDS;

impl NodeDS {
    pub fn node_create(datastore: &DataStoreConn, node_create: &nodesrv::Node) -> Result<Option<nodesrv::Node>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(node_create.get_spec()).unwrap();
        let status_str = serde_json::to_string(node_create.get_status()).unwrap();
        debug!("◖☩ START: node_create ");
        let rows = &conn.query(
            "SELECT * FROM insert_node_v1($1,$2)",
            &[&(spec_str as String), &(status_str as String)],
        ).map_err(Error::NodeCreate)?;

        debug!(">● ROWS: node_create =>\n{:?}", &rows);
        let node = row_to_node(&rows.get(0))?;
        debug!("◖☩ DONE: node_create ");
        return Ok(Some(node.clone()));
    }

    pub fn node_list(datastore: &DataStoreConn) -> Result<Option<nodesrv::NodeGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_nodes_v1()", &[]).map_err(
            Error::NodeList,
        )?;

        let mut response = nodesrv::NodeGetResponse::new();

        let mut node_collection = Vec::new();

        debug!(">● ROWS: node_list =>\n{:?}", &rows);
        for row in rows {
            node_collection.push(row_to_node(&row)?)
        }
        response.set_node_collection(node_collection);
        Ok(Some(response))
    }
}

fn row_to_node(row: &postgres::rows::Row) -> Result<nodesrv::Node> {
    let mut node = nodesrv::Node::new();
    debug!("◖☩ START: row_to_node");

    let id: i64 = row.get("id");
    let status: String = row.get("status");
    let spec: String = row.get("spec");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    node.set_id(id.to_string() as String);
    let spec_obj: nodesrv::Spec = serde_json::from_str(&spec).unwrap();
    let status_obj: nodesrv::Status = serde_json::from_str(&status).unwrap();
    node.set_spec(spec_obj);
    node.set_status(status_obj);
    node.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ ASM: row_to_node =>\n{:?}", node);
    debug!("◖☩ DONE: row_to_node");
    Ok(node)
}
