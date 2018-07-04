// Copyright 2018 The Rio Advancement Inc
//

//! Api gets wired here for the node api server.
//!

use api::audit::blockchain_api::EventLog;
use api::audit::config::BlockchainConn;
use api::events::EventLogger;
use api::objectstorage::config::ObjectStorageCfg;
use api::security::config::SecurerConn;
use api::{audit, authorize, cluster, deploy, devtooling, objectstorage, security, Api};
use audit::config::InfluxClientConn;
use audit::vulnerable::vulnerablity::AnchoreClient;
use auth::rbac::{permissions, license};
use config::Config;
use db::data_store::*;
use error::Result;
use http_gateway;
use http_gateway::app::prelude::*;
use http_gateway::http::pack;
use http_gateway::http::middleware::EntitlementAct;
use iron;
use mount::Mount;
use node::runtime::ApiSender;
use persistent;
use protocol::cache::ExpanderSender;
use router::Router;
use std::sync::Arc;
use telemetry::metrics::prometheus::PrometheusClient;

// ApiSrv using GatewayCfg.
#[derive(Clone, Debug)]
pub struct ApiSrv {
    config: Arc<Config>,
}

impl ApiSrv {
    pub fn new(config: Arc<Config>) -> Self {
        ApiSrv {
            config: config.clone(),
        }
    }

    // A generic implementation that launches `Node` and optionally creates threads
    // for aran api handlers.
    // Aran api v1 prefix is `/api/v1`
    pub fn start(self, api_sender: ApiSender, ds: Box<DataStoreConn>) -> Result<()> {
        //You are free to add move evs of type
        // persistent::Read::<EventLog>
        // However we won't be having it.
        let ev = persistent::Read::<EventLog>::both(EventLogger::new(
            api_sender,
            &self.config.blockchain.cache_dir,
            self.config.blockchain.enabled.clone(),
        ));
        http_gateway::app::start::<Wirer, _, _>(ev, self.config.clone(), ds);

        Ok(())
    }
}

struct Wirer;

impl HttpGateway for Wirer {
    const APP_NAME: &'static str = "rioos-api";

    type Config = Config;

    fn add_middleware(_config: Arc<Self::Config>, chain: &mut iron::Chain, ds: Box<DataStoreConn>) {
        //let ods = DataStoreConn::new().ok();
        let pds: DataStoreConn = *ds.clone();
        chain.link(persistent::Read::<DataStoreBroker>::both(Arc::new(
                    pds,
        )));
        let mut permissions = permissions::Permissions::new(ds.clone());
        permissions.with_cache();
        chain.link_before(Arc::new(RBAC::new(&*_config, permissions)));

        let mut license = license::LicensesFascade::new(ds.clone());
        license.with_cache();
        chain.link_before(Arc::new(EntitlementAct::new(&*_config, license)));

        chain.link_after(pack::CompressionMiddleware);

        chain.link_after(Custom404);

        chain.link_after(Cors);
    }

    fn mount(_config: Arc<Self::Config>, chain: iron::Chain) -> Mount {
        let mut mount = Mount::new();

        mount.mount("/api/v1", chain);

        mount
    }

