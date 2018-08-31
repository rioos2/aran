// Copyright 2018 The Rio Advancement Inc

//! The Stacks maker fascasde.
//! This creates STACKS_FACTORY and the following tree.
//!                    |
//!                    V
//!              o-----o-----o----o
//!              |                |
//!        ASSEMBLY_FACTORY    ASSEMBLY_FACTORY
//!            |                |
//!        ASSEMBLY...n    ASSEMBLY...n
use super::StacksFactoryOutput;
use assembler::Assembler;
use assembler::ServicesConfig;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::{Error, Result};
use models::stacksfactory;
use protocol::api::base::{ChildTypeMeta, MetaFields, Status};
use protocol::api::blueprint::PlanProperties;
use protocol::api::deploy::{AssemblyFactory, AssemblyFactorySpec, StacksFactory};
use protocol::api::schema::type_meta_url;
use rand::distributions::Alphanumeric;
use rand::{self, Rng};
use std::collections::BTreeMap;

const PRE_NAME_LEN: usize = 5;

const RIOOS_SH_BLUEPRINT_APPLIED: &'static str = "rioos_sh_blueprint_applied";

/// Builds a deployer for each type.
pub struct DeployerFactory<'a>(DeployerFactoryInner<'a>);

enum DeployerFactoryInner<'a> {
    /// Builds a new connector for each name by applying all configurations with a
    /// matching prefix. This is considered "static" because the set of configurations may
    /// not be updated dynamically.
    DynamicStacks(StacksDeployer<'a>),
}

impl<'a> DeployerFactory<'a> {
    pub fn new(conn: &'a DataStoreConn, config: &ServicesConfig) -> DeployerFactory<'a> {
        let f = StacksDeployer {
            conn: conn,
            service_config: (*config).clone(),
        };
        DeployerFactory(DeployerFactoryInner::DynamicStacks(f))
    }

    pub fn mk_stacker(&self, stacks: &StacksFactory) -> Result<StacksFactory> {
        match self.0 {
            DeployerFactoryInner::DynamicStacks(ref c) => c.mk_deploy(stacks),
            /*
            TO-DO: When you have more types open this up.
            Don't know when we'll hit this.
            _ => Err(Error::StacksFactoryInvalidType(
                "Can't invoke stacks deployer. Invoked in an incorrect way.".to_string(),
            )),*/
        }
    }
}

struct StacksDeployer<'a> {
    conn: &'a DataStoreConn,
    service_config: ServicesConfig,
}

impl<'a> StacksDeployer<'a> {
    /// Builds a new stacksfactory by applying all configurations.
    fn mk_stacks_factory(&self, stacks: &StacksFactory) -> StacksFactoryOutput {
        stacksfactory::DataStore::new(&self.conn).create(&stacks.clone())
    }

    fn mk_deploy(&self, factory: &StacksFactory) -> Result<StacksFactory> {
        match self.mk_stacks_factory(&factory) {
            Ok(Some(stacks)) => match stacks.get_spec().get_plan() {
                Some(plan) => {
                    let assembled_factorys = plan.get_plan()
                        .into_iter()
                        .map(|plan_property| {
                            let assembly_factory: AssemblyFactory = self.build_assembly_factory(&plan_property, &stacks);
                            Assembler::new(&self.conn, &self.service_config).assemble(&assembly_factory)
                        })
                        .fold(vec![], |mut acc, x| match x {
                            Ok(one_assembly_factory) => {
                                acc.push(one_assembly_factory);
                                acc
                            }
                            Err(_) => acc,
                        });

                    let f: &mut StacksFactory = &mut stacks.clone();
                    f.get_mut_spec().set_assembly_factory(assembled_factorys);

                    Ok(f.clone())
                }
                None => Err(Error::StacksFactoryInvalidType(
                    "Can't invoke stacks deployer. Invoked in an incorrect way.".to_string(),
                )),
            },
            Err(err) => Err(err),
            Ok(None) => Err(Error::Db(RecordsNotFound)),
        }
    }

    ///Build the assembly by setting up a object meta (name, account id of the parent,
    ///and its type meta from the parent)
    fn build_assembly_factory(&self, plan_property: &PlanProperties, parent: &StacksFactory) -> AssemblyFactory {
        let mut assembly_factory = AssemblyFactory::new();
        let mut spec = AssemblyFactorySpec::new();
        //Transfer resources (cpu, ram, disk)  from stackfactory to assemblyfactory
        assembly_factory.set_resources(parent.get_resources().clone());
        //Transfer secret  from stackfactory to assemblyfactory
        assembly_factory.set_secret(parent.get_secret().clone());
        //Transfer spec:toleration from stackfactory to assemblyfactory
        spec.set_tolerations(parent.get_spec().get_tolerations().to_vec());
        //Transfer spec:node_selector from stackfactory to assemblyfactory
        spec.set_node_selector(parent.get_spec().get_node_selector().clone());
        //Transfer spec:affinity from stackfactory to assemblyfactory
        spec.set_affinity(parent.get_spec().get_affinity().clone());
        //Transfer spec:restart_policy from stackfactory to assemblyfactory
        spec.set_restart_policy(parent.get_spec().get_restart_policy().to_string());
        //Don't Transfer the plan as its redundant. We can
        //set it up as None as well.
        spec.set_plan(None);
        assembly_factory.set_spec(spec);

        //Transfer the stacks plan id here.
        assembly_factory.set_plan(parent.get_plan());
        //Transfer the replicas
        assembly_factory.set_replicas(parent.get_replicas());

        //Transfer the stacks ObjectMeta (name , account_id)
        let ref mut om = assembly_factory.mut_meta(
            assembly_factory.object_meta(),
            format!("{}-{}", self.pre_name(), parent.get_name().to_string()),
            parent.get_account(),
        );
        assembly_factory.set_status(Status::pending());
        // set the parents datacenter/location or clustername
        assembly_factory.set_cluster_name(om, parent.get_cluster_name());
        assembly_factory.set_owner_reference(
            om,
            parent.type_meta().kind,
            parent.type_meta().api_version,
            parent.object_meta().name,
            parent.get_id().to_string(),
        );
        assembly_factory.set_labels(om, parent.get_labels());
        assembly_factory.set_initializers(om, parent.get_initializers());
        assembly_factory.set_finalizers(om, parent.get_finalizers());
        assembly_factory.set_meta(type_meta_url(parent.children()), om.clone());
        // Stick the metadata with the parent's metada +
        // individual plans name as rioos_sh_blueprint_applied
        // Example  rioos_sh_blueprint_applied = "hyperledger:fabric/latest"
        let mut b = BTreeMap::new();
        parent
            .get_metadata()
            .into_iter()
            .map(|kv| {
                let k = String::from(kv.0.clone());
                let v = String::from(kv.1.clone());
                b.insert(k.clone(), v.clone());
                (k, v)
            })
            .collect::<BTreeMap<String, String>>();

        b.insert(RIOOS_SH_BLUEPRINT_APPLIED.to_string(), plan_property.get_name());

        assembly_factory.set_metadata(b);

        assembly_factory
    }

    //Generates a pre_name of 5 ascii random character
    fn pre_name(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(PRE_NAME_LEN)
            .collect::<String>()
            .to_lowercase()
    }
}
