// Copyright 2018 The Rio Advancement Inc

//! A module containing the health insight for the datacenter

use super::expression::*;

use super::super::error::{self, Result};

use chrono::prelude::*;
use metrics::prometheus::PrometheusClient;
use protocol::api::node;
use protocol::api::node::{MetricResponse, BuildQuery, QueryBuilder};

use serde_json;
use std::collections::BTreeMap;
use std::ops::Div;

const ASSEMBLY_JOBS: &'static str = "job=rioos_sh_machines";

//Rioos prometheus tool automatically allocated "rioos-nodes" job, so we use it
const IDLEMODE: &'static str = "mode=idle";

const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";

const NETWORK_DEFAULT_LAST_X_MINUTE: &'static str = "[1m]";

const METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID: &'static str = "rioos_assemblyfactory_id";
const METRIC_LBL_RIOOS_ASSEMBLY_ID: &'static str = "rioos_assembly_id";


#[derive(Clone)]
pub struct QueryMaker<'a> {
    client: &'a PrometheusClient,
    querys: Vec<QueryBuilder>,
}

#[derive(Clone)]
struct QueryProperties {
    pub metric_names: Vec<String>,
    pub labels: Vec<String>,
    pub last_x_minutes: String,
    pub avg_by_name: String,
}

impl<'a> QueryMaker<'a> {
    pub fn new(prom: &'a PrometheusClient) -> Self {
        QueryMaker {
            client: &*prom,
            querys: vec![],
        }
    }

    pub fn set_overall_query(&mut self) {
        //query for ram guage
        self.average_memory_query(collect_properties(
            vec![
                "node_memory_MemTotal".to_string(),
                "node_memory_MemFree".to_string(),
                "node_memory_Buffers".to_string(),
            ],
            vec![node::NODE_JOBS.to_string()],
            "",
            "",
        ));
        //query for cpu guage
        self.avg_cpu_query(collect_properties(
            vec!["node_cpu".to_string()],
            vec![node::NODE_JOBS.to_string(), IDLEMODE.to_string()],
            METRIC_DEFAULT_LAST_X_MINUTE,
            "instance",
        ));
        //query for disk guage
        self.average_disk_query(collect_properties(
            vec![
                "node_filesystem_size".to_string(),
                "node_filesystem_free".to_string(),
            ],
            vec![node::NODE_JOBS.to_string()],
            "",
            "",
        ));
        //statistics for ninja and senseis
        for x in node::NODES.iter() {
            self.node_cpu_query(
                collect_properties(
                    vec!["node_cpu".to_string()],
                    vec![x.1.to_string(), IDLEMODE.to_string()],
                    METRIC_DEFAULT_LAST_X_MINUTE,
                    "instance",
                ),
                &format!("{}-cpu", x.0),
            );

            self.node_network_query(
                collect_properties(
                    vec![
                        "node_network_transmit_bytes_total".to_string(),
                        "node_network_receive_bytes_total".to_string(),
                        "node_network_receive_errs_total".to_string(),
                        "node_network_transmit_errs_total".to_string(),
                    ],
                    vec![x.1.to_string()],
                    NETWORK_DEFAULT_LAST_X_MINUTE,
                    "",
                ),
                &format!("{}-{}", x.0, node::NODES_METRIC_SOURCE[2]),
            );

            self.node_process_query(
                collect_properties(
                    vec![
                        "node_process_cpu".to_string(),
                        "node_process_mem".to_string(),
                    ],
                    vec![x.1.to_string()],
                    "",
                    "",
                ),
                &format!("{}-{}", x.0, node::NODES_METRIC_SOURCE[0]),
            );

            self.node_network_query(
                collect_properties(
                    vec![
                        "node_disk_mega_bytes_read".to_string(),
                        "node_disk_mega_bytes_written".to_string(),
                        "node_disk_io_now".to_string(),
                        "node_disk_mega_bytes_io_total".to_string(),
                    ],
                    vec![x.1.to_string()],
                    "",
                    "",
                ),
                &format!("{}-{}", x.0, node::NODES_METRIC_SOURCE[1]),
            );
        }
    }

