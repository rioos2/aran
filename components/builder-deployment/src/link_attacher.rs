use link_calculator::LinkerAction;
use error::Result;
use std::collections::BTreeMap;
use protocol::{servicesrv, asmsrv};
pub type LinkerActions = Vec<LinkerAction>;

pub struct LinkerGenerated {
    pub actions: LinkerActions,
    pub group: String,
    pub labels: BTreeMap<String, String>,
    pub af_id: Vec<String>,
}

pub struct LinkerGenerator<'a> {
    group: String,
    labels: BTreeMap<String, String>,
    af_id: Vec<String>,
    response: &'a asmsrv::AssemblyFactory,
    assembly: BTreeMap<String, String>,
}

//The Linker actions builder
impl<'a> LinkerGenerator<'a> {
    pub fn new(assemblyfactory: &'a asmsrv::AssemblyFactory, assembly: BTreeMap<String, String>) -> Self {
        LinkerGenerator {
            group: assemblyfactory.get_plan_data().get_group_name(),
            labels: assemblyfactory.get_object_meta().get_labels().clone(),
            af_id: vec![assemblyfactory.get_id()],
            response: &*assemblyfactory,
            assembly: assembly,
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
            af_id: self.af_id.clone(),
        })
    }

    /// Returns a `LinkerAction` representing the service that the deployment tried to link
    fn build_loadbalancer_action(&self) -> LinkerAction {
        let mut s: servicesrv::Services = self.response.clone().into();
        let mut selector = BTreeMap::new();
        selector.insert(
            servicesrv::RIO_ASM_FAC_ID.to_string(),
            self.response.get_id(),
        );
        s.set_spec(servicesrv::Spec::new(
            selector,
            servicesrv::LOADBALANCER,
            "",
            BTreeMap::new(),
            BTreeMap::new(),
        ));
        LinkerAction::LoadBalancerAdd(s)
    }

    /// Returns a `LinkerAction` representing the service that the deployment tried to link
    fn build_dns_action(&self) -> LinkerAction {
        let mut s: servicesrv::Services = self.response.clone().into();
        let mut selector = BTreeMap::new();
        selector.insert(
            servicesrv::RIO_ASM_FAC_ID.to_string(),
            self.response.get_id(),
        );
        s.set_spec(servicesrv::Spec::new(
            selector,
            servicesrv::EXTERNALNAME,
            "",
            self.assembly.clone(),
            BTreeMap::new(),
        ));
        LinkerAction::DNSPeerAdd(s)
    }
}
