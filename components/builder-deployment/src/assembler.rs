// Copyright 2018 The Rio Advancement Inc
use builder::attacher::Attacher;
use builder::replicas::{AssembledMap, Replicas};
use builder::service::actions::{AssembledMapRule, AttachGenerator};
use builder::service::rules::ServiceRule;
use db::data_store::DataStoreConn;
use error::Result;
use protocol::api::deploy::AssemblyFactory;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct ServicesConfig {
    pub dns: String,
}

impl ServicesConfig {
    const DNS_NAMESERVER_KEY: &'static str = "rioos_sh_dns_nameservers";

    pub fn as_map(&self) -> BTreeMap<String, String> {
        vec![
            (Self::DNS_NAMESERVER_KEY.to_string(), self.dns.clone()),
        ].into_iter()
            .collect::<BTreeMap<_, String>>()
    }

    pub fn as_map_deployless(&self) -> BTreeMap<String, String> {
        vec![(Self::DNS_NAMESERVER_KEY.to_string(), self.dns.clone())]
            .into_iter()
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

        self.build_services(&Ok(amp.clone()).map(|am| (am.0, am.1, ServiceRule::Assemble)));

        Ok(amp.clone().0)
    }

    ///Returns a resassembled assembly
    /// This can be where we want to move up the desired replica count or down.
    // This applies a ServiceRuleEvent::ReAssemble
    pub fn reassemble(&self, desired_replicas: u32, current_replicas: u32, factory: &AssemblyFactory) -> Result<AssemblyFactory> {
        let amp = &self.rebuild_replicas(desired_replicas, current_replicas, factory.clone())?;

        self.build_services(&Ok(amp.clone()).map(|am| (am.0, am.1.clone(), ServiceRule::ReAssemble)));

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
        Replicas::new(&self.conn, current_replicas, desired_replicas, &factory).upto_desired()
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
