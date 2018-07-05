// Copyright 2018 The Rio Advancement Inc
//

//! The actual attacher that attaches actions
//!

#![allow(unused_must_use)]

use error::Result;

use super::service::state::LinkersState;
use protocol::api::linker;

use builder::service::actions::ServiceAttachActions;

use db::data_store::DataStoreConn;

/// The adjuster that is responsible for adjusting the serviceattachaction. This is termed
/// as adjuster since this is like "maybe  create a service".
pub trait AttachAdjuster {
    fn do_adjust(&self, deplink: &ServiceAttachAction) -> Result<Option<linker::Services>>;
}

/// The attachable service actions as enum.
pub enum ServiceAttachAction {
    LoadBalancer(linker::Services),
    InternalDNS(linker::Services),
}

///
pub struct Attacher<'a> {
    conn: &'a DataStoreConn,
    parents: Vec<String>,
}

///
impl<'a> Attacher<'a> {
    ///
    pub fn new(conn: &'a DataStoreConn, parents: Vec<String>) -> Self {
        Attacher {
            conn: conn,
            parents: parents.clone(),
        }
    }

    ///
    pub fn attach(&self, actions: ServiceAttachActions) -> Result<()> {
        actions.into_iter().for_each(|x| {
            self.do_adjust(&x);
        });
        Ok(())
    }

    //Call the ServiceTree and do add_loadbalancer_conenction
    fn attach_loadbalancer(&self, data: &linker::Services) -> Result<Option<linker::Services>> {
        LinkersState::new(self.parents.clone(), &self.conn).add_loadbalancer_connection(data)
    }

    //Call the ServiceTree and do add_loadbalancer_conenction
    fn attach_internal_dns(&self, data: &linker::Services) -> Result<Option<linker::Services>> {
        LinkersState::new(self.parents.clone(), &self.conn).add_dns_connection(data)
    }
}

impl<'a> AttachAdjuster for Attacher<'a> {
    fn do_adjust(&self, link: &ServiceAttachAction) -> Result<Option<linker::Services>> {
        return match link {
            &ServiceAttachAction::LoadBalancer(ref ax) => self.attach_loadbalancer(&ax),
            &ServiceAttachAction::InternalDNS(ref ax) => self.attach_internal_dns(&ax),
        };
    }
}
