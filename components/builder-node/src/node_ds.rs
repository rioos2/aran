// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use itertools::Itertools;
use std::ops::Div;
use std::collections::BTreeMap;

use protocol::api::node;
use protocol::api::base::{IdGet, MetaFields, WhoAmITypeMeta};
use protocol::api::schema::type_meta_url;

use telemetry::metrics::prometheus::PrometheusClient;
use telemetry::metrics::collector::{Collector, CollectorScope};

use serde_json;
use rand;

use postgres;
use db::data_store::DataStoreConn;

use super::{NodeOutput, NodeOutputList};

const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";

const NETWORK_DEFAULT_LAST_X_MINUTE: &'static str = "[1m]";



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

    pub fn update(datastore: &DataStoreConn, upd_node: &node::Node) -> NodeOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_node_v1($1, $2, $3, $4, $5)",
            &[
                &(upd_node.get_id().parse::<i64>().unwrap()),
                &(upd_node.get_node_ip() as String),
                &(serde_json::to_value(upd_node.get_spec()).unwrap()),
                &(serde_json::to_value(upd_node.get_status()).unwrap()),
                &(serde_json::to_value(upd_node.object_meta()).unwrap()),
            ],
        ).map_err(Error::NodeUpdate)?;

        if rows.len() > 0 {
            let node = row_to_node(&rows.get(0))?;
            return Ok(Some(node));
        }
        Ok(None)
    }

    pub fn discovery(datastore: &DataStoreConn, ips: Vec<String>) -> NodeOutputList {
        match Self::list_blank(datastore) {
            Ok(Some(node)) => {
                let mut response = Vec::new();
                ips.iter()
                    .map(|x| {
                        node.iter()
                            .map(|y| if x.to_string() == y.get_node_ip() {
                                response.push(y.clone());
                            } else {
                                response.push(mk_node(x));
                            })
                            .collect::<Vec<_>>();
                    })
                    .collect::<Vec<_>>();
                Ok(Some(response))
            }
            Ok(None) => {
                let mut response = Vec::new();
                ips.iter()
                    .map(|x| { response.push(mk_node(x)); })
                    .collect::<Vec<_>>();
                Ok(Some(response))
            }
            Err(_err) => Ok(None),
        }
    }

    pub fn healthz_all(client: &PrometheusClient) -> Result<Option<node::HealthzAllGetResponse>> {
        //Generete the collected(gauges,statistics,os utilization) for all nodes with
        //current cpu utilization of all nodes
        //current ram utilization of all nodes
        //current disk utilization of all nodes
        let gauges_collected = get_gauges(client)?;
        let mut guages = node::Guages::new();
        guages.set_title("Cumulative operations counter".to_string());
        guages.set_counters(gauges_collected.0);
        //current statistic of each node contains(cpu,network)
        let mut statistics = node::Statistics::new();
        statistics.set_title("Statistics".to_string());
        statistics.set_nodes(get_statistics(client, gauges_collected.1)?);
        //Collect the overall utilization of os in all machines
        let os_statistics = get_os_statistics(client)?;
        let mut metrics = node::OSUsages::new();
        metrics.set_items(os_statistics.0.iter().flat_map(|s| (*s).clone()).collect());
        metrics.set_title("OS Usages".to_owned());
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

fn get_gauges(client: &PrometheusClient) -> Result<(Vec<node::Counters>, Vec<node::PromResponse>)> {
    let mut counters = Vec::new();
    // ram count
    collect_ram(client)?
        .into_iter()
        .map(|x| {
            let g1: node::Counters = x.into();
            counters.push(g1);
        })
        .collect::<Vec<_>>();
    //cpu count
    let cpu_response = collect_cpu(client)?;
    cpu_response
        .0
        .into_iter()
        .map(|x| {
            let g2: node::Counters = x.into();
            counters.push(g2);
        })
        .collect::<Vec<_>>();
    // disk count
    collect_disk(client)?
        .into_iter()
        .map(|x| {
            let g3: node::Counters = x.into();
            counters.push(g3);
        })
        .collect::<Vec<_>>();
    Ok((counters, cpu_response.1))
}

fn get_statistics(client: &PrometheusClient, cpu_nodes_collected: Vec<node::PromResponse>) -> Result<Vec<node::NodeStatistic>> {
    //Statistics metric of the each node
    if cpu_nodes_collected.len() == 0 {
        return Ok(vec![]);
    }

    let mut node_statistics = vec![node::NodeStatistic::new()];
    cpu_nodes_collected
        .into_iter()
        .map(|x| { node_statistics = x.into(); })
        .collect::<Vec<_>>();

    Ok(append_disk(
        append_process(
            append_network_speed(
                node_statistics,
                collect_network(client)?,
            )?,
            collect_process(client)?,
        )?,
        collect_disk_io(client)?,
    )?)
}

fn get_os_statistics(client: &PrometheusClient) -> Result<(Vec<Vec<node::Item>>, Vec<node::Counters>)> {
    let os_response = collect_os_usage(client)?;
    let os_usages = os_response
        .1
        .into_iter()
        .map(|p| {
            let p1: node::OSUsages = p.into();
            p1.get_items()
        })
        .collect::<Vec<_>>();

    let mut os_cpu_usages = Vec::new();
    os_response
        .0
        .into_iter()
        .map(|x| {
            let g2: node::Counters = x.into();
            os_cpu_usages.push(g2);
        })
        .collect::<Vec<_>>();

    Ok((os_usages, os_cpu_usages))
}

fn collect_ram(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
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

fn collect_disk(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
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

fn collect_cpu(client: &PrometheusClient) -> Result<(Vec<node::PromResponse>, Vec<node::PromResponse>)> {
    //collect the average node cpu and statistic of each node
    let node_scope = collect_scope(
        vec!["node_cpu".to_string()],
        vec![node::NODE_JOBS.to_string(), node::IDLEMODE.to_string()],
        METRIC_DEFAULT_LAST_X_MINUTE,
        "instance",
    );
    Ok(Collector::new(client, node_scope).overall_node_cpu()?)
}

fn collect_network(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
    //collect the network_metric for node
    let network_scope = collect_scope(
        vec![
            "node_network_transmit_bytes_total".to_string(),
            "node_network_receive_bytes_total".to_string(),
            "node_network_receive_errs_total".to_string(),
            "node_network_transmit_errs_total".to_string(),
        ],
        vec![],
        NETWORK_DEFAULT_LAST_X_MINUTE,
        "",
    );
    Ok(Collector::new(client, network_scope).network_metric()?)
}

fn collect_disk_io(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
    //collect the disk_metric for node
    let disk_scope = collect_scope(
        vec![
            "node_disk_mega_bytes_read".to_string(),
            "node_disk_mega_bytes_written".to_string(),
            "node_disk_io_now".to_string(),
            "node_disk_mega_bytes_io_total".to_string(),
        ],
        vec![],
        "",
        "",
    );
    Ok(Collector::new(client, disk_scope).disk_metric()?)
}

fn collect_process(client: &PrometheusClient) -> Result<Vec<node::PromResponse>> {
    //collect the process_metric for node
    let process_scope = collect_scope(
        vec![
            "node_process_cpu".to_string(),
            "node_process_mem".to_string(),
        ],
        vec![],
        "",
        "",
    );
    Ok(Collector::new(client, process_scope).process_metric()?)
}

fn collect_os_usage(client: &PrometheusClient) -> Result<(Vec<node::PromResponse>, Vec<node::PromResponse>)> {
    //collect the average node cpu  of  os
    let os_scope = collect_scope(
        vec!["node_cpu".to_string()],
        vec![node::ASSEMBLY_JOBS.to_string(), node::IDLEMODE.to_string()],
        METRIC_DEFAULT_LAST_X_MINUTE,
        "rioos_os_name",
    );
    Ok(Collector::new(client, os_scope).metric_by_os_usage()?)
}

fn append_network_speed(nodes: Vec<node::NodeStatistic>, mut networks: Vec<node::PromResponse>) -> Result<Vec<node::NodeStatistic>> {
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
                x.set_network_speed(group_network(&net_collection));
                x
            } else {
                return x;
            })
            .collect::<Vec<_>>(),
    )
}

