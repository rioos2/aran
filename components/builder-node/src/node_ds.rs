// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{nodesrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;
use rio_net::metrics::prometheus::PrometheusClient;
use rio_net::metrics::collector::{Collector, CollectorScope};
use serde_json;
const OS: &'static str = "rioos_os_name=ubuntu";
const NODE: &'static str = "Node";
pub struct NodeDS;

impl NodeDS {
    pub fn create(datastore: &DataStoreConn, node_create: &nodesrv::Node) -> Result<Option<nodesrv::Node>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_node_v1($1,$2,$3)",
            &[
                &(node_create.get_node_ip() as String),
                &(serde_json::to_string(node_create.get_spec()).unwrap()),
                &(serde_json::to_string(node_create.get_status()).unwrap()),
            ],
        ).map_err(Error::NodeCreate)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;

            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, node_get: &asmsrv::IdGet) -> Result<Option<nodesrv::Node>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = conn.query(
            "SELECT * from get_node_v1($1)",
            &[&(node_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::NodeGet)?;
        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }
    pub fn show_by_node_ip(datastore: &DataStoreConn, node_get: &asmsrv::IdGet) -> Result<Option<nodesrv::NodeGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_nodes_by_node_ip_v1($1)",
            &[&(node_get.get_id() as String)],
        ).map_err(Error::NodeList)?;

        let mut response = nodesrv::NodeGetResponse::new();

        let mut node_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                node_collection.push(row_to_node(&row)?)
            }
            response.set_node_collection(node_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }



    pub fn list(datastore: &DataStoreConn) -> Result<Option<nodesrv::NodeGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_nodes_v1()", &[]).map_err(
            Error::NodeList,
        )?;

        let mut response = nodesrv::NodeGetResponse::new();

        let mut node_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                node_collection.push(row_to_node(&row)?)
            }
            response.set_node_collection(node_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn status_update(datastore: &DataStoreConn, node: &nodesrv::Node) -> Result<Option<nodesrv::Node>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = conn.query(
            "SELECT set_node_status_v1($1, $2)",
            &[
                &(node.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_string(node.get_status()).unwrap()),
            ],
        ).map_err(Error::NodeSetStatus)?;
        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn healthz_all(client: &PrometheusClient) -> Result<Option<nodesrv::HealthzAllGetResponse>> {
        let nodes_metric_scope: Vec<String> = vec![
            "cpu_total".to_string(),
            "ram_total".to_string(),
            "disk_total".to_string(),
        ];

        let nodes_group_scope: Vec<String> = vec!["job=prometheus".to_string(), "group=canary".to_string()];

        let scope = CollectorScope {
            metric_names: nodes_metric_scope,
            labels: nodes_group_scope,
            last_x_minutes: "".to_string(),
        };


        let mut health_checker = Collector::new(client, scope);

        let metric_response = health_checker.overall().unwrap();

        let mut coun_collection = Vec::new();
        for data in metric_response.0 {
            let lgauges: nodesrv::Counters = data.into();
            coun_collection.push(lgauges);
        }

        let mut guague = nodesrv::Guages::new();
        guague.set_title("Cumulative operations counter".to_string());
        guague.set_counters(coun_collection);


        let mut lstatistics = vec![nodesrv::NodeStatistic::new()];
        for st_data in metric_response.1 {
            lstatistics = st_data.into();
        }
        let mut statistic = nodesrv::Statistics::new();
        statistic.set_title("Statistics".to_string());
        statistic.set_nodes(lstatistics);

        let metric_scope = vec!["node_cpu".to_string()];
        let label_group: Vec<String> = vec![
            OS.to_string(),
            nodesrv::JOBS.to_string(),
            nodesrv::MODE.to_string(),
        ];

        let scope_data = CollectorScope {
            metric_names: metric_scope,
            labels: label_group,
            last_x_minutes: nodesrv::METRIC_DEFAULT_LAST_X_MINUTE.to_string(),
        };

        let mut os_checker = Collector::new(client, scope_data);
        let os_response = os_checker.metric_by_os_usage().unwrap();

        let mut metrics = nodesrv::Osusages::new();

        let all_items = os_response
            .into_iter()
            .map(|p| {
                let p1: nodesrv::Osusages = p.into();
                p1.get_items()
            })
            .collect::<Vec<_>>();

        metrics.set_items(all_items.iter().flat_map(|s| (*s).clone()).collect());
        metrics.set_title("Scale metrics ".to_owned());

        let mut res = nodesrv::HealthzAllGet::new();
        res.set_title("Command center operations".to_string());
        // res.set_gauges(guague);
        // res.set_statistics(statistic);
        res.set_osusages(metrics);

        let response: nodesrv::HealthzAllGetResponse = res.into();

        Ok(Some(response))
    }
}

fn row_to_node(row: &postgres::rows::Row) -> Result<nodesrv::Node> {
    let mut node = nodesrv::Node::new();

    let id: i64 = row.get("id");
    let node_ip: String = row.get("node_ip");
    let status: String = row.get("status");
    let spec: String = row.get("spec");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    node.set_id(id.to_string());
    node.set_node_ip(node_ip);
    node.set_spec(serde_json::from_str(&spec).unwrap());
    node.set_status(serde_json::from_str(&status).unwrap());

    let mut obj_meta = asmsrv::ObjectMeta::new();
    let mut owner_collection = Vec::new();
    let owner = asmsrv::OwnerReferences::new();
    owner_collection.push(owner);
    obj_meta.set_name(id.to_string());
    obj_meta.set_owner_references(owner_collection);
    node.set_object_meta(obj_meta);
    node.set_type_meta(asmsrv::TypeMeta::new(NODE));

    node.set_created_at(created_at.to_rfc3339());
    Ok(node)
}
