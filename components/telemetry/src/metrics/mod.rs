// Copyright 2018 The Rio Advancement Inc

//! A module containing the metrics http clients
pub mod expression;
pub mod prometheus;
pub mod executer;
pub mod query;
pub mod hooks;

use chrono::naive::NaiveDateTime;
use protocol::api::node;
use serde_json;
use std::collections::BTreeMap;

pub const NODE_MEMORY_TOTAL: &'static str = "node_memory_MemTotal";
pub const NODE_MEMORY_FREE: &'static str = "node_memory_MemFree";
pub const NODE_MEMORY_BUFFER: &'static str = "node_memory_Buffers";
pub const NODE_CPU: &'static str = "node_cpu";
pub const INSTANCE: &'static str = "instance";

pub const NODE_FILE_SYSTEM_SIZE: &'static str = "node_filesystem_size";
pub const NODE_FILE_SYSTEM_FREE: &'static str = "node_filesystem_free";

pub const NODE_NETWORK_TRANSMIT_BYTES_TOTAL: &'static str = "node_network_transmit_bytes_total";
pub const NODE_NETWORK_RECEIVE_BYTES_TOTAL: &'static str = "node_network_receive_bytes_total";
pub const NODE_NETWORK_RECEIVE_ERRS_TOTAL: &'static str = "node_network_receive_errs_total";
pub const NODE_NETWORK_TRANSMIT_ERRS_TOTAL: &'static str = "node_network_transmit_errs_total";

pub const NODE_PROCESS_CPU: &'static str = "node_process_cpu";
pub const NODE_PROCESS_MEM: &'static str = "node_process_mem";

pub const NODE_DISK_MEGA_BYTES_READ: &'static str = "node_disk_mega_bytes_read";
pub const NODE_DISK_MEGA_BYTES_WRITTEN: &'static str = "node_disk_mega_bytes_written";
pub const NODE_DISK_IO_NOW: &'static str = "node_disk_io_now";
pub const NODE_DISK_MEGA_BYTES_IO_TOTAL: &'static str = "node_disk_mega_bytes_io_total";


pub const CONTAINER_CPU_USAGE_SEC_TOTAL: &'static str = "container_cpu_usage_seconds_total";
pub const CONTAINER_MEM_USAGE_BYTES: &'static str = "container_memory_usage_bytes";
pub const CONTAINER_SPEC_MEM_LIMIT_BYTES: &'static str = "container_spec_memory_limit_bytes";


pub const CONTAINER_FS_USAGE_BYTES: &'static str = "container_fs_usage_bytes";
pub const CONTAINER_FS_LIMIT_BYTES: &'static str = "container_fs_limit_bytes";

pub const RIOOS_NAME: &'static str = "rioos_os_name";


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetricResponse {
    status: StatusData,
    pub data: Vec<PromResponse>,
}

type Timestamp = f64;
type Value = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusData {
    Success,
    Error,
}

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InvalidExpression(String),
    Timeout(String),
    InvalidResponse(serde_json::Error),
    Unexpected(u16),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixItem {
    pub metric: BTreeMap<String, String>,
    pub values: Vec<Scalar>,
}
pub type Matrix = Vec<MatrixItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantVecItem {
    pub metric: BTreeMap<String, String>,
    pub value: Scalar,
}
pub type InstantVec = Vec<InstantVecItem>;

pub type Scalar = (Timestamp, Value);

pub type Str = (Timestamp, String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType", content = "result")]
#[serde(rename_all = "lowercase")]
pub enum Data {
    Matrix(Matrix),
    Vector(InstantVec),
    Scalar(Scalar),
    String(Str),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromResponse {
    pub name: String,
    pub result: Data,
    #[serde(rename = "errorType")]
    #[serde(default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

impl PromResponse {
    pub fn new() -> PromResponse {
        PromResponse {
            name: "".to_string(),
            result: Data::String((0.0, "".to_string())),
            error_type: None,
            error: None,
        }
    }
}

//convert the PromResponse into Counters value
impl Into<node::Counters> for PromResponse {
    fn into(mut self) -> node::Counters {
        let mut counters = node::Counters::new();
        counters.set_name(self.name);
        if let Data::Vector(ref mut instancevec) = self.result {
            instancevec
                .into_iter()
                .map(|x| { counters.set_counter(x.value.1.to_owned()); })
                .collect::<Vec<_>>();
        }
        counters
    }
}


//convert the PromResponse into NodeStatistic value
impl Into<Vec<node::NodeStatistic>> for PromResponse {
    fn into(mut self) -> Vec<node::NodeStatistic> {
        let mut collections = Vec::new();
        if let Data::Vector(ref mut instancevec) = self.result {
            collections = instancevec
                .into_iter()
                .map(|x| {
                    let mut node = node::NodeStatistic::new();
                    let instance = x.metric
                        .get("instance")
                        .unwrap_or(&"".to_string())
                        .to_owned();
                    let ins: Vec<&str> = instance.split("-").collect();
                    node.set_name(ins[1].to_string());
                    node.set_counter(x.value.1.to_owned());
                    node.set_id(ins[0].to_string().replace(".", "_").to_string());

                    node.set_kind("Node".to_string());
                    node.set_api_version("v1".to_string());
                    node.set_health("up".to_string());
                    node
                })
                .collect::<Vec<_>>();
        }
        collections
    }
}

//convert the PromResponse into OSUsages value
impl Into<node::OSUsages> for PromResponse {
    fn into(mut self) -> node::OSUsages {
        let mut osusage = node::OSUsages::new();
        if let Data::Matrix(ref mut instancevec) = self.result {
            let item_collection = instancevec
                .into_iter()
                .map(|x| {
                    let mut item = node::Item::new();
                    item.set_id(
                        x.metric
                            .get(node::METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID)
                            .unwrap_or(&"none".to_string())
                            .to_owned(),
                    );
                    item.set_name(
                        x.metric
                            .get("rioos_os_name")
                            .unwrap_or(&"none".to_string())
                            .to_owned(),
                    );
                    let values = x.values
                        .clone()
                        .into_iter()
                        .map(|s| {
                            let mut value_data = node::ValueData::new();
                            value_data.set_date(
                                NaiveDateTime::from_timestamp(s.0.round() as i64, 0)
                                    .to_string()
                                    .to_owned(),
                            );
                            value_data.set_value(s.1.to_owned());
                            value_data
                        })
                        .collect::<Vec<_>>();
                    item.set_values(values);
                    item
                })
                .collect::<Vec<_>>();
            osusage.set_items(item_collection);
        }
        osusage
    }
}

impl Into<BTreeMap<String, String>> for PromResponse {
    fn into(mut self) -> BTreeMap<String, String> {
        let mut data = BTreeMap::new();
        if let Data::Vector(ref mut instancevec) = self.result {
            instancevec
                .iter_mut()
                .map(|x| if x.metric
                    .get(node::METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID)
                    .is_some()
                {
                    data.insert(
                        x.metric
                            .get(node::METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID)
                            .unwrap_or(&"".to_string())
                            .to_string(),
                        x.value.1.clone(),
                    );
                } else {
                    data.insert(
                        x.metric
                            .get(node::METRIC_LBL_RIOOS_ASSEMBLY_ID)
                            .unwrap_or(&"".to_string())
                            .to_string(),
                        x.value.1.clone(),
                    );
                })
                .collect::<Vec<_>>();
        }
        data
    }
}
