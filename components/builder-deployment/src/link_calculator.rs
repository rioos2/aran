use protocol::{servicesrv, asmsrv};
use linkers_state::LinkersState;
use db::data_store::DataStoreConn;
use error::Result;
use std::collections::BTreeMap;
use regex::RegexSet;
use link_attacher::{LinkerGenerated, LinkerActions};

pub enum LinkerAction {
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
    actions: LinkerActions,
    is_unedged: bool,
}

impl LinkCalculator {
    pub fn new(conn: &DataStoreConn, af_id: Vec<String>, linkgen: LinkerGenerated) -> Self {
        let mut l = LinkCalculator {
            conn: conn.clone(),
            af_id: af_id,
            actions: linkgen.actions,
            is_unedged: false,
        };
        l.is_unedged = l.has_ability(&linkgen.group, linkgen.labels);
        l
    }

    pub fn attach(&self) -> Result<()> {
        self.actions.iter().for_each(|x| {
            self.loadbalancer(x);
            self.dns(x);
        });
        Ok(())
    }
}

pub trait Linkerable {
    fn has_ability(&self, category: &str, labels: BTreeMap<String, String>) -> bool;
    fn loadbalancer(&self, deplink: &LinkerAction) -> Result<Option<servicesrv::Services>>;
    fn dns(&self, deplink: &LinkerAction) -> Result<Option<servicesrv::Services>>;
    fn handle_add_lb(&self, data: &servicesrv::Services) -> Result<Option<servicesrv::Services>>;
    fn handle_add_dns(&self, data: &servicesrv::Services) -> Result<Option<servicesrv::Services>>;
}

impl Linkerable for LinkCalculator {
    /// Verifies if the deployment has the ability to link
    /// Write a function that takes a string and does what is needed.
    fn has_ability(&self, category: &str, labels: BTreeMap<String, String>) -> bool {
        prefixed_for_linking(category) || labels.contains_key(asmsrv::LOADBALANCER)
    }


    ///use the parms for DeploymentLink and
    fn loadbalancer(&self, deplink: &LinkerAction) -> Result<Option<servicesrv::Services>> {
        if self.is_unedged {
            return match deplink {
                &LinkerAction::LoadBalancerAdd(ref lx) => self.handle_add_lb(&lx),
                _ => Ok(None),
                // LinkerTypeActions::LoadBalancerDelete(address) => self.handle_del_lb(&address),
            };
        }
        Ok(None)

    }


    ///use the parms for DeploymentLink and
    fn dns(&self, deplink: &LinkerAction) -> Result<Option<servicesrv::Services>> {
        match deplink {
            &LinkerAction::DNSPeerAdd(ref dx) => self.handle_add_dns(&dx),
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
        println!("**************************************************************************8");
        let data = LinkersState::new(self.af_id.clone(), &self.conn)
            .add_dns_connection(data)?;
        Ok(data)
    }
}



fn prefixed_for_linking(group_name: &str) -> bool {
    let category = group_name.split("_").collect::<Vec<_>>();
    let set = RegexSet::new(&[r"container", r"application"]).unwrap();
    let matches: Vec<_> = set.matches(category[1]).into_iter().collect();
    if !matches.is_empty() {
        return true;
    }
    return false;
}
