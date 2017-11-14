// Copyright (c) 2017 RioCorp Inc
//

use std::str::FromStr;
use std::iter::Iterator;
use std::collections::HashMap;
use protocol::jobsrv;
use ServiceGraph;

pub struct LinkerGraphStats {
    pub target: &str,
    pub node_count: usize,
    pub edge_count: usize,
}

pub struct LinkerGraph {
    graphs: HashMap<&str, ServiceGraph>,
}

impl LinkerGraph {
    pub fn new() -> Self {
        let mut graphs = HashMap::new();

        for target_str in &[LoadBalancer, ExternalName] {
            graphs.insert(target_str, ServiceGraph::new());
        }

        LinkerGraph { graphs: graphs }
    }

    pub fn graph(&self, target_str: &str) -> Option<&ServiceGraph> {
        self.graphs.get(&target)
    }

    pub fn graph_mut(&mut self, target_str: &str) -> Option<&mut ServiceGraph> {
        self.graphs.get_mut(&target)
    }

    pub fn build<T>(&mut self, services: T) -> Map<&str, LinkerGraphStats>
    where
        T: Iterator<Item = servicesrv::Service>,
    {
        for s in services {
            match self.graph_mut(s.get_type()) { //LoadBalancer or ExternalName
                Some(ref mut graph) => {
                    graph.extend(&s);
                }
                None => (),
            }
        }

        let mut target_stats = BTreeMap::new();
        for (target, graph) in self.graphs.iter() {
            let stats = graph.stats();
            let ts = LinkerGraphStats {
                target: target.clone(),
                node_count: stats.node_count,
                edge_count: stats.edge_count,
            };
            target_stats.insert(target.clone(), ts);
        }

        target_stats
    }
}
