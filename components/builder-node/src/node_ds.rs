// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use itertools::Itertools;

use protocol::api::node;
use protocol::api::base::{IdGet, MetaFields, WhoAmITypeMeta};

use telemetry::metrics::prometheus::PrometheusClient;
use telemetry::metrics::collector::{Collector, CollectorScope};
use rio_net::http::schema::type_meta_url;

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

        let metrics = get_guages(client)?;
        let mut guages = node::Guages::new();
        guages.set_title("Cumulative operations counter".to_string());
        guages.set_counters(metrics.0);

        let mut statistics = node::Statistics::new();
        statistics.set_title("Statistics".to_string());
        statistics.set_nodes(get_statistics(client, metrics.1)?);

        let os_statistics = get_os_statistics(client)?;

        let mut metrics = node::Osusages::new();
        metrics.set_items(os_statistics.0.iter().flat_map(|s| (*s).clone()).collect());
        metrics.set_title("Scale metrics ".to_owned());
        metrics.set_cumulative(os_statistics.1[0].clone());

        let mut res = node::HealthzAllGet::new();
        res.set_title("Command center operations".to_string());
        res.set_gauges(guages);
        res.set_statistics(statistics);
        res.set_osusages(metrics);

        let response: node::HealthzAllGetResponse = res.into();

        Ok(Some(response))
    }
}


fn get_guages(client: &PrometheusClient) -> Result<(Vec<node::Counters>, Vec<node::PromResponse>)> {
    //Generete the collected prometheus data as HealthzAllGetResponse

    let mut coun_collection = Vec::new();
    // memory count
    memory_response(client)?
        .into_iter()
        .map(|x| {
            let g1: node::Counters = x.into();
            coun_collection.push(g1);
        })
        .collect::<Vec<_>>();

    let node_response = node_response(client)?;

    // cpu count
    node_response
        .0
        .into_iter()
        .map(|x| {
            let g2: node::Counters = x.into();
            coun_collection.push(g2);
        })
        .collect::<Vec<_>>();

    // disk count
    disk_response(client)?
        .into_iter()
        .map(|x| {
            let g3: node::Counters = x.into();
            coun_collection.push(g3);
        })
        .collect::<Vec<_>>();
    Ok((coun_collection, node_response.1))
}

fn get_statistics(client: &PrometheusClient, node_response: Vec<node::PromResponse>) -> Result<Vec<node::NodeStatistic>> {
    //Statistics metric of the each node
    if node_response.len() == 0 {
        let mut node = node::NodeStatistic::new();
        let jackie = node.who_am_i();
        node.set_type_meta(type_meta_url(jackie));
        return Ok(vec![node]);

    }

    let mut lstatistics = vec![node::NodeStatistic::new()];

    node_response
        .into_iter()
        .map(|x| { lstatistics = x.into(); })
        .collect::<Vec<_>>();

    Ok(node_with_network(lstatistics, network_response(client)?)?)
}

fn get_os_statistics(client: &PrometheusClient) -> Result<(Vec<Vec<node::Item>>, Vec<node::Counters>)> {
    let os_response = os_response(client)?;

    let all_items = os_response
        .1
        .into_iter()
        .map(|p| {
            let p1: node::Osusages = p.into();
            p1.get_items()
        })
        .collect::<Vec<_>>();

    let mut os_collection = Vec::new();

    os_response
        .0
        .into_iter()
        .map(|x| {
            let g2: node::Counters = x.into();
            os_collection.push(g2);
        })
        .collect::<Vec<_>>();

    Ok((all_items, os_collection))
}

fn memory_response(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
    // collect the overall usage of memory mteric of the all node
    let memory_scope = collect_scope(
        vec![
            "node_memory_MemTotal".to_string(),
            "node_memory_MemFree".to_string(),
            "node_memory_Buffers".to_string(),
        ],
        vec![node::NODE_JOBS.to_string()],
        "",
        "",
    );
    Ok(Collector::new(client, memory_scope).average_memory()?)

}

fn disk_response(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
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
    Ok(Collector::new(client, disk_scope).average_disk()?)

}

