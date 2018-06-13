// Copyright 2018 The Rio Advancement Inc
//

//! AttachGenerator generates the attachable actions
//!
#![allow(unused_must_use)]

use std::collections::BTreeMap;

use error::Result;

use assembler::ServicesConfig;

use protocol::api::base::{MetaFields, Status, WhoAmITypeMeta};
use protocol::api::deploy::AssemblyFactory;
use protocol::api::linker::Services;
use protocol::api::schema::type_meta_url;

use builder::attacher::ServiceAttachAction;
use builder::service::rules::{ServiceRule, ServiceTallyer};
use builder::service::{SERVICE_TYPE_EXTERNALNAME, SERVICE_TYPE_LOADBALANCER};

pub type AssembledMapRule = Result<(AssemblyFactory, Vec<(String, String)>, ServiceRule)>;

pub type ServiceAttachActions = Vec<ServiceAttachAction>;

pub struct AttachGenerator;

//The Service actions generator for attaching
impl AttachGenerator {
    /// Returns a `ServiceActions` representing the linkeraction for every enum
    /// The actions created are
    /// 1. Loadbalancer:
    /// - If the initializer has loadbalancer and the plan category is application or container
    /// (or)
    ///   If the  ServiceRuleEvent is scale
    /// 2. DNS:
    ///   If the  ServiceRuleEvent is not scale
    pub fn generate(ampr: &AssembledMapRule, config: ServicesConfig) -> ServiceAttachActions {
        let mut actions = vec![];

        ampr.as_ref().map(|a2pl| {
            if Self::satisfied(ampr) || ServiceTallyer::tally_rule(&a2pl.2) {
                actions.push(Self::build_loadbalancer(&a2pl.0, config));
            }

            if !ServiceTallyer::tally_rule(&a2pl.2) {
                actions.push(Self::build_internal_dns(&a2pl.0, a2pl.clone().1.to_vec()));
            }
        });

        actions
    }

    /// Returns a `ServiceAction` representing the service that the deployment tried to link
    fn build_loadbalancer(
        factory: &AssemblyFactory,
        config: ServicesConfig,
    ) -> ServiceAttachAction {
        let mut s: Services = Services::new();

        let ref mut om = s.mut_meta(
            s.object_meta(),
            factory.object_meta().name,
            factory.get_account(),
        );
        s.set_cluster_name(om, factory.get_cluster_name());

        s.set_owner_reference(
            om,
            factory.type_meta().kind,
            factory.type_meta().api_version,
            factory.object_meta().name,
            factory.get_id(),
        );

        let jackie = s.who_am_i();
        s.set_meta(type_meta_url(jackie), om.clone());
        s.set_status(Status::pending());
        s.with_type(SERVICE_TYPE_LOADBALANCER.to_string());

        s.set_metadata(config.as_map());
        ServiceAttachAction::LoadBalancer(s)
    }

    /// Returns a `ServiceAction` representing the service that the deployment tried to link
    fn build_internal_dns(
        factory: &AssemblyFactory,
        assembly: Vec<(String, String)>,
    ) -> ServiceAttachAction {
        let mut s: Services = Services::new();

        let ref mut om = s.mut_meta(s.object_meta(), factory.get_name(), factory.get_account());
        s.set_cluster_name(om, factory.get_cluster_name());
        s.set_owner_reference(
            om,
            factory.type_meta().kind,
            factory.type_meta().api_version,
            factory.object_meta().name,
            factory.get_id(),
        );

        let jackie = s.who_am_i();
        s.set_meta(type_meta_url(jackie), om.clone());
        s.set_status(Status::pending());
        let mut names = BTreeMap::new();
        assembly
            .clone()
            .into_iter()
            .map(|x| names.insert(x.0, x.1))
            .collect::<Vec<_>>();

        s.with_type_names(SERVICE_TYPE_EXTERNALNAME.to_string(), names.clone());

        ServiceAttachAction::InternalDNS(s)
    }

    /// Returns true:
    /// If the initializer has loadbalancer and the plan category is application or container.
    fn satisfied(ampr: &AssembledMapRule) -> bool {
        ampr.as_ref()
            .map(|a| {
                let factory = &a.0;
                factory
                    .get_initializers()
                    .has_pending(SERVICE_TYPE_LOADBALANCER.to_string())
                    && ServiceTallyer::tally(
                        factory
                            .get_spec()
                            .get_plan()
                            .map_or("".to_string(), |p| p.get_category()),
                    )
            })
            .unwrap_or(false)
    }
}
