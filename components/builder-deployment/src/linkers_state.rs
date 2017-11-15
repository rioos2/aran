// // Copyright (c) 2017 RioCorp Inc
// //
// use linker_graph::LinkerGraph;
// use linker_ds::LinkersDS;
// use assembly_ds::AssemblyDS;
// use protocol::servicesrv;
// const LOADBALANCER: &'static str = "LoadBalancer";
// const EXTERNALNAME: &'static str = "ExternalName";
//
//
// impl ServiceState {
//     /// Creates new `ServiceState with the assemblyfactory ids`
//     /// The goal is to build a graph like below
//     ///             Service id 1 => (0, node_idx1)-> (ASM1, ASM2, AS3)
//     ///             Service id 2 => (2, node_idx2)-> (ASM4)
//     /// This helps us cross pollinate across assembly factories.
//     pub fn new(state_ids: Vec<af_ids>, conn: &DataStoreConn) -> ServiceState {
//
//         let mut graph = LinkerGraph::new();
//
//         let linkers = state_ids.map(|id| {
//             let assemblys = AssemblyDS::show_by_assemblyfactory(conn, &IdGet { id })?;
//             let services = LinkersDS::list_by_assembly_factory(conn, &IdGet { id })?;
//             services.map(|s| (s, assemblys.items.assemblys))
//         }); /// Vector<(service, Assemblys)>
//
//         let stats = graph.build(linkers.into_iter());
//
//         ServiceState {
//             stats: stats,
//             state: graph,
//             conn: &DataStoreConn,
//         }
//     }
//
//     /// Return list of connected loadbalancers
//     pub fn loadbalancers_connections(&self) -> Vec<Service> {
//         self.state.get(LOADBALANCER).iter().cloned().collect()
//     }
//     /// Return list of our connected dns
//     pub fn dns_connections(&self) -> Vec<Service> {
//         self.state.get(EXTERNALNAME).iter().cloned().collect()
//     }
//     /// add loadbalancer links if none of the nodes exists of type LoadBalancer exists
//     /// [LoadBalancer] = node_count=0
//     pub fn add_loadbalancer_connection(&self, service: &Service) {
//         if self.stats.get(LOADBALANCER).node_count <= 0 {
//             LinkersDS::create(conn, service)
//         }
//
//     }
//     /// add dns connection if none of the nodes exists of type ExternalName
//     /// [ExternalName] = node_count=0
//     pub fn add_dns_connection(&self, service: &Service) {
//         if self.stats.get(EXTERNALNAME).node_count <= 0 {
//             LinkersDS::create(conn, service)
//         }
//     }
//     /// remove loadbalancer links if none of the edges exists of type LoadBalancer exists
//     /// [LoadBalancer] = edge_count=0
//     pub fn remove_loadbalancer_connection(&self, service: &Service) -> bool {
//         if self.dns_connections().is_empty() {
//             LinkersDS::remove(conn, service)
//         }
//     }
//     /// remove dns links if none of the edges exists of type ExternalName exists
//     /// [ExternalName] = node_count=0
//     pub fn remove_dns_connection(&self, service: &Service) -> bool {
//         if self.dns_connections().is_empty() {
//             LinkersDS::remove(conn, service)
//         }
//     }
// }
//
// fn main() {
//     //// Usecase 1: New assemblyfactory trying to add dns service
//     let dns = servicesrv::Services::new(); //create a "From" to Service
//     let l = LinkersState::new(ids, conn);
//     l.add_dns_connection(dns);
//
//
//     //// Usecase 2: Remove
//     let lb = servicesrv::Services::new(); //create a "From" to Service
//     let s = LinkersState::new(ids, conn);
//     l.remove_loadbalancer_connection(lb);
//
//     //// Usecase 3: Remove
//     let lb = servicesrv::Services::new(); //create a "From" to Service
//     let l = LinkersState::new(ids, conn);
//     l.remove_dns_connection(lb);
// }