fn group_network(network: &Vec<node::MatrixItem>) -> Vec<node::NetworkSpeed> {
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
            let mut net = node::NetworkDevice::new();
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
            let mut group = node::NetworkSpeed::new();
            let mut throughput: Vec<node::SpeedSummary> = vec![];
            let mut error: Vec<node::SpeedSummary> = vec![];
            x.throughput[0]
                .values
                .iter()
                .map(|y| {
                    x.throughput[1]
                        .values
                        .iter()
                        .map(|z| if y.0 == z.0 {
                            throughput.push((
                                NaiveDateTime::from_timestamp(y.0.round() as i64, 0)
                                    .format("%H:%M:%S")
                                    .to_string()
                                    .to_owned(),
                                y.1.clone().parse::<i32>().unwrap_or(0).div(1024).div(1024),
                                z.1.clone().parse::<i32>().unwrap_or(0).div(1024).div(1024),
                            ));
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
                            error.push((
                                NaiveDateTime::from_timestamp(y.0.round() as i64, 0)
                                    .format("%H:%M:%S")
                                    .to_string()
                                    .to_owned(),
                                y.1.clone().parse::<i32>().unwrap_or(0).div(1024).div(1024),
                                z.1.clone().parse::<i32>().unwrap_or(0).div(1024).div(1024),
                            ));
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


fn append_process(nodes: Vec<node::NodeStatistic>, mut process: Vec<node::PromResponse>) -> Result<Vec<node::NodeStatistic>> {
    Ok(
        nodes
            .into_iter()
            .map(|mut x| if let node::Data::Vector(ref mut instancevec) =
                process[0].clone().data
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
                x.set_process(group_process(&net_collection));
                x
            } else {
                return x;
            })
            .collect::<Vec<_>>(),
    )
}

fn append_disk(nodes: Vec<node::NodeStatistic>, mut disk: Vec<node::PromResponse>) -> Result<Vec<node::NodeStatistic>> {

    Ok(
        nodes
            .into_iter()
            .map(|mut x| if let node::Data::Vector(ref mut instancevec) =
                disk[0].clone().data
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
                x.set_disk(group_disk(&net_collection));
                x
            } else {
                return x;
            })
            .collect::<Vec<_>>(),
    )
}
fn group_disk(disk: &Vec<node::InstantVecItem>) -> Vec<BTreeMap<String, String>> {

    let merged = disk.iter()
        .flat_map(|s| s.metric.get("device"))
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

    merged
        .into_iter()
        .map(|x| {
            let mut disk_metric = BTreeMap::new();
            disk_metric.insert("name".to_string(), x.to_string());
            disk.into_iter()
                .map(|y| if x == y.metric.get("device").unwrap() {
                    disk_metric.insert(
                        y.metric
                            .get("__name__")
                            .unwrap_or(&"".to_string())
                            .to_string(),
                        y.value.clone().1,
                    );

                })
                .collect::<Vec<_>>();
            disk_metric
        })
        .collect::<_>()

}



fn group_process(process: &Vec<node::InstantVecItem>) -> Vec<BTreeMap<String, Vec<BTreeMap<String, String>>>> {
    let merged = process
        .iter()
        .flat_map(|s| s.metric.get("__name__"))
        .collect::<Vec<_>>()
        .into_iter()
        .unique()
        .collect::<Vec<_>>();

    merged
        .into_iter()
        .map(|x| {
            let mut process_metric = BTreeMap::new();
            let mut a = Vec::new();
            process
                .into_iter()
                .map(|y| if x == y.metric.get("__name__").unwrap() {
                    let mut group = BTreeMap::new();
                    group.insert(
                        "pid".to_string(),
                        y.metric.get("pid").unwrap_or(&"".to_string()).to_string(),
                    );
                    group.insert(
                        "command".to_string(),
                        y.metric
                            .get("command")
                            .unwrap_or(&"".to_string())
                            .to_string(),
                    );
                    group.insert("value".to_string(), y.value.clone().1);
                    a.push(group)
                })
                .collect::<Vec<_>>();
            process_metric.insert(x.to_string(), a);
            process_metric
        })
        .collect::<_>()
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

fn mk_node(ip: &str) -> node::Node {
    let mut node = node::Node::new();
    let jackie = node.who_am_i();
    let ref mut om = node.mut_meta(node.object_meta(), ip.to_string(), "".to_string());
    node.set_meta(type_meta_url(jackie), om.clone());
    node.set_node_ip(ip.to_string());
    node.set_id(rand::random::<u64>().to_string());
    node
}
