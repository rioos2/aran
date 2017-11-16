// Copyright (c) 2017 RioCorp Inc
//

use std::collections::HashMap;
use std::cmp::Ordering;
use std::str::FromStr;
use protocol::servicesrv;
use petgraph::{Graph, Direction};
use petgraph::graph::NodeIndex;
use petgraph::algo::{is_cyclic_directed, connected_components};
use indent::ServiceIdent;
#[derive(Debug)]
pub struct Stats {
    pub node_count: usize,
    pub edge_count: usize,
    pub connected_comp: usize,
    pub is_cyclic: bool,
}

#[derive(Eq)]
struct HeapEntry {
    pkg_index: usize,
    rdep_count: usize,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &HeapEntry) -> Ordering {
        self.rdep_count.cmp(&other.rdep_count)
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &HeapEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &HeapEntry) -> bool {
        self.pkg_index == other.pkg_index
    }
}

pub struct ServiceGraph {
    service_max: usize,
    service_map: HashMap<String, (usize, NodeIndex)>,
    latest_map: HashMap<String, ServiceIdent>,
    service_names: Vec<String>,
    graph: Graph<usize, usize>,
}

impl ServiceGraph {
    pub fn new() -> Self {
        ServiceGraph {
            service_max: 0,
            service_map: HashMap::new(),
            latest_map: HashMap::new(),
            service_names: Vec::new(),
            graph: Graph::<usize, usize>::new(),
        }
    }

    fn generate_id(&mut self, name: &str) -> (usize, NodeIndex) {
        let short_name = name;

        let id = if self.service_map.contains_key(short_name) {
            let val = *self.service_map.get(short_name).unwrap();
            val
        } else {
            self.service_names.push(short_name.to_string());
            assert_eq!(self.service_names[self.service_max], short_name);

            let node_index = self.graph.add_node(self.service_max);
            self.service_map.insert(short_name.to_string(), (
                self.service_max,
                node_index,
            ));
            self.service_max = self.service_max + 1;

            (self.service_max - 1, node_index)
        };

        id
    }


    pub fn extend(&mut self, service: &servicesrv::Services) -> (usize, usize) {
        let name = format!("{}", service.get_id());
        let (srv_id, pkg_node) = self.generate_id(&name);

        assert_eq!(srv_id, pkg_node.index());

        let pkg_ident = ServiceIdent::from_str(&name).unwrap();
        let short_name = &name;

        let add_deps = if self.latest_map.contains_key(short_name) {
            let skip_update = {
                let latest = self.latest_map.get(short_name).unwrap();
                pkg_ident < *latest
            };

            if skip_update {
                false
            } else {
                let neighbors: Vec<NodeIndex> = self.graph
                    .neighbors_directed(pkg_node, Direction::Incoming)
                    .collect();
                for n in neighbors {
                    let e = self.graph.find_edge(n, pkg_node).unwrap();
                    self.graph.remove_edge(e).unwrap();
                }
                self.latest_map.insert(
                    short_name.to_string(),
                    pkg_ident.clone(),
                );
                true
            }
        } else {
            self.latest_map.insert(
                short_name.to_string(),
                pkg_ident.clone(),
            );
            true
        };

        // if add_deps {
        //     let deps = package.get_deps();
        //     for dep in deps {
        //         let depname = format!("{}", dep);
        //         let (_, dep_node) = self.generate_id(&depname);
        //         self.graph.extend_with_edges(&[(dep_node, pkg_node)]);
        //
        //         // sanity check
        //         if is_cyclic_directed(&self.graph) {
        //             warn!(
        //                 "graph is cyclic after adding {} -> {} - rolling back",
        //                 depname,
        //                 name
        //             );
        //             let e = self.graph.find_edge(dep_node, pkg_node).unwrap();
        //             self.graph.remove_edge(e).unwrap();
        //         }
        //     }
        // }

        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn search(&self, phrase: &str) -> Vec<String> {
        let v: Vec<String> = self.service_names
            .iter()
            .cloned()
            .filter(|s| s.contains(phrase))
            .collect();

        v
    }

    pub fn latest(&self) -> Vec<String> {
        self.latest_map.values().map(|x| format!("{}", x)).collect()
    }

    // Given an identifier in 'origin/name' format, returns the
    // most recent version (fully-qualified package ident string)
    pub fn resolve(&self, name: &str) -> Option<String> {
        match self.latest_map.get(name) {
            Some(ident) => Some(format!("{}", ident)),
            None => None,
        }
    }

    pub fn stats(&self) -> Stats {
        Stats {
            node_count: self.graph.node_count(),
            edge_count: self.graph.edge_count(),
            connected_comp: connected_components(&self.graph),
            is_cyclic: is_cyclic_directed(&self.graph),
        }
    }
}

#[cfg(test)]
mod test {
    use protobuf::RepeatedField;
    use super::*;

    #[test]
    fn empty_graph() {
        let mut graph = ServiceGraph::new();
        let packages = Vec::new();

        let (ncount, ecount) = graph.build(packages.into_iter());
        assert_eq!(ncount, 0);
        assert_eq!(ecount, 0);
    }

    #[test]
    fn disallow_circular_dependency() {
        let mut graph = ServiceGraph::new();
        let mut packages = Vec::new();

        let mut package1 = jobsrv::JobGraphPackage::new();
        package1.set_ident("foo/bar/1/2".to_string());
        let mut package1_deps = RepeatedField::new();
        package1_deps.push("foo/baz/1/2".to_string());
        package1.set_deps(package1_deps);
        packages.push(package1);

        let mut package2 = jobsrv::JobGraphPackage::new();
        package2.set_ident("foo/baz/1/2".to_string());
        let mut package2_deps = RepeatedField::new();
        package2_deps.push("foo/bar/1/2".to_string());
        package2.set_deps(package2_deps);
        packages.push(package2.clone());

        let (ncount, ecount) = graph.build(packages.into_iter());

        assert_eq!(ncount, 2);
        assert_eq!(ecount, 1); // only the first edge added

        let stats = graph.stats();
        assert_eq!(stats.is_cyclic, false);

        let pre_check = graph.check_extend(&package2);
        assert_eq!(pre_check, false);
    }

    #[test]
    fn pre_check_with_dep_not_present() {
        let mut graph = ServiceGraph::new();

        let mut package1 = jobsrv::JobGraphPackage::new();
        package1.set_ident("foo/bar/1/2".to_string());
        let mut package1_deps = RepeatedField::new();
        package1_deps.push("foo/baz/1/2".to_string());
        package1.set_deps(package1_deps);

        let mut package2 = jobsrv::JobGraphPackage::new();
        package2.set_ident("foo/baz/1/2".to_string());
        let mut package2_deps = RepeatedField::new();
        package2_deps.push("foo/xyz/1/2".to_string());
        package2.set_deps(package2_deps);

        let pre_check1 = graph.check_extend(&package1);
        assert_eq!(pre_check1, true);

        let (_, _) = graph.extend(&package1);

        let pre_check2 = graph.check_extend(&package2);
        assert_eq!(pre_check2, true);

        let (_, _) = graph.extend(&package2);
    }
}
