use chrono::prelude::*;
use itertools::Itertools;
use protocol::api::base::MetaFields;
use protocol::api::node;
use protocol::api::node::{Data, InstantVecItem, PromResponse};
use serde_json;
use std::collections::BTreeMap;
use std::ops::Div;

pub struct Collector {
    content: node::MetricResponse,
    statistics: Vec<node::NodeStatistic>,
}

impl Collector {
    pub fn new(content: node::MetricResponse) -> Self {
        Collector {
            content: content,
            statistics: vec![node::NodeStatistic::new()],
        }
    }

    pub fn get_reports(&mut self) -> node::HealthzAllGet {
        let mut x = node::HealthzAllGet::new();
        x.set_title("Command center operations".to_string());
        x.set_gauges(self.mk_guages());
        x.set_statistics(self.new_statistics());
        x
    }

    pub fn get_metrics(&mut self, name: &str) -> BTreeMap<String, String> {
        self.content
            .data
            .iter()
            .filter(|x| x.name == name)
            .map(|x| {
                let d: BTreeMap<String, String> = x.clone().into();
                d
            })
            .collect::<Vec<_>>()
            .iter()
            .next()
            .unwrap()
            .clone()
    }

    fn mk_guages(&self) -> node::Guages {
        self.content
            .data
            .iter()
            .filter(|x| {
                x.name == node::CAPACITY_CPU || x.name == node::CAPACITY_MEMORY || x.name == node::CAPACITY_STORAGE
            })
            .map(|x| x.clone().into())
            .collect::<Vec<_>>()
            .into()
    }

    fn new_statistics(&mut self) -> node::Statistics {
        let mut statistics = node::Statistics::new();
        statistics.set_title("Statistics".to_string());
        statistics.set_ninjas(self.mk_statistics(node::NODES[1].0));
        statistics.set_senseis(self.mk_statistics(node::NODES[0].0));
        statistics
    }

    fn mk_statistics(&mut self, name: &str) -> Vec<node::NodeStatistic> {
        let data = self.content
            .clone()
            .data
            .into_iter()
            .filter(|x| x.name == format!("{}-cpu", name))
            .map(|x| { self.statistics = x.into(); })
            .collect::<Vec<_>>();
        if self.statistics.len() <= 0 {
            return self.statistics.clone();
        }
        for x in node::NODES_METRIC_SOURCE.iter() {
            self.set_node_resources(&format!("{}-{}", name, x));
        }
        self.statistics.clone()
    }

    fn set_node_resources(&mut self, name: &str) {
        let mut process = self.content
            .clone()
            .data
            .into_iter()
            .filter(|x| x.name == name)
            .collect::<Vec<_>>()[0]
            .clone();

        let data = self.statistics
            .clone()
            .into_iter()
            .map(|mut x| if let node::Data::Vector(ref mut instancevec) =
                process.result
            {
                let mut instance_item = instancevec
                    .iter()
                    .filter(|y| {
                        let instance = y.metric
                            .get("instance")
                            .unwrap_or(&"".to_string())
                            .to_owned();
                        let ins: Vec<&str> = instance.split("-").collect();
                        x.get_id() == ins[0].to_string()
                    })
                    .collect::<Vec<_>>();
                let name: Vec<&str> = name.split("-").collect();
                match name[1] {
                    "process" => {
                        x.set_process(group_process(&mut instance_item));
                        x
                    }
                    "disk" => {
                        x.set_disk(group_disk(&mut instance_item));
                        x
                    }
                    _ => x,

                }
            } else if let node::Data::Matrix(ref mut instancevec) = process.result {
                let mut instance_item = instancevec
                    .iter()
                    .filter(|y| {
                        let instance = y.metric
                            .get("instance")
                            .unwrap_or(&"".to_string())
                            .to_owned();
                        let ins: Vec<&str> = instance.split("-").collect();
                        x.get_id() == ins[0].to_string()
                    })
                    .collect::<Vec<_>>();
                x.set_network_speed(group_network(&mut instance_item));
                x
            } else {
                return x;
            })
            .collect::<Vec<_>>();
        self.statistics = data;
    }
}

fn group_process(process: &mut Vec<&node::InstantVecItem>) -> Vec<BTreeMap<String, Vec<BTreeMap<String, String>>>> {
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

fn group_disk(disk: &mut Vec<&node::InstantVecItem>) -> Vec<BTreeMap<String, String>> {
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

fn group_network(network: &mut Vec<&node::MatrixItem>) -> Vec<node::NetworkSpeed> {
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
        .filter(|x| x.throughput.len() > 0 && x.error.len() > 0)
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
