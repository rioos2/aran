// use protocol::{servicesrv, asmsrv};
// use linkers_state::LinkersState;
// use db::data_store::DataStoreConn;
// use error::{Result, Error};
//
//
// pub trait Linkerable {
//     fn hasAbility(data: &str) -> bool;
//     fn loadbalancer(&self) -> Result<Option<servicesrv::Services>>;
//     fn dns(&self) -> Result<Option<servicesrv::Services>>;
//     fn handle_add_lb(&self) -> Result<Option<servicesrv::Services>>;
//     fn handle_add_dns(&self) -> Result<Option<servicesrv::Services>>;
// }
//
// pub enum LinkerTypeActions {
//     LoadBalancerAdd(servicesrv::Services),
//     LoadBalancerDelete(servicesrv::Services),
//     LoadBalancerUpdate(servicesrv::Services),
//
//     DNSPeerAdd(servicesrv::Services),
//     DNSPeerDelete(servicesrv::Services),
//     DNSPeerUpdate(servicesrv::Services),
// }
//
// pub struct LinkCalculator {
//     conn: DataStoreConn,
//     af_id: Vec<String>,
//     deplink: LinkerTypeActions,
// }
//
// impl LinkCalculator {
//     fn new(conn: &DataStoreConn, af_id: Vec<String>, deplink: LinkerTypeActions) -> LinkCalculator {
//         LinkCalculator {
//             conn: conn.clone(),
//             af_id: af_id,
//             deplink: deplink,
//         }
//     }
// }
// impl Linkerable for LinkCalculator {
//     /// Verifies if the deployment has the ability to link
//     /// Write a function that takes a string and does what is needed.
//     fn hasAbility(data: &str) -> bool {
//         prefixed_for_linking(data)
//     }
//
//     ///use the parms for DeploymentLink and
//     fn loadbalancer(&self) -> Result<Option<servicesrv::Services>> {
//         match self.deplink {
//             LinkerTypeActions::LoadBalancerAdd(lx) => self.handle_add_lb(lx),
//             LinkerTypeActions::LoadBalancerDelete(address) => self.handle_del_lb(&address),
//         }
//     }
//
//     ///use the parms for DeploymentLink and
//     fn dns(&self) -> Result<Option<servicesrv::Services>> {
//         match self.deplink {
//             LinkerTypeActions::DNSPeerAdd(dx) => self.handle_add_dns(dx),
//             LinkerTypeActions::DNSPeerDelete(dx) => self.handle_del_dns(&dx),
//         }
//
//     }
//     //Call the linkergraph and do add_loadbalancer_conenction
//     fn handle_add_lb(&self) -> Result<Option<servicesrv::Services>> {
//         LinkersState::new(self.af_id, &self.conn)
//             .add_loadbalancer_connection()?;
//     }
//
//     //Call the linkergraph and do add_loadbalancer_conenction
//     fn handle_add_dns(&self) -> Result<Option<servicesrv::Services>> {
//         LinkersState::new(self.af_id, &self.conn)
//             .add_dns_connection()?;
//     }
// }
//
// //Use his to convert from something to your own Service
// // impl From<asmsrv::AssemblyFactory> for servicesrv::Services {
// //     fn from(data: asmsrv::AssemblyFactory) -> servicesrv::Services {
// //         let mut service = servicesrv::Services::new();
// //         service.set_name(data.get_id());
// //         service
// //     }
// // }
//
//
// fn prefixed_for_linking(categories: &str) -> bool {
//     let available = &["application", "app"]; // store this somewhere loadable from config.
//     //Write a regex to check if the category  matched available.
//     false
// }
