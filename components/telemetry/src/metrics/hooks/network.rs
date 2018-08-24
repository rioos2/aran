
use super::super::*;
use chrono::prelude::*;
use itertools::Itertools;
use protocol::api::node;
use std::collections::BTreeMap;
use std::ops::Div;


pub struct Network {
    statistics: Vec<node::NodeStatistic>,
    content: node::PromResponse,
}

impl Network {
    pub fn new(statistics: Vec<node::NodeStatistic>, content: node::PromResponse) -> Self {
        Network {
            statistics: statistics,
            content: content,
        }
    }
    pub fn get_network(&self) -> Vec<node::NodeStatistic> {
        self.statistics
            .clone()
            .into_iter()
            .map(|mut x| if let node::Data::Matrix(ref mut instancevec) =
                self.content.result.clone()
            {
                let mut instance_item = instancevec
                    .iter()
                    .filter(|y| {
                        let instance = y.metric.get(INSTANCE).unwrap_or(&"".to_string()).to_owned();
                        let ins: Vec<&str> = instance.split("-").collect();
                        x.get_id() == ins.first().unwrap_or(&"").to_string()
                    })
                    .collect::<Vec<_>>();
                x.set_network_speed(group_network(&mut instance_item));
                x
            } else {
                x
            })
            .collect::<Vec<_>>()
    }
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
                    if y.metric.get("__name__").unwrap() == NODE_NETWORK_RECEIVE_BYTES_TOTAL || y.metric.get("__name__").unwrap() == NODE_NETWORK_TRANSMIT_BYTES_TOTAL {
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
