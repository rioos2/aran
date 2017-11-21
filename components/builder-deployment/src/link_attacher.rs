use link_calculator::LinkerAction;
use error::Result;
use std::collections::BTreeMap;
use protocol::servicesrv;
pub type LinkerActions = Vec<LinkerAction>;

pub struct LinkerGenerated {
    pub actions: LinkerActions,
    pub group: String,
    pub labels: BTreeMap<String, String>,
}

pub struct LinkerGenerator {
    group: String,
    labels: BTreeMap<String, String>,
}

//The Linker actions builder
impl LinkerGenerator {
    pub fn new(group: &str, labels: BTreeMap<String, String>) -> Self {
        LinkerGenerator {
            group: group.to_string(),
            labels: labels,
        }
    }

    /// Returns a `LinkerActions` representing the linkeraction for every enum
    pub fn generate(&self) -> Result<LinkerGenerated> {
        let loadbalancer = self.build_loadbalancer_action();

        let dns = self.build_dns_action();

        Ok(LinkerGenerated {
            actions: vec![loadbalancer, dns],
            group: self.group.clone(),
            labels: self.labels.clone(),
        })
    }

    /// Returns a `LinkerAction` representing the service that the deployment tried to link
    fn build_loadbalancer_action(&self) -> LinkerAction {
        let s = servicesrv::Services::new();
        LinkerAction::LoadBalancerAdd(s)
    }

    /// Returns a `LinkerAction` representing the service that the deployment tried to link
    fn build_dns_action(&self) -> LinkerAction {
        let s = servicesrv::Services::new();
        LinkerAction::DNSPeerAdd(s)
    }
}
