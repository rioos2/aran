// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{nodesrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct NodeDS;

impl NodeDS {
    pub fn node_create(datastore: &DataStoreConn, node_create: &nodesrv::Node) -> Result<Option<nodesrv::Node>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(node_create.get_spec()).unwrap();
        let status_str = serde_json::to_string(node_create.get_status()).unwrap();
        let type_meta = serde_json::to_string(node_create.get_type_meta()).unwrap();
        let object_meta = serde_json::to_string(node_create.get_object_meta()).unwrap();
        debug!("◖☩ START: node_create ");
        let rows = &conn.query(
            "SELECT * FROM insert_node_v1($1,$2,$3,$4)",
            &[
                &(spec_str as String),
                &(status_str as String),
                &(object_meta as String),
                &(type_meta as String),
            ],
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
        response.set_node_collection(node_collection, "NodeList".to_string(), "v1".to_string());
        Ok(Some(response))
    }

    pub fn node_status_update(datastore: &DataStoreConn, node: &nodesrv::Node) -> Result<()> {
        let conn = datastore.pool.get_shard(0)?;
        let id = node.get_id().parse::<i64>().unwrap();
        let status_str = serde_json::to_string(node.get_status()).unwrap();
        conn.execute(
            "SELECT set_node_status_v1($1, $2)",
            &[&id, &(status_str as String)],
        ).map_err(Error::NodeSetStatus)?;
        Ok(())
    }

    pub fn healthz_all(client: &PrometheusClient) -> Result<Option<nodesrv::NodeMetricGetResponse>> {
        //make the url randomized, by storing mocks.
        let content = client.overall("token", "59c2402c120000d2009c0a4e").map_err(
             Error::NodeList,
        )?;

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
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    node.set_id(id.to_string() as String);
    let spec_obj: nodesrv::Spec = serde_json::from_str(&spec).unwrap();
    let status_obj: nodesrv::Status = serde_json::from_str(&status).unwrap();
    node.set_spec(spec_obj);
    node.set_status(status_obj);
    node.set_created_at(created_at.to_rfc3339());
    let object_meta_obj: asmsrv::ObjectMeta = serde_json::from_str(&object_meta).unwrap();
    node.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    node.set_type_meta(type_meta_obj);
    debug!("◖☩ ASM: row_to_node =>\n{:?}", node);
    debug!("◖☩ DONE: row_to_node");
    Ok(node)
}
