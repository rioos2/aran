// Copyright 2018 The Rio Advancement Inc

//! A module containing the health insight for the datacenter

use super::*;
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

    //Build the query to get metrics for the dashboard.
    //1. Gauges to indicate overall consumption of (CPU, RAM, DISK, GPU)
    //2. Statistics of the sensei nodes/ninja nodes
    //    - node cpu, network, disks, processes
    //3. All the OS Usage of the VPS(Digital cloud)
    pub fn build_consumption_in_datacenter(&mut self) -> Vec<QueryBuilder> {

        self.snapshot_memory_usage(collect_properties(
            vec![
                NODE_MEMORY_TOTAL.to_string(),
                NODE_MEMORY_FREE.to_string(),
                NODE_MEMORY_BUFFER.to_string(),
            ],
            vec![node::NODE_JOBS.to_string()],
            "",
            "",
        ));

        self.snapshot_cpu_usage(collect_properties(
            vec![NODE_CPU.to_string()],
            vec![node::NODE_JOBS.to_string(), IDLEMODE.to_string()],
            METRIC_DEFAULT_LAST_X_MINUTE,
            INSTANCE,
        ));

        self.snapshot_disks_usage(collect_properties(
            vec![
                NODE_FILE_SYSTEM_SIZE.to_string(),
                NODE_FILE_SYSTEM_FREE.to_string(),
            ],
            vec![node::NODE_JOBS.to_string()],
            "",
            "",
        ));

        for x in node::NODES.iter() {
            self.snapshot_cpu_usage_in_node(
                collect_properties(
                    vec![NODE_CPU.to_string()],
                    vec![x.1.to_string(), IDLEMODE.to_string()],
                    METRIC_DEFAULT_LAST_X_MINUTE,
                    INSTANCE,
                ),
                &format!("{}-cpu", x.0),
            );

            self.snapshot_disk_io_and_network_bandwidth_usage(
                collect_properties(
                    vec![
                        NODE_NETWORK_TRANSMIT_BYTES_TOTAL.to_string(),
                        NODE_NETWORK_RECEIVE_BYTES_TOTAL.to_string(),
                        NODE_NETWORK_RECEIVE_ERRS_TOTAL.to_string(),
                        NODE_NETWORK_TRANSMIT_ERRS_TOTAL.to_string(),
                    ],
                    vec![x.1.to_string()],
                    NETWORK_DEFAULT_LAST_X_MINUTE,
                    "",
                ),
                &format!("{}-{}", x.0, node::NODES_METRIC_SOURCE[2]),
            );

            self.snapshot_process_usage_in_node(
                collect_properties(
                    vec![NODE_PROCESS_CPU.to_string(), NODE_PROCESS_MEM.to_string()],
                    vec![x.1.to_string()],
                    "",
                    "",
                ),
                &format!("{}-{}", x.0, node::NODES_METRIC_SOURCE[0]),
            );

            self.snapshot_disk_io_and_network_bandwidth_usage(
                collect_properties(
                    vec![
                        NODE_DISK_MEGA_BYTES_READ.to_string(),
                        NODE_DISK_MEGA_BYTES_WRITTEN.to_string(),
                        NODE_DISK_IO_NOW.to_string(),
                        NODE_DISK_MEGA_BYTES_IO_TOTAL.to_string(),
                    ],
                    vec![x.1.to_string()],
                    "",
                    "",
                ),
                &format!("{}-{}", x.0, node::NODES_METRIC_SOURCE[1]),
            );
        }
        self.querys.clone()
    }
    //Provides the cpu usage in an assembly
    //The (total cpu - ide cpu) is returned.
    //The job name we look for is ASSEMBLY_JOBS
    pub fn snapshot_cpu_usage_in_machine(&mut self, id: &str, job: &str) -> Vec<QueryBuilder> {
        self.snapshot_cpu_usage_in_node(
            collect_properties(
                vec![NODE_CPU.to_string()],
                vec![
                    format!("{}={}", job, id).to_string(),
                    ASSEMBLY_JOBS.to_string(),
                    IDLEMODE.to_string(),
                ],
                METRIC_DEFAULT_LAST_X_MINUTE,
                job,
            ),
            node::CAPACITY_CPU,
        );
        self.querys.clone()
    }
    //Provide the cpu usage in a container
    //The (container_cpu_usage_seconds_total is returned)
    //The job name we look for is CONTAINER_JOBS
    pub fn snapshot_cpu_usage_in_contaner(&mut self, id: &str, job: &str) -> Vec<QueryBuilder> {
        self.snapshot_cpu_usage_in_container(collect_properties(
            vec![CONTAINER_CPU_USAGE_SEC_TOTAL.to_string()],
            vec![format!("{}={}", job, id).to_string()],
            METRIC_DEFAULT_LAST_X_MINUTE,
            job,
        ));

        self.snapshot_memory_and_storage_usage_in_container(
            collect_properties(
                vec![
                    CONTAINER_MEM_USAGE_BYTES.to_string(),
                    CONTAINER_SPEC_MEM_LIMIT_BYTES.to_string(),
                ],
                vec![format!("{}={}", job, id).to_string()],
                "",
                "",
            ),
            node::CAPACITY_MEMORY,
        );
        self.snapshot_memory_and_storage_usage_in_container(
            collect_properties(
                vec![
                    CONTAINER_FS_USAGE_BYTES.to_string(),
                    CONTAINER_FS_LIMIT_BYTES.to_string(),
                ],
                vec![format!("{}={}", job, id).to_string()],
                "",
                "",
            ),
            node::CAPACITY_STORAGE,
        );
        self.querys.clone()
    }

    //The query to build the snapshot usage of memory in a the datacenter
    fn snapshot_memory_usage(&mut self, scope: QueryProperties) {
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

    //The query to build the snapshot usage of cpu in a the datacenter
    fn snapshot_cpu_usage(&mut self, scope: QueryProperties) {
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
    //The query to build the snapshot usage of disk usage in a the datacenter
    fn snapshot_disks_usage(&mut self, scope: QueryProperties) {
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

    //The query to build the snapshot usage of cpu in a the node
    fn snapshot_cpu_usage_in_node(&mut self, scope: QueryProperties, name: &str) {
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
    //The query to build the snapshot network bandwidth and a disk io in a the node
    fn snapshot_disk_io_and_network_bandwidth_usage(&mut self, scope: QueryProperties, name: &str) {
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
    //The query to build the process usage in a the node
    fn snapshot_process_usage_in_node(&mut self, scope: QueryProperties, name: &str) {
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

    //The query to build the cpu usage in a the container
    fn snapshot_cpu_usage_in_container(&mut self, scope: QueryProperties) {
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
    //The query to build the storage and memory usage in a the container
    fn snapshot_memory_and_storage_usage_in_container(&mut self, scope: QueryProperties, name: &str) {
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

    pub fn pull_metrics(&self, querys: Vec<QueryBuilder>) -> Result<MetricResponse> {
        let res = self.client.pull_metrics(BuildQuery::with_querys(querys))?;
        println!("----------------------------------------------\n{:?}", res);
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
