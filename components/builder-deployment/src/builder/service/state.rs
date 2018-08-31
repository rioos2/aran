// Copyright 2018 The Rio Advancement Inc
//
#![allow(unused_must_use)]

use std::collections::BTreeMap;

use error::Result;

use protocol::api::base::IdGet;
use protocol::api::linker;

use builder::service::graph::ServiceGraph;
use builder::service::tree::{ServiceTree, ServiceTreeStats};
use builder::service::{SERVICE_TYPE_EXTERNALNAME, SERVICE_TYPE_LOADBALANCER};

use db::data_store::DataStoreConn;
use models::{assembly, service};

pub struct LinkersState<'a> {
    _state: ServiceTree,
    stats: BTreeMap<String, ServiceTreeStats>,
    conn: &'a DataStoreConn,
}

impl<'a> LinkersState<'a> {
    /// Creates new `ServiceState with the assemblyfactory ids`
    /// The goal is to build a graph like below
    ///             Service id 1 => (0, node_idx1)-> (ASM1, ASM2, AS3)
    ///             Service id 2 => (2, node_idx2)-> (ASM4)
    /// This helps us cross pollinate across assembly factories.
    pub fn new(state_ids: Vec<String>, conn: &'a DataStoreConn) -> LinkersState<'a> {
        let mut graph = ServiceTree::new();

        let linkers = state_ids
            .iter()
            .map(|id| {
                let id_get = IdGet::with_id(id.to_string());
                let assemblys = try!(assembly::DataStore::new(conn).show_by_assemblyfactory(&id_get,));
                let services = try!(service::DataStore::list_by_assembly_factory(conn, &id_get));
                Ok(services.map(|s| (s, assemblys.unwrap())))
            })
            .collect::<Vec<_>>();

        let stats = graph.build(linkers.into_iter());

        LinkersState {
            _state: graph,
            stats: stats,
            conn: &*conn,
        }
    }

    /// Return list of connected loadbalancers
    pub fn _loadbalancers_connections(&self) -> Vec<&ServiceGraph> {
        self._state.graphs.get(SERVICE_TYPE_LOADBALANCER).iter().cloned().collect::<Vec<_>>()
    }
    /// Return list of our connected dns
    pub fn _dns_connections(&self) -> Vec<&ServiceGraph> {
        self._state.graphs.get(SERVICE_TYPE_EXTERNALNAME).iter().cloned().collect::<Vec<_>>()
    }
    /// add loadbalancer links if none of the nodes exists of type LoadBalancer exists
    /// [LoadBalancer] = node_count=0

    pub fn add_loadbalancer_connection(&self, service: &linker::Services) -> Result<Option<linker::Services>> {
        if self.stats.get(SERVICE_TYPE_LOADBALANCER).unwrap().node_count <= 0 {
            service::DataStore::create(self.conn, &service)?;
        }
        Ok(None)
    }
    /// add dns connection if none of the nodes exists of type ExternalName
    /// [ExternalName] = node_count=0
    pub fn add_dns_connection(&self, service: &linker::Services) -> Result<Option<linker::Services>> {
        if self.stats.get(SERVICE_TYPE_EXTERNALNAME).unwrap().node_count <= 0 {
            service::DataStore::create(self.conn, &service)?;
        }
        Ok(None)
    }
    /*/// remove loadbalancer links if none of the edges exists of type LoadBalancer exists
    /// [LoadBalancer] = edge_count=0
    pub fn remove_loadbalancer_connection(&self, service: &linker::Services) -> bool {
        if self.dns_connections().is_empty() {
            // ServiceDS::remove(self.conn, service)
        }
    }
    /// remove dns links if none of the edges exists of type ExternalName exists
    /// [ExternalName] = node_count=0
    pub fn remove_dns_connection(&self, service: &linker::Services) -> bool {
        if self.dns_connections().is_empty() {
            // ServiceDS::remove(self.conn, service)
        }
    }*/
}
