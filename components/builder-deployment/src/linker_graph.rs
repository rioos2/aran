// Copyright (c) 2017 RioCorp Inc
//
use std::iter::Iterator;
use std::collections::HashMap;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv};
use service_graph::ServiceGraph;
use std::collections::BTreeMap;
use protocol::asmsrv::{LOADBALANCER, EXTERNALNAME};

pub struct LinkerGraphStats {
    pub target: String,
    pub node_count: usize,
    pub edge_count: usize,
}

pub struct LinkerGraph {
    pub graphs: HashMap<String, ServiceGraph>,
}

impl LinkerGraph {
    pub fn new() -> Self {
        let mut graphs = HashMap::new();

        for target_str in &[LOADBALANCER, EXTERNALNAME] {
            graphs.insert(target_str.to_string(), ServiceGraph::new());
        }

        LinkerGraph { graphs: graphs }
    }

    pub fn graph(&self, target_str: &str) -> Option<&ServiceGraph> {
        self.graphs.get(target_str)
    }

    pub fn graph_mut(&mut self, target_str: &str) -> Option<&mut ServiceGraph> {
        self.graphs.get_mut(target_str)
    }

    pub fn build<T>(&mut self, services: T) -> BTreeMap<String, LinkerGraphStats>
    where
        T: Iterator<Item = Result<(Option<servicesrv::ServicesGetResponse>, Vec<asmsrv::Assembly>)>>,
    {
        for s in services {
            for data in s.unwrap().0.unwrap().get_items() {
                match self.graph_mut(&data.get_spec().get_service_type()) { //LoadBalancer or ExternalName
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
