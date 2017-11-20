use protocol::{servicesrv, asmsrv};
use linkers_state::LinkersState;
use db::data_store::DataStoreConn;
use error::{Result, Error};
use std::collections::BTreeMap;
use regex::RegexSet;

pub enum LinkerTypeActions {
    LoadBalancerAdd(servicesrv::Services),
    // LoadBalancerDelete(servicesrv::Services),
    // LoadBalancerUpdate(servicesrv::Services),
    DNSPeerAdd(servicesrv::Services),
    // DNSPeerDelete(servicesrv::Services),
    // DNSPeerUpdate(servicesrv::Services),
}

pub struct LinkCalculator {
    conn: DataStoreConn,
    af_id: Vec<String>,
}

impl LinkCalculator {
    pub fn new(conn: &DataStoreConn, af_id: Vec<String>) -> Self {
        LinkCalculator {
            conn: conn.clone(),
            af_id: af_id,
        }
    }
}

pub trait Linkerable {
    fn has_ability(category: &str) -> bool;
    fn need_lb(labels: BTreeMap<String, String>) -> bool;
    fn loadbalancer(&self, deplink: LinkerTypeActions) -> Result<Option<servicesrv::Services>>;
    fn dns(&self, deplink: LinkerTypeActions) -> Result<Option<servicesrv::Services>>;
    fn handle_add_lb(&self, data: &servicesrv::Services) -> Result<Option<servicesrv::Services>>;
    fn handle_add_dns(&self, data: &servicesrv::Services) -> Result<Option<servicesrv::Services>>;
}

impl Linkerable for LinkCalculator {
    /// Verifies if the deployment has the ability to link
    /// Write a function that takes a string and does what is needed.
    fn has_ability(category: &str) -> bool {
        prefixed_for_linking(category)
    }

    fn need_lb(labels: BTreeMap<String, String>) -> bool {
        labels.contains_key(asmsrv::LOADBALANCER)
    }

    ///use the parms for DeploymentLink and
    fn loadbalancer(&self, deplink: LinkerTypeActions) -> Result<Option<servicesrv::Services>> {
        match deplink {
            LinkerTypeActions::LoadBalancerAdd(ref lx) => self.handle_add_lb(&lx),
            _ => Ok(None),
            // LinkerTypeActions::LoadBalancerDelete(address) => self.handle_del_lb(&address),
        }
    }

    ///use the parms for DeploymentLink and
    fn dns(&self, deplink: LinkerTypeActions) -> Result<Option<servicesrv::Services>> {
        match deplink {
            LinkerTypeActions::DNSPeerAdd(ref dx) => self.handle_add_dns(&dx),
            _ => Ok(None),
            // LinkerTypeActions::DNSPeerDelete(dx) => self.handle_del_dns(&dx),
        }

    }
    //Call the linkergraph and do add_loadbalancer_conenction
    fn handle_add_lb(&self, data: &servicesrv::Services) -> Result<Option<servicesrv::Services>> {
        let data = LinkersState::new(self.af_id.clone(), &self.conn)
            .add_loadbalancer_connection(data)?;
        Ok(data)
    }

    //Call the linkergraph and do add_loadbalancer_conenction
    fn handle_add_dns(&self, data: &servicesrv::Services) -> Result<Option<servicesrv::Services>> {
        let data = LinkersState::new(self.af_id.clone(), &self.conn)
            .add_dns_connection(data)?;
        Ok(data)
    }
}



fn prefixed_for_linking(category: &str) -> bool {
    let set = RegexSet::new(&[r"container", r"application"]).unwrap();
    let matches: Vec<_> = set.matches(category).into_iter().collect();
    if !matches.is_empty() {
        return true;
    }
    return false;
}
