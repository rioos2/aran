// Copyright (c) 2017 RioCorp Inc
//
use linker_graph::{LinkerGraph, LinkerGraphStats};
use linker_ds::LinkersDS;
use assembly_ds::AssemblyDS;
use db::data_store::DataStoreConn;
use protocol::servicesrv;
use std::collections::BTreeMap;
use protocol::asmsrv::{IdGet, LOADBALANCER, EXTERNALNAME};
use service_graph::ServiceGraph;
use error::Result;

pub struct LinkersState<'a> {
    stats: BTreeMap<String, LinkerGraphStats>,
    state: LinkerGraph,
    conn: &'a DataStoreConn,
}

impl<'a> LinkersState<'a> {
    /// Creates new `ServiceState with the assemblyfactory ids`
    /// The goal is to build a graph like below
    ///             Service id 1 => (0, node_idx1)-> (ASM1, ASM2, AS3)
    ///             Service id 2 => (2, node_idx2)-> (ASM4)
    /// This helps us cross pollinate across assembly factories.
    pub fn new(state_ids: Vec<String>, conn: &'a DataStoreConn) -> LinkersState<'a> {

        let mut graph = LinkerGraph::new();

        let linkers = state_ids
            .iter()
            .map(|id| {
                let mut id_get = IdGet::new();
                id_get.set_id(id.to_string());
                let assemblys = AssemblyDS::show_by_assemblyfactory(conn, &id_get);
                let services = LinkersDS::list_by_assembly_factory(conn, &id_get);
                services.map(|s| (s, assemblys.unwrap().unwrap().get_items()))
            })
            .collect::<Vec<_>>();
        println!(
            "********************linkers********************************8\n{:?}",
            linkers
        );

        let stats = graph.build(linkers.into_iter());

        LinkersState {
            stats: stats,
            state: graph,
            conn: &*conn,
        }
    }

    /// Return list of connected loadbalancers
    pub fn loadbalancers_connections(&self) -> Vec<&ServiceGraph> {
        self.state
            .graphs
            .get(LOADBALANCER)
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    }
    /// Return list of our connected dns
    pub fn dns_connections(&self) -> Vec<&ServiceGraph> {
        self.state
            .graphs
            .get(EXTERNALNAME)
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    }
    /// add loadbalancer links if none of the nodes exists of type LoadBalancer exists
    /// [LoadBalancer] = node_count=0
    pub fn add_loadbalancer_connection(&self, service: &servicesrv::Services) -> Result<Option<servicesrv::Services>> {
        if self.stats.get(LOADBALANCER).unwrap().node_count <= 0 {
            let data = LinkersDS::create(self.conn, service)?;
            return Ok(data);
        }
        Ok(None)
    }
    /// add dns connection if none of the nodes exists of type ExternalName
    /// [ExternalName] = node_count=0
    pub fn add_dns_connection(&self, service: &servicesrv::Services) -> Result<Option<servicesrv::Services>> {
        if self.stats.get(EXTERNALNAME).unwrap().node_count <= 0 {
            let data = LinkersDS::create(self.conn, service)?;
            return Ok(data);
        }
        Ok(None)
    }
    /*/// remove loadbalancer links if none of the edges exists of type LoadBalancer exists
    /// [LoadBalancer] = edge_count=0
    pub fn remove_loadbalancer_connection(&self, service: &servicesrv::Services) -> bool {
        if self.dns_connections().is_empty() {
            // LinkersDS::remove(self.conn, service)
        }
    }
    /// remove dns links if none of the edges exists of type ExternalName exists
    /// [ExternalName] = node_count=0
    pub fn remove_dns_connection(&self, service: &servicesrv::Services) -> bool {
        if self.dns_connections().is_empty() {
            // LinkersDS::remove(self.conn, service)
        }
    }*/
}
