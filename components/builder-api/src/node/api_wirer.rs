// Copyright 2018 The Rio Advancement Inc
//

//! Api gets wired here for the node api server.
//!

use std::sync::Arc;

use router::Router;
use mount::Mount;
use iron;
use error::Result;

use persistent;

use http_gateway;
use http_gateway::http::pack;
use http_gateway::app::prelude::*;

use telemetry::metrics::prometheus::PrometheusClient;

use audit::vulnerable::vulnerablity::AnchoreClient;
use audit::config::InfluxClientConn;

use api::Api;
use api::events::EventLogger;
use api::audit::config::BlockchainConn;
use api::security::config::SecurerConn;
use config::Config;

use api::{audit, authorize, cluster, deploy, devtooling, security};
use node::runtime::ApiSender;

use db::data_store::*;
use api::audit::blockchain_api::EventLog;

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
    pub fn start(self, api_sender: ApiSender) -> Result<()> {
        //You are free to add move evs of type
        // persistent::Read::<EventLog>
        // However we won't be having it.
        let ev = persistent::Read::<EventLog>::both(EventLogger::new(
            api_sender,
            &self.config.blockchain.cache_dir,
            self.config.blockchain.enabled.clone(),
        ));
        http_gateway::app::start::<Wirer, _, _>(ev, self.config.clone());

        Ok(())
    }
}

struct Wirer;

impl HttpGateway for Wirer {
    const APP_NAME: &'static str = "rioos-api";

    type Config = Config;

    fn add_middleware(_config: Arc<Self::Config>, chain: &mut iron::Chain) {
        let ods = DataStoreConn::new().ok();

        match ods {
            Some(ds) => {
                chain.link(persistent::Read::<DataStoreBroker>::both(Arc::new(ds)));
            }
            None => {
                error!("Failed to wire the api middleware, \ndatabase isn't ready.");
            }
        }

        chain.link_after(pack::CompressionMiddleware);

        chain.link_after(Custom404);

        chain.link_after(Cors);
    }

    fn mount(_config: Arc<Self::Config>, chain: iron::Chain) -> Mount {
        let mut mount = Mount::new();

        mount.mount("/api/v1", chain);

        mount
    }

    fn router(config: Arc<Self::Config>) -> Router {
        let mut router = Router::new();

        let ods = DataStoreConn::new().ok();

        match ods {
            Some(ds) => {
                let one_ref_ds = Arc::new(ds);

                //cluster apis
                let mut network = cluster::network_api::NetworkApi::new(one_ref_ds.clone());
                network.wire(config.clone(), &mut router);

                let mut node = cluster::node_api::NodeApi::new(
                    one_ref_ds.clone(),
                    Box::new(PrometheusClient::new(&*config.clone())),
                );
                node.wire(config.clone(), &mut router);

                let mut diagnostics = cluster::diagnostics_api::DiagnosticsApi::new(
                    one_ref_ds.clone(),
                    Box::new(PrometheusClient::new(&*config.clone())),
                    config.clone(),
                );
                diagnostics.wire(config.clone(), &mut router);

                let mut storage = cluster::storage_api::StorageApi::new(one_ref_ds.clone());
                storage.wire(config.clone(), &mut router);

                let mut service = deploy::service::ServiceApi::new(one_ref_ds.clone());
                service.wire(config.clone(), &mut router);

                let mut endpoints = deploy::endpoint::EndpointApi::new(one_ref_ds.clone());
                endpoints.wire(config.clone(), &mut router);

                let mut plan = deploy::plan_factory::PlanFactory::new(one_ref_ds.clone());
                plan.wire(config.clone(), &mut router);

                //deployment apis
                let mut assembly = deploy::assembly::AssemblyApi::new(
                    one_ref_ds.clone(),
                    Box::new(PrometheusClient::new(&*config.clone())),
                );
                assembly.wire(config.clone(), &mut router);

                let mut assembly_factory = deploy::assembly_factory::AssemblyFactoryApi::new(one_ref_ds.clone());
                assembly_factory.wire(config.clone(), &mut router);

                //securer apis
                let mut securer = security::auth_api::AuthenticateApi::new(one_ref_ds.clone());
                securer.wire(config.clone(), &mut router);

                let mut passticket = security::passticket_api::PassTicketApi::new(one_ref_ds.clone());
                passticket.wire(config.clone(), &mut router);

                let mut secret = security::secret_api::SecretApi::new(
                    one_ref_ds.clone(),
                    Box::new(SecurerConn::new(&*config.clone())),
                );
                secret.wire(config.clone(), &mut router);

                let mut service_account = security::service_account_api::SeriveAccountApi::new(one_ref_ds.clone());
                service_account.wire(config.clone(), &mut router);

                //job apis
                let mut job = deploy::job::JobApi::new(one_ref_ds.clone());
                job.wire(config.clone(), &mut router);

                let mut volume = deploy::volume::VolumeApi::new(one_ref_ds.clone());
                volume.wire(config.clone(), &mut router);

                //scaling apis
                let mut hscale = deploy::horizontalscaling::HorizontalScalingApi::new(
                    one_ref_ds.clone(),
                    Box::new(PrometheusClient::new(&*config.clone())),
                );
                hscale.wire(config.clone(), &mut router);

                let mut vscale = deploy::vertical_scaling::VerticalScalingApi::new(
                    one_ref_ds.clone(),
                    Box::new(PrometheusClient::new(&*config.clone())),
                );
                vscale.wire(config.clone(), &mut router);

                let mut console = deploy::console::Containers::new(one_ref_ds.clone(), config.clone());
                console.wire(config.clone(), &mut router);

                //origin
                let mut origin = deploy::origin::OriginApi::new(one_ref_ds.clone());
                origin.wire(config.clone(), &mut router);

                let mut team = deploy::team::TeamApi::new(one_ref_ds.clone());
                team.wire(config.clone(), &mut router);

                let mut role = authorize::role::RoleApi::new(one_ref_ds.clone());
                role.wire(config.clone(), &mut router);

                let mut permission = authorize::permission::PermissionApi::new(one_ref_ds.clone());
                permission.wire(config.clone(), &mut router);

                let mut settings = security::settings_map_api::SettingsMapApi::new(one_ref_ds.clone());
                settings.wire(config.clone(), &mut router);

                let mut log = audit::log_api::LogApi::new(
                    one_ref_ds.clone(),
                    Box::new(InfluxClientConn::new(&*config.clone())),
                );
                log.wire(config.clone(), &mut router);

                let mut vuln = audit::vuln_api::VulnApi::new(
                    one_ref_ds.clone(),
                    Box::new(AnchoreClient::new(&*config.clone())),
                );
                vuln.wire(config.clone(), &mut router);

                let mut build_config = devtooling::build_config::BuildConfigApi::new(one_ref_ds.clone());
                build_config.wire(config.clone(), &mut router);

                let mut build = devtooling::build::BuildApi::new(one_ref_ds.clone());
                build.wire(config.clone(), &mut router);

                let mut image_references = devtooling::image_references::ImageReferencesApi::new(one_ref_ds.clone());
                image_references.wire(config.clone(), &mut router);

                let mut image_marks = devtooling::image_marks::ImageMarksApi::new(one_ref_ds.clone());
                image_marks.wire(config.clone(), &mut router);

                let mut block_chain = audit::blockchain_api::BlockChainApi::new(
                    one_ref_ds.clone(),
                    Box::new(BlockchainConn::new(&*config.clone())),
                );
                block_chain.wire(config.clone(), &mut router);
            }
            None => {
                error!("Failed to wire the router, \ndatabase isn't ready.");
            }
        }

        router
    }
}
