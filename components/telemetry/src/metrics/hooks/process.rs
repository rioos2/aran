use super::super::*;
use itertools::Itertools;

use protocol::api::node;
use std::collections::BTreeMap;

pub struct Process {
    statistics: Vec<node::NodeStatistic>,
    content: PromResponse,
}

impl Process {
    pub fn new(statistics: Vec<node::NodeStatistic>, content: PromResponse) -> Self {
        Process {
            statistics: statistics,
            content: content,
        }
    }
    pub fn get_process(&self) -> Vec<node::NodeStatistic> {
        self.statistics
            .clone()
            .into_iter()
            .map(|mut x| if let Data::Vector(ref mut instancevec) =
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
                x.set_process(group_process(&mut instance_item));
                x
            } else {
                x
            })
            .collect::<Vec<_>>()
    }
}

fn group_process(process: &mut Vec<&InstantVecItem>) -> Vec<BTreeMap<String, Vec<BTreeMap<String, String>>>> {
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