    pub fn set_assembly_cpu_query(&mut self, id: &str) {
        self.node_cpu_query(
            collect_properties(
                vec!["node_cpu".to_string()],
                vec![
                    format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLY_ID, id).to_string(),
                    ASSEMBLY_JOBS.to_string(),
                    IDLEMODE.to_string(),
                ],
                METRIC_DEFAULT_LAST_X_MINUTE,
                "rioos_assembly_id",
            ),
            node::CAPACITY_CPU,
        );
    }

    pub fn set_container_query(&mut self, id: &str) {
        self.container_cpu_query(collect_properties(
            vec!["container_cpu_usage_seconds_total".to_string()],
            vec![
                format!(
                    "{}={}",
                    METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID,
                    id
                ).to_string(),
            ],
            METRIC_DEFAULT_LAST_X_MINUTE,
            METRIC_LBL_RIOOS_ASSEMBLY_ID,
        ));

        self.container_query(
            collect_properties(
                vec![
                    "container_memory_usage_bytes".to_string(),
                    "container_spec_memory_limit_bytes".to_string(),
                ],
                vec![
                    format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID, id).to_string(),
                ],
                "",
                "",
            ),
            node::CAPACITY_MEMORY,
        );
        self.container_query(
            collect_properties(
                vec![
                    "container_fs_usage_bytes".to_string(),
                    "container_fs_limit_bytes".to_string(),
                ],
                vec![
                    format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID, id).to_string(),
                ],
                "",
                "",
            ),
            node::CAPACITY_STORAGE,
        );
    }



    //collect the overall memory of all nodes
    fn average_memory_query(&mut self, scope: QueryProperties) {
        let avg = Functions::Sum(AvgInfo {
            operator: Operators::Sum(SumInfo {
                labels: scope.labels.clone(),
                metric: scope.metric_names.clone(),
                total: "sum".to_string(),
            }),
        });
        self.querys.push(QueryBuilder::with_name_query(
            node::CAPACITY_MEMORY.to_string(),
            format!(
                "{}",
                MetricQueryBuilder::new(MetricQuery {
                    functions: avg,
                    by: "".to_string(),
                })
            ),
        ));
    }
    // collect the average data for the cpu usage from prometheus
    fn avg_cpu_query(&mut self, scope: QueryProperties) {
        for metric_name in scope.metric_names.iter() {
            let avg = Functions::Avg(AvgInfo {
                operator: Operators::IRate(IRateInfo {
                    labels: scope.labels.clone(),
                    metric: metric_name.to_string(),
                    last_x_minutes: scope.last_x_minutes.clone(),
                }),
            });
            self.querys.push(QueryBuilder::with_name_query(
                node::CAPACITY_CPU.to_string(),
                format!(
                    "avg(100 - ({} * 100))",
                    MetricQueryBuilder::new(MetricQuery {
                        functions: avg,
                        by: format!("avg by ({})", scope.avg_by_name.clone()),
                    })
                ),
            ));
        }
    }
    //collect the overall disk of all nodes
    fn average_disk_query(&mut self, scope: QueryProperties) {
        let avg = Functions::SumDisk(AvgInfo {
            operator: Operators::SumDisk(SumInfo {
                labels: scope.labels.clone(),
                metric: scope.metric_names.clone(),
                total: "sum".to_string(),
            }),
        });
        self.querys.push(QueryBuilder::with_name_query(
            node::CAPACITY_STORAGE.to_string(),
            format!(
                "{}",
                MetricQueryBuilder::new(MetricQuery {
                    functions: avg,
                    by: "".to_string(),
                })
            ),
        ));
    }

    // collect the average data for the cpu usage from prometheus
    fn node_cpu_query(&mut self, scope: QueryProperties, name: &str) {
        for metric_name in scope.metric_names.iter() {
            let avg = Functions::Avg(AvgInfo {
                operator: Operators::IRate(IRateInfo {
                    labels: scope.labels.clone(),
                    metric: metric_name.to_string(),
                    last_x_minutes: scope.last_x_minutes.clone(),
                }),
            });
            self.querys.push(QueryBuilder::with_name_query(
                name.to_string(),
                format!(
                    "100 - ({} * 100)",
                    MetricQueryBuilder::new(MetricQuery {
                        functions: avg,
                        by: format!("avg by ({})", scope.avg_by_name.clone()),
                    })
                ),
            ));
        }
    }

    fn node_network_query(&mut self, scope: QueryProperties, name: &str) {
        let network_query = Functions::Network(AvgInfo {
            operator: Operators::Network(SumInfo {
                labels: scope.labels.clone(),
                metric: scope.metric_names.clone(),
                total: scope.last_x_minutes.clone(),
            }),
        });
        self.querys.push(QueryBuilder::with_name_query(
            name.to_string(),
            format!("{}", network_query),
        ));
    }

    fn node_process_query(&mut self, scope: QueryProperties, name: &str) {
        let process_query = Functions::Network(AvgInfo {
            operator: Operators::Process(SumInfo {
                labels: scope.labels.clone(),
                metric: scope.metric_names.clone(),
                total: scope.last_x_minutes.clone(),
            }),
        });
        self.querys.push(QueryBuilder::with_name_query(
            name.to_string(),
            format!("{}", process_query),
        ));
    }

    fn container_cpu_query(&mut self, scope: QueryProperties) {
        for metric_name in scope.metric_names.iter() {
            let sum = Functions::Sum(AvgInfo {
                operator: Operators::IRate(IRateInfo {
                    labels: scope.labels.clone(),
                    metric: metric_name.to_string(),
                    last_x_minutes: scope.last_x_minutes.clone(),
                }),
            });
            self.querys.push(QueryBuilder::with_name_query(
                node::CAPACITY_CPU.to_string(),
                format!(
                    "sum by({}) ({})*100",
                    scope.avg_by_name.clone(),
                    sum
                ),
            ));
        }
    }

    fn container_query(&mut self, scope: QueryProperties, name: &str) {
        let mut q = vec![];
        for metric_name in scope.metric_names.iter() {
            let sum = Operators::NoOp(IRateInfo {
                labels: scope.labels.clone(),
                metric: metric_name.to_string(),
                last_x_minutes: scope.last_x_minutes.clone(),
            });
            q.push(sum);
        }
        self.querys.push(QueryBuilder::with_name_query(
            name.to_string(),
            format!("{}/{}*100", q[0], q[1]),
        ));
    }

    pub fn pull_metrics(&self) -> Result<MetricResponse> {
        let res = self.client.pull_metrics(
            BuildQuery::with_querys(self.querys.clone()),
        )?;
        Ok(res)
    }
}

fn collect_properties(metric_scope: Vec<String>, labels: Vec<String>, duration: &str, avg_by: &str) -> QueryProperties {
    QueryProperties {
        metric_names: metric_scope,
        labels: labels,
        last_x_minutes: duration.to_string(),
        avg_by_name: avg_by.to_string(),
    }
}
