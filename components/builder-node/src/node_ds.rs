// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{nodesrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;
use rio_net::metrics::prometheus::PrometheusClient;
use rio_net::metrics::collector::Collector;
use serde_json;

pub struct NodeDS;

impl NodeDS {
    pub fn node_create(datastore: &DataStoreConn, node_create: &nodesrv::Node) -> Result<Option<nodesrv::Node>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(node_create.get_spec()).unwrap();
        let status_str = serde_json::to_string(node_create.get_status()).unwrap();

        let rows = &conn.query(
            "SELECT * FROM insert_node_v1($1,$2)",
            &[&(spec_str as String), &(status_str as String)],
        ).map_err(Error::NodeCreate)?;


        let node = row_to_node(&rows.get(0))?;

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

    //this doesn't have typemeta and objectmeta, maybe we should add it.
    pub fn healthz_all(client: &PrometheusClient) -> Result<Option<nodesrv::HealthzAllGetResponse>> {
        let mut response = nodesrv::HealthzAllGetResponse::new();
        let mut health_checker = Collector::new(client);

        let metric_response = health_checker.metrics().unwrap(); //TO-DO: you need send back the correct error.
        println!( "gauges\n------\n{:?}\n",
            metric_response.0
        );

        println!( "statistics\n----------\n{:?}\n",
            metric_response.1
        );

        //TO-DO: You need to add an Into which converts PromResponse to Gauges and PromResponse to Statistics
        // let lgauges: nodesrv::Gauges = metric_response.0;
        // let lstatistics: nodesrv::Statistics = metric_response.1;
        // response.set_gauges(lgauges);
        // response.set_statistics(lstatistics);

        Ok(Some(response))
    }
}

fn row_to_node(row: &postgres::rows::Row) -> Result<nodesrv::Node> {
    let mut node = nodesrv::Node::new();

    let id: i64 = row.get("id");
    let status: String = row.get("status");
    let spec: String = row.get("spec");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    node.set_id(id.to_string() as String);
    let spec_obj: nodesrv::Spec = serde_json::from_str(&spec).unwrap();
    let status_obj: nodesrv::Status = serde_json::from_str(&status).unwrap();
    node.set_spec(spec_obj);
    node.set_status(status_obj);

    let mut obj_meta = asmsrv::ObjectMeta::new();
    let mut owner_collection = Vec::new();
    let owner = asmsrv::OwnerReferences::new();
    owner_collection.push(owner);
    obj_meta.set_name(id.to_string() as String);
    obj_meta.set_owner_references(owner_collection);
    node.set_object_meta(obj_meta);
    let mut type_meta = asmsrv::TypeMeta::new();
    type_meta.set_kind("Node".to_string());
    type_meta.set_api_version("v1".to_string());
    node.set_type_meta(type_meta);
    node.set_created_at(created_at.to_rfc3339());
    Ok(node)
}
