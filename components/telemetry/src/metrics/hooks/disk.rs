use super::super::*;
use itertools::Itertools;
use protocol::api::node;
use std::collections::BTreeMap;

pub struct Disk {
    statistics: Vec<node::NodeStatistic>,
    content: PromResponse,
}

impl Disk {
    pub fn new(statistics: Vec<node::NodeStatistic>, content: PromResponse) -> Self {
        Disk {
            statistics: statistics,
            content: content,
        }
    }
    pub fn get_disk(&self) -> Vec<node::NodeStatistic> {
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
                x.set_disk(group_disk(&mut instance_item));
                x
            } else {
                x
            })
            .collect::<Vec<_>>()
    }
}

fn group_disk(disk: &mut Vec<&InstantVecItem>) -> Vec<BTreeMap<String, String>> {
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