fn node_response(client: &PrometheusClient) -> Result<(Vec<node::PromResponse>, Vec<node::PromResponse>)> {
    //collect the average node cpu and statistic of each node
    let node_scope = collect_scope(
        vec!["node_cpu".to_string()],
        vec![node::NODE_JOBS.to_string(), node::IDLEMODE.to_string()],
        METRIC_DEFAULT_LAST_X_MINUTE,
        "instance",
    );

    Ok(Collector::new(client, node_scope).overall_node_cpu()?)

}

fn network_response(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
    //collect the network_metric for node
    let network_scope = collect_scope(
        vec![
            "node_network_transmit_bytes_total".to_string(),
            "node_network_receive_bytes_total".to_string(),
            "node_network_receive_errs_total".to_string(),
            "node_network_transmit_errs_total".to_string(),
        ],
        vec![],
        METRIC_DEFAULT_LAST_X_MINUTE,
        "",
    );

    Ok(Collector::new(client, network_scope).network_metric()?)

}

fn os_response(client: &PrometheusClient) -> Result<(Vec<node::PromResponse>, Vec<node::PromResponse>)> {

    //collect the average node cpu  of  os
    let os_scope = collect_scope(
        vec!["node_cpu".to_string()],
        vec![node::ASSEMBLY_JOBS.to_string(), node::IDLEMODE.to_string()],
        METRIC_DEFAULT_LAST_X_MINUTE,
        "rioos_os_name",
    );

    Ok(Collector::new(client, os_scope).metric_by_os_usage()?)
}

fn node_with_network(nodes: Vec<node::NodeStatistic>, mut networks: Vec<node::PromResponse>) -> Result<Vec<node::NodeStatistic>> {
    Ok(
        nodes
            .into_iter()
            .map(|mut x| if let node::Data::Matrix(ref mut instancevec) =
                networks[0].clone().data
            {
                let mut net_collection = Vec::new();
                instancevec
                    .iter()
                    .map(|y| if x.get_name() ==
                        y.metric.get("instance").unwrap().to_string()
                    {
                        net_collection.push(y.clone())
                    })
                    .collect::<Vec<_>>();
                x.set_network(group_network(&net_collection));
                x
            } else {
                return x;
            })
            .collect::<Vec<_>>(),
    )
}

fn group_network(network: &Vec<node::MatrixItem>) -> Vec<node::NetworkGroup> {
    let merged = network
        .iter()
        .flat_map(|s| s.metric.get("device"))
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

    let data = merged
        .into_iter()
        .map(|x| {
            let mut net = node::NetworkData::new();
            let mut a = Vec::new();
            let mut b = Vec::new();
            network
                .into_iter()
                .map(|y| if x == y.metric.get("device").unwrap() {
                    if y.metric.get("__name__").unwrap() == "node_network_receive_bytes_total" || y.metric.get("__name__").unwrap() == "node_network_transmit_bytes_total" {
                        a.push(y.clone())
                    } else {
                        b.push(y.clone())
                    }
                })
                .collect::<Vec<_>>();
            net.set_name(x.to_string());
            net.set_throughput(a);
            net.set_error(b);
            net
        })
        .collect::<Vec<_>>();

    data.iter()
        .map(|x| {
            let mut group = node::NetworkGroup::new();
            let mut throughput: Vec<node::NetworkType> = vec![];
            let mut error: Vec<node::NetworkType> = vec![];
            x.throughput[0]
                .values
                .iter()
                .map(|y| {
                    x.throughput[1]
                        .values
                        .iter()
                        .map(|z| if y.0 == z.0 {
                            throughput.push((y.0, y.1.clone(), z.1.clone()));
                        })
                        .collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
            x.error[0]
                .values
                .iter()
                .map(|y| {
                    x.error[1]
                        .values
                        .iter()
                        .map(|z| if y.0 == z.0 {
                            error.push((y.0, y.1.clone(), z.1.clone()));
                        })
                        .collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
            group.set_name(x.name.clone());
            group.set_throughput(throughput);
            group.set_error(error);
            group
        })
        .collect::<Vec<_>>()
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
