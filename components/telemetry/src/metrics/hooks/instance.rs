// Copyright 2018 The Rio Advancement Inc

//! The startup hook is responsible for setting the sensei nodes during the startup.

use std::collections::BTreeMap;

use super::{process, disk, network};
use super::super::*;
use metrics::hooks::BeforeMetrics;
use protocol::api::node;
use serde_json;


#[derive(Debug)]
pub struct Instance {
    name: String,
    content: BTreeMap<String, PromResponse>,
}

impl Instance {
    pub fn new(name: &str, content: BTreeMap<String, PromResponse>) -> Instance {
        Instance {
            name: name.to_string(),
            content: content,
        }
    }
    fn get_content(&mut self) -> Option<String> {
        let statistics: Vec<node::NodeStatistic> = self.content
            .get(&format!("{}-{}", self.name, CAPACITY_CPU))
            .unwrap_or(&PromResponse::new())
            .clone()
            .into();

        let statistics_with_process = process::Process::new(
            statistics,
            self.content
                .get(&format!("{}-{}", self.name, NODE_PROCESS))
                .unwrap_or(&PromResponse::new())
                .clone(),
        ).get_process();

        let statistics_with_disk = disk::Disk::new(
            statistics_with_process,
            self.content
                .get(&format!("{}-{}", self.name, NODE_DISK))
                .unwrap_or(&PromResponse::new())
                .clone(),
        ).get_disk();

        let statistics_with_network = network::Network::new(
            statistics_with_disk,
            self.content
                .get(&format!("{}-{}", self.name, NODE_NETWORK))
                .unwrap_or(&PromResponse::new())
                .clone(),
        ).get_network();

        serde_json::to_string(&statistics_with_network).ok()
    }
}

impl BeforeMetrics for Instance {
    fn before(&mut self) -> Option<String> {
        self.get_content()
    }
}
