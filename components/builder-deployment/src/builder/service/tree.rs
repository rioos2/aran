// Copyright 2018 The Rio Advancement Inc
//
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::iter::Iterator;

use error::Result;

use protocol::api::{linker, deploy};

use builder::service::{SERVICE_TYPE_LOADBALANCER, SERVICE_TYPE_EXTERNALNAME};
use builder::service::graph::ServiceGraph;

pub struct ServiceTreeStats {
    pub target: String,
    pub node_count: usize,
    pub edge_count: usize,
}

pub struct ServiceTree {
    pub graphs: HashMap<String, ServiceGraph>,
}

impl ServiceTree {
    pub fn new() -> Self {
        let mut graphs = HashMap::new();

        for target_str in &[SERVICE_TYPE_LOADBALANCER, SERVICE_TYPE_EXTERNALNAME] {
            graphs.insert(target_str.to_string(), ServiceGraph::new());
        }

        ServiceTree { graphs: graphs }
    }

    pub fn _graph(&self, target_str: &str) -> Option<&ServiceGraph> {
        self.graphs.get(target_str)
    }

    pub fn graph_mut(&mut self, target_str: &str) -> Option<&mut ServiceGraph> {
        self.graphs.get_mut(target_str)
    }

    pub fn build<T>(&mut self, services: T) -> BTreeMap<String, ServiceTreeStats>
    where
        T: Iterator<Item = Result<Option<(Vec<linker::Services>, Vec<deploy::Assembly>)>>>,
    {
        for s in services {
            for data in s.unwrap().unwrap_or((vec![], vec![])).0 {
                match self.graph_mut(&data.get_spec().get_service_type()) {
                    //LoadBalancer or ExternalName
                    Some(ref mut graph) => {
                        graph.extend(&data);
                    }
                    None => (),
                }
            }
        }

        let mut target_stats = BTreeMap::new();
        for (target, graph) in self.graphs.iter() {
            let stats = graph.stats();
            let ts = ServiceTreeStats {
                target: target.clone(),
                node_count: stats.node_count,
                edge_count: stats.edge_count,
            };
            target_stats.insert(target.clone(), ts);
        }

        target_stats
    }
}
