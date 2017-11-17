// use protocol::servicesrv;
//
// pub trait Linkerable {
//     fn hasAbility(&self) -> bool;
//     fn loadbalancer() -> servicesrv::Services;
//     fn dns() -> servicesrv::Services;
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
// impl Linkerable for LinkCalculator {
//     fn new() {}
//
//     /// Verifies if the deployment has the ability to link
//     /// Write a function that takes a string and does what is needed.
//     fn hasAbility() -> bool {
//         prefixed_for_linking("")
//     }
//
//     ///use the parms for DeploymentLink and
//     fn loadbalancer(parms: String, delink: DeploymentLink) {
//         match deplink {
//             DeploymentLink::LoadBalancerAdd(lx) => {
//                 self.handle_add_lb(lx);
//             }
//             DeploymentLink::LoadBalancerDelete(address) => {
//                 self.handle_del_lb(&address);
//             }
//         }
//     }
//
//     ///use the parms for DeploymentLink and
//     fn dns(parms: String, delink: DeploymentLink) {
//         match deplink {
//             DeploymentLink::DNSPeerAdd(dx) => {
//                 self.handle_add_dns(dx);
//             }
//             DeploymentLink::DNSPeerDelete(dx) => {
//                 self.handle_del_dns(&dx);
//             }
//         }
//
//     }
//     //Call the linkergraph and do add_loadbalancer_conenction
//     fn handle_add_lb(&self, ldx: String) {
//         LinkerGraph::new().add_loadbalancer_connection()
//     }
//
//     //Call the linkergraph and do add_loadbalancer_conenction
//     fn handle_add_dns(&self, ldx: String) {
//         LinkerGraph::new().add_dns_connection()
//     }
// }
//
// //Use his to convert from something to your own Service
// impl From<asmsrv::AssemblyFactory> for servicesrv::Services {
//     fn from(data: asmsrv::AssemblyFactory) -> servicesrv::Services {
//         let mut service = servicesrv::Services::new();
//         service.set_name(data.get_id());
//         service
//     }
// }
//
//
// fn prefixed_for_linking(categories: &str) -> bool {
//     let available = &["application", "app"]; // store this somewhere loadable from config.
//     //Write a regex to check if the category  matched available.
//     false
// }
