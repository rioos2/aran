// Copyright 2018 The Rio Advancement Inc

//! The startup hook is responsible for setting the sensei nodes during the startup.

use clusters::models;
use db::data_store::DataStoreConn;
use error::{Error, Result};
use hooks::BeforeHook;
use protocol::api::base::{MetaFields, WhoAmITypeMeta};
use protocol::api::deploy::PHASE_RUNNING;
use protocol::api::node::{Bridge, NodeInfo, NodeStatus};
use protocol::api::schema::type_meta_url;
use protocol::api::senseis;
use rio_core::util::stat;
use rio_core::util::stat::ProbedStat;
use rio_core::util::sys;
use std::collections::BTreeMap;

pub const CAPACITY_CPU: &'static str = "cpu";
pub const CAPACITY_MEMORY: &'static str = "memory";
pub const CAPACITY_STORAGE: &'static str = "storage";

pub struct Sensei {
    conn: Box<DataStoreConn>,
}

impl Sensei {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        Sensei { conn: datastore }
    }

    fn register(&self) -> Result<()> {
        let pr = self.stats();
        let uname = try!(sys::uname());

        let mut s = senseis::Senseis::new();

        let ref mut om = s.mut_meta(s.object_meta(), uname.node_name, "rioos_system".to_string());

        // ObjectMeta and TypeMeta
        let jackie = s.who_am_i();
        s.set_meta(type_meta_url(jackie), om.clone());

        let mut status = NodeStatus::new();
        status.set_phase(PHASE_RUNNING.to_string());

        let mut ni = NodeInfo::new();
        ni.set_machine_id(uname.sys_name.clone());
        ni.set_system_uuid(uname.version);
        ni.set_kernel_version(uname.release);
        ni.set_os_image(uname.sys_name);
        ni.set_architecture(pr.get_arch());
        ni.set_bridges(
            pr.get_bridges()
                .into_iter()
                .map(|pb| Bridge::new("", &pb.get_name(), vec![], ""))
                .collect(),
        );

        let mut cap = BTreeMap::new();
        cap.insert(CAPACITY_CPU.to_string(), pr.get_cpu());
        cap.insert(CAPACITY_MEMORY.to_string(), pr.get_memory().get_total());
        cap.insert(CAPACITY_STORAGE.to_string(), pr.get_storage().get_total());
        status.set_capacity(cap);
        status.set_node_info(ni);

        s.set_node_ip(pr.get_host_address());
        s.set_status(status);

        let senseis = models::senseis::DataStore::new(&self.conn);
        senseis.create(&s)?;

        Ok(())
    }

    fn stats(&self) -> ProbedStat {
        stat::Probe::new().probe()
    }
}

impl BeforeHook for Sensei {
    fn before(&self) -> Result<()> {
        self.register()?;
        Ok(())
    }
}
