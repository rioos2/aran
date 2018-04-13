// Copyright 2018 The Rio Advancement Inc
use std::collections::BTreeMap;

use error::Result;

use builder::replicas::{AssembledMap, Replicas};

use builder::attacher::Attacher;
use builder::service::actions::{AttachGenerator, AssembledMapRule};
use builder::service::rules::ServiceRule;

use protocol::api::deploy::AssemblyFactory;

use db::data_store::DataStoreConn;

#[derive(Clone)]
pub struct ServicesConfig {
    pub loadbalancer_imagein: String,
    pub loadbalancer_imagename: String,
    pub loadbalancer_cpu: String,
    pub loadbalancer_mem: String,
    pub loadbalancer_disk: String,
}

impl ServicesConfig {
    const LOAD_BALANCER_IMAGEIN_KEY: &'static str = "rioos_sh_loadbalancer_imagein";
    const LOAD_BALANCER_IMAGENAME_KEY: &'static str = "rioos_sh_loadbalancer_imagename";
    const LOAD_BALANCER_CPU_KEY: &'static str = "rioos_sh_loadbalancer_cpu";
    const LOAD_BALANCER_MEM_KEY: &'static str = "rioos_sh_loadbalancer_mem";
    const LOAD_BALANCER_DISK_KEY: &'static str = "rioos_sh_loadbalancer_disk";

    pub fn as_map(&self) -> BTreeMap<String, String> {
        println!("as map function", );
        vec![
            (
                Self::LOAD_BALANCER_IMAGEIN_KEY.to_string(),
                self.loadbalancer_imagein.clone()
            ),
            (
                Self::LOAD_BALANCER_IMAGENAME_KEY.to_string(),
                self.loadbalancer_imagename.clone()
            ),
            (
                Self::LOAD_BALANCER_CPU_KEY.to_string(),
                self.loadbalancer_cpu.clone()
            ),
            (
                Self::LOAD_BALANCER_MEM_KEY.to_string(),
                self.loadbalancer_mem.clone()
            ),
            (
                Self::LOAD_BALANCER_DISK_KEY.to_string(),
                self.loadbalancer_disk.clone()
            ),
        ].into_iter()
            .collect::<BTreeMap<_, String>>()
    }
}

///
pub struct Assembler<'a> {
    conn: &'a DataStoreConn,
    config: ServicesConfig,
}

/// Responsible for assembling the declaration of assemblyfactory.
impl<'a> Assembler<'a> {
    pub fn new(conn: &'a DataStoreConn, config: &'a ServicesConfig) -> Self {
        Assembler {
            conn: conn,
            config: (*config).clone(),
        }
    }

    ///
    ///
    pub fn assemble(&self, factory: &AssemblyFactory) -> Result<AssemblyFactory> {
        let amp = &self.build_replicas(factory.clone())?;

        self.build_services(&Ok(amp.clone()).map(
            |am| (am.0, am.1, ServiceRule::Assemble),
        ));

        Ok(amp.clone().0)
    }

    ///Returns a resassembled assembly
    /// This can be where we want to move up the desired replica count or down.
    // This applies a ServiceRuleEvent::ReAssemble
    pub fn reassemble(&self, desired_replicas: u32, current_replicas: u32, factory: &AssemblyFactory) -> Result<AssemblyFactory> {
        let amp = &self.rebuild_replicas(
            desired_replicas,
            current_replicas,
            factory.clone(),
        )?;

        self.build_services(&Ok(amp.clone()).map(|am| {
            (am.0, am.1.clone(), ServiceRule::ReAssemble)
        }));

        Ok(amp.clone().0)
    }

    ///
    ///
    fn build_replicas(&self, factory: AssemblyFactory) -> AssembledMap {
        Replicas::new(&self.conn, 0, factory.get_replicas(), &factory).new_desired()
    }

    ///
    ///
    fn rebuild_replicas(&self, desired_replicas: u32, current_replicas: u32, factory: AssemblyFactory) -> AssembledMap {
        Replicas::new(&self.conn, current_replicas, desired_replicas, &factory).upto_desired(&factory.get_id())
    }

    ///
    ///
    #[allow(unused)]
    fn build_services(&self, ampr: &AssembledMapRule) {
        let actions = AttachGenerator::generate(ampr, self.config.clone());

        Attacher::new(&self.conn, Assembler::parents(&ampr)).attach(actions);
    }

    ///
    ///
    fn parents(amp: &AssembledMapRule) -> Vec<String> {
        vec![(amp.as_ref().map(|x| x.0.get_id().clone())).unwrap()]
    }
}
