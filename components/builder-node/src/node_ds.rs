// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::node;
use protocol::api::base::{IdGet, MetaFields};

use rio_net::metrics::prometheus::PrometheusClient;
use rio_net::metrics::collector::{Collector, CollectorScope};

use serde_json;

use postgres;
use db::data_store::DataStoreConn;

use super::{NodeOutput, NodeOutputList};

const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";

pub struct NodeDS;

impl NodeDS {
    pub fn create(datastore: &DataStoreConn, node_create: &node::Node) -> NodeOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_node_v1($1,$2,$3,$4,$5)",
            &[
                &(node_create.get_node_ip() as String),
                &(serde_json::to_value(node_create.get_spec()).unwrap()),
                &(serde_json::to_value(node_create.get_status()).unwrap()),
                &(serde_json::to_value(node_create.object_meta()).unwrap()),
                &(serde_json::to_value(node_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::NodeCreate)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;

            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, node_get: &IdGet) -> NodeOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * from get_node_v1($1)",
            &[&(node_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::NodeGet)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn show_by_node_ip(datastore: &DataStoreConn, node_get: &IdGet) -> NodeOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_nodes_by_node_ip_v1($1)",
            &[&(node_get.get_id() as String)],
        ).map_err(Error::NodeList)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_node(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_blank(datastore: &DataStoreConn) -> NodeOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_nodes_v1()", &[]).map_err(
            Error::NodeList,
        )?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_node(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn status_update(datastore: &DataStoreConn, upd: &node::NodeStatusUpdate) -> NodeOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_node_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::NodeSetStatus)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn healthz_all(client: &PrometheusClient) -> Result<Option<node::HealthzAllGetResponse>> {
        // collect the overall usage of memory mteric of the all node
        let memory_scope = collect_scope(
            vec![
                "node_memory_MemTotal".to_string(),
                "node_memory_MemFree".to_string(),
                "node_memory_Buffers".to_string(),
                "node_memory_Cached".to_string(),
            ],
            vec![node::NODE_JOBS.to_string()],
            "",
            "",
        );
        let memory_response = Collector::new(client, memory_scope).average_memory()?;

        // collect the overall usage of memory mteric of the all node
        let disk_scope = collect_scope(
            vec![
                "node_filesystem_size".to_string(),
                "node_filesystem_free".to_string(),
            ],
            vec![node::NODE_JOBS.to_string()],
            "",
            "",
        );
        let disk_response = Collector::new(client, disk_scope).average_disk()?;

        //collect the average node cpu and statistic of each node
        let node_scope = collect_scope(
            vec!["node_cpu".to_string()],
            vec![node::NODE_JOBS.to_string(), node::IDLEMODE.to_string()],
            METRIC_DEFAULT_LAST_X_MINUTE,
            "instance",
        );

        let mut health_checker = Collector::new(client, node_scope);
        let metric_response = health_checker.overall_node_cpu()?;

        //collect the average node cpu  of  os
        let os_scope = collect_scope(
            vec!["node_cpu".to_string()],
            vec![node::ASSEMBLY_JOBS.to_string(), node::IDLEMODE.to_string()],
            METRIC_DEFAULT_LAST_X_MINUTE,
            "rioos_os_name",
        );

        let mut os_checker = Collector::new(client, os_scope);

        let os_response_data = os_checker.overall_node_cpu()?;

        let os_response = os_checker.metric_by_os_usage()?;

        //Generete the collected prometheus data as HealthzAllGetResponse

        let mut coun_collection = Vec::new();

        // memory count
        memory_response
            .into_iter()
            .map(|x| {
                let g1: node::Counters = x.into();
                coun_collection.push(g1);
            })
            .collect::<Vec<_>>();

        // cpu count
        metric_response
            .0
            .into_iter()
            .map(|x| {
                let g2: node::Counters = x.into();
                coun_collection.push(g2);
            })
            .collect::<Vec<_>>();

        // disk count
        disk_response
            .into_iter()
            .map(|x| {
                let g3: node::Counters = x.into();
                coun_collection.push(g3);
            })
            .collect::<Vec<_>>();

        //Statistics metric of the each node
        let mut lstatistics = vec![node::NodeStatistic::new()];
        if metric_response.1.len() > 0 {
            metric_response
                .1
                .into_iter()
                .map(|x| { lstatistics = x.into(); })
                .collect::<Vec<_>>();
        }
        let mut node = node::NodeStatistic::new();
        node.set_kind("Node".to_string());
        node.set_api_version("v1".to_string());
        lstatistics = vec![node];

        let mut guages = node::Guages::new();
        guages.set_title("Cumulative operations counter".to_string());
        guages.set_counters(coun_collection);

        let mut statistics = node::Statistics::new();
        statistics.set_title("Statistics".to_string());
        statistics.set_nodes(lstatistics);

        let mut metrics = node::Osusages::new();

        let all_items = os_response
            .into_iter()
            .map(|p| {
                let p1: node::Osusages = p.into();
                p1.get_items()
            })
            .collect::<Vec<_>>();

        let mut os_collection = Vec::new();

        os_response_data
            .0
            .into_iter()
            .map(|x| {
                let g2: node::Counters = x.into();
                os_collection.push(g2);
            })
            .collect::<Vec<_>>();

        metrics.set_items(all_items.iter().flat_map(|s| (*s).clone()).collect());
        metrics.set_title("Scale metrics ".to_owned());
        metrics.set_cumulative(os_collection[0].clone());

        let mut res = node::HealthzAllGet::new();
        res.set_title("Command center operations".to_string());
        res.set_gauges(guages);
        res.set_statistics(statistics);
        res.set_osusages(metrics);

        let response: node::HealthzAllGetResponse = res.into();

        Ok(Some(response))
    }
}

fn collect_scope(metric_scope: Vec<String>, labels: Vec<String>, duration: &str, avg_by: &str) -> CollectorScope {
    CollectorScope {
        metric_names: metric_scope,
        labels: labels,
        last_x_minutes: duration.to_string(),
        avg_by_name: avg_by.to_string(),
    }
}

fn row_to_node(row: &postgres::rows::Row) -> Result<node::Node> {
    let mut node = node::Node::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    node.set_id(id.to_string());
    node.set_node_ip(row.get("node_ip"));
    node.set_spec(serde_json::from_value(row.get("spec")).unwrap());
    node.set_status(serde_json::from_value(row.get("status")).unwrap());
    node.set_created_at(created_at.to_rfc3339());
    Ok(node)
}