    fn router(config: Arc<Self::Config>, ds: Box<DataStoreConn>) -> Router {
        let mut router = Router::new();
        //cluster apis
        let mut network = cluster::network_api::NetworkApi::new(ds.clone());
        network.wire(config.clone(), &mut router);

        let mut node = cluster::node_api::NodeApi::new(
            ds.clone(),
            Box::new(PrometheusClient::new(&*config.clone())),
        );
        node.wire(config.clone(), &mut router);

        let mut healthz = cluster::healthz_api::HealthzApi::new(
            ds.clone(),
            Box::new(PrometheusClient::new(&*config.clone())),
        );
        healthz.wire(config.clone(), &mut router);

        let mut sensei = cluster::senseis_api::SenseisApi::new(ds.clone());
        sensei.wire(config.clone(), &mut router);


        let mut diagnostics = cluster::diagnostics_api::DiagnosticsApi::new(
            ds.clone(),
            Box::new(PrometheusClient::new(&*config.clone())),
            config.clone(),
        );
        diagnostics.wire(config.clone(), &mut router);

        let mut storage = cluster::storage_api::StorageApi::new(ds.clone());
        storage.wire(config.clone(), &mut router);

        let mut s3 = objectstorage::bucket_api::ObjectStorageApi::new(Box::new(
            ObjectStorageCfg::new(&*config.clone()),
        ));
        s3.wire(config.clone(), &mut router);

        let mut service = deploy::service::ServiceApi::new(ds.clone());
        service.wire(config.clone(), &mut router);

        let mut endpoints = deploy::endpoint::EndpointApi::new(ds.clone());
        endpoints.wire(config.clone(), &mut router);

        let mut plan = deploy::plan_factory::PlanFactory::new(ds.clone());
        plan.wire(config.clone(), &mut router);

        //deployment apis
        let mut assembly = deploy::assembly::AssemblyApi::new(
            ds.clone(),
            Box::new(PrometheusClient::new(&*config.clone())),
        );
        assembly.wire(config.clone(), &mut router);

        let mut assembly_factory =
            deploy::assembly_factory::AssemblyFactoryApi::new(ds.clone());
            assembly_factory.wire(config.clone(), &mut router);

        let mut stacks_factory =
            deploy::stacks_factory::StacksFactoryApi::new(ds.clone());
            stacks_factory.wire(config.clone(), &mut router);

        //securer apis
        let mut securer = security::auth_api::AuthenticateApi::new(ds.clone());
        securer.wire(config.clone(), &mut router);

        let mut passticket =
            security::passticket_api::PassTicketApi::new(ds.clone());
            passticket.wire(config.clone(), &mut router);

        let mut secret = security::secret_api::SecretApi::new(
            ds.clone(),
            Box::new(SecurerConn::new(&*config.clone())),
        );
        secret.wire(config.clone(), &mut router);

        let mut service_account =
            security::service_account_api::SeriveAccountApi::new(ds.clone());
        service_account.wire(config.clone(), &mut router);

        //job apis
        let mut job = deploy::job::JobApi::new(ds.clone());
        job.wire(config.clone(), &mut router);

        let mut volume = deploy::volume::VolumeApi::new(ds.clone());
        volume.wire(config.clone(), &mut router);

        //scaling apis
        let mut hscale = deploy::horizontalscaling::HorizontalScalingApi::new(
            ds.clone(),
            Box::new(PrometheusClient::new(&*config.clone())),
        );
        hscale.wire(config.clone(), &mut router);

        let mut vscale = deploy::vertical_scaling::VerticalScalingApi::new(
            ds.clone(),
            Box::new(PrometheusClient::new(&*config.clone())),
        );
        vscale.wire(config.clone(), &mut router);

        let mut console =
            deploy::console::Containers::new(ds.clone(), config.clone());
        console.wire(config.clone(), &mut router);

        //origin
        let mut origin = deploy::origin::OriginApi::new(ds.clone());
        origin.wire(config.clone(), &mut router);

        let mut team = deploy::team::TeamApi::new(ds.clone());
        team.wire(config.clone(), &mut router);

        let mut role = authorize::role::RoleApi::new(ds.clone());
        role.wire(config.clone(), &mut router);

        let mut permission =
            authorize::permission::PermissionApi::new(ds.clone());
        permission.wire(config.clone(), &mut router);

        let mut settings =
            security::settings_map_api::SettingsMapApi::new(ds.clone());
        settings.wire(config.clone(), &mut router);

        let mut log = audit::log_api::LogApi::new(
            ds.clone(),
            Box::new(InfluxClientConn::new(&*config.clone())),
        );
        log.wire(config.clone(), &mut router);

        let mut vuln = audit::vuln_api::VulnApi::new(
            ds.clone(),
            Box::new(AnchoreClient::new(&*config.clone())),
        );
        vuln.wire(config.clone(), &mut router);

        let mut build_config =
            devtooling::build_config::BuildConfigApi::new(ds.clone());
        build_config.wire(config.clone(), &mut router);

        let mut build = devtooling::build::BuildApi::new(ds.clone());
        build.wire(config.clone(), &mut router);

        let mut image_references =
            devtooling::image_references::ImageReferencesApi::new(ds.clone());
        image_references.wire(config.clone(), &mut router);

        let mut image_marks =
            devtooling::image_marks::ImageMarksApi::new(ds.clone());
        image_marks.wire(config.clone(), &mut router);

        let mut block_chain = audit::blockchain_api::BlockChainApi::new(
            ds.clone(),
            Box::new(BlockchainConn::new(&*config.clone())),
        );
        block_chain.wire(config.clone(), &mut router);

    router
    }
}
