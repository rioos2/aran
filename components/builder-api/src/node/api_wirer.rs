// Copyright 2018 The Rio Advancement Inc
//

//! Api gets wired here for the node api server.
//!

use std::sync::Arc;
use std::thread;

use router::Router;
use mount::Mount;
use iron::{Chain, Iron};
use error::{Result, Error};

use persistent;

use rio_net::http::middleware::*;
use rio_net::http::pack;
use rio_net::metrics::prometheus::PrometheusClient;
use rio_net::metrics::vulnerablity::AnchoreClient;
use node::runtime::Runtime;

use api::Api;
use api::events::EventLogger;
use config::Config;
use common::ui::UI;

use api::{cluster, security, deploy, audit, devtooling};
use node::runtime::ApiSender;

use db::data_store::*;
use hyper_native_tls::NativeTlsServer;
use rio_core::fs::rioconfig_config_path;
use std::path::PathBuf;
use api::audit::blockchain_api::EventLog;

// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

// Node that contains `NodeApiConfig`.
#[derive(Clone, Debug)]
pub struct Wirer {
    config: Arc<Config>,
}

impl Wirer {
    pub fn new(config: Arc<Config>) -> Self {
        Wirer { config: config.clone() }
    }

    // A generic implementation that launches `Node` and optionally creates threads
    // for aran api handlers.
    // Aran api v1 prefix is `/api/v1`
    pub fn start(self, ui: &mut UI, api_sender: ApiSender, rg: Runtime) -> Result<()> {
        let ods = DataStoreConn::new().ok();

        let api_wired_thread = match ods {
            Some(ds) => {
                ui.begin("Router");

                let mut mount = Mount::new();
                let mut router = Router::new();

                //cluster apis
                let mut network = cluster::network_api::NetworkApi::new(Box::new(ds.clone()));
                network.wire(self.config.clone(), &mut router);

                let mut node = cluster::node_api::NodeApi::new(
                    Box::new(ds.clone()),
                    Box::new(PrometheusClient::new(&*self.config.clone())),
                );
                node.wire(self.config.clone(), &mut router);

                let mut storage = cluster::storage_api::StorageApi::new(Box::new(ds.clone()));
                storage.wire(self.config.clone(), &mut router);

                let mut service = deploy::service::ServiceApi::new(Box::new(ds.clone()));
                service.wire(self.config.clone(), &mut router);

                let mut endpoints = deploy::endpoint::EndpointApi::new(Box::new(ds.clone()));
                endpoints.wire(self.config.clone(), &mut router);

                let mut plan = deploy::plan_factory::PlanFactory::new(Box::new(ds.clone()));
                plan.wire(self.config.clone(), &mut router);

                //deployment apis
                let mut assembly = deploy::assembly::AssemblyApi::new(
                    Box::new(ds.clone()),
                    Box::new(PrometheusClient::new(&*self.config.clone())),
                );
                assembly.wire(self.config.clone(), &mut router);

                let mut assembly_factory = deploy::assembly_factory::AssemblyFactoryApi::new(Box::new(ds.clone()));
                assembly_factory.wire(self.config.clone(), &mut router);

                //securer apis
                let mut securer = security::auth_api::AuthenticateApi::new(Box::new(ds.clone()));
                securer.wire(self.config.clone(), &mut router);

                let mut passticket = security::passticket_api::PassTicketApi::new(Box::new(ds.clone()));
                passticket.wire(self.config.clone(), &mut router);

                let mut secret = security::secret_api::SecretApi::new(
                    Box::new(ds.clone()),
                    Box::new(SecurerConn::new(&*self.config.clone())),
                );
                secret.wire(self.config.clone(), &mut router);

                let mut service_account = security::service_account_api::SeriveAccountApi::new(Box::new(ds.clone()));
                service_account.wire(self.config.clone(), &mut router);

                //job apis
                let mut job = deploy::job::JobApi::new(Box::new(ds.clone()));
                job.wire(self.config.clone(), &mut router);

                let mut volume = deploy::volume::VolumeApi::new(Box::new(ds.clone()));
                volume.wire(self.config.clone(), &mut router);

                //scaling apis
                let mut hscale = deploy::horizontalscaling::HorizontalScalingApi::new(
                    Box::new(ds.clone()),
                    Box::new(PrometheusClient::new(&*self.config.clone())),
                );
                hscale.wire(self.config.clone(), &mut router);

                let mut vscale = deploy::vertical_scaling::VerticalScalingApi::new(
                    Box::new(ds.clone()),
                    Box::new(PrometheusClient::new(&*self.config.clone())),
                );
                vscale.wire(self.config.clone(), &mut router);

                //origin
                let mut origin = deploy::origin::OriginApi::new(Box::new(ds.clone()));
                origin.wire(self.config.clone(), &mut router);

                let mut team = deploy::team::TeamApi::new(Box::new(ds.clone()));
                team.wire(self.config.clone(), &mut router);

                let mut authorize = security::authorize_api::AuthorizeApi::new(Box::new(ds.clone()));
                authorize.wire(self.config.clone(), &mut router);

                let mut settings = security::settings_map_api::SettingsMapApi::new(Box::new(ds.clone()));
                settings.wire(self.config.clone(), &mut router);

                let mut log = audit::log_api::LogApi::new(
                    Box::new(ds.clone()),
                    Box::new(InfluxClientConn::new(&*self.config.clone())),
                );
                log.wire(self.config.clone(), &mut router);

                let mut vuln = audit::vuln_api::VulnApi::new(
                    Box::new(ds.clone()),
                    Box::new(AnchoreClient::new(&*self.config.clone())),
                );
                vuln.wire(self.config.clone(), &mut router);

                let mut build_config = devtooling::build_config::BuildConfigApi::new(Box::new(ds.clone()));
                build_config.wire(self.config.clone(), &mut router);

                let mut build = devtooling::build::BuildApi::new(Box::new(ds.clone()));
                build.wire(self.config.clone(), &mut router);

                let mut image_references = devtooling::image_references::ImageReferencesApi::new(Box::new(ds.clone()));
                image_references.wire(self.config.clone(), &mut router);

                let mut image_marks = devtooling::image_marks::ImageMarksApi::new(Box::new(ds.clone()));
                image_marks.wire(self.config.clone(), &mut router);


                let mut block_chain = audit::blockchain_api::BlockChainApi::new(
                    Box::new(ds.clone()),
                    Box::new(BlockchainConn::new(&*self.config.clone())),
                );
                block_chain.wire(self.config.clone(), &mut router);

                mount.mount("/api/v1", router);

                let mut chain = Chain::new(mount);

                chain.link(persistent::Read::<EventLog>::both(EventLogger::new(
                    api_sender,
                    &self.config.blockchain.cache_dir,
                    *&self.config.blockchain.enabled,
                )));

                chain.link_after(pack::CompressionMiddleware);

                chain.link_after(Custom404);

                chain.link_after(Cors);

                chain.link(persistent::Read::<DataStoreBroker>::both(
                    ds.setup(ui)?.clone(),
                ));

                let conf = self.config.clone();

                let thread = thread::spawn(move || {
                    let mut server = Iron::new(chain);
                    server.threads = HTTP_THREAD_COUNT;

                    match conf.http.tls_pkcs12_file {
                        Some(ref tls_location) => {
                            let tls = NativeTlsServer::new(
                                PathBuf::from(&*rioconfig_config_path(None).join(tls_location.clone())),
                                &self.config
                                    .http
                                    .tls_pkcs12_pwd
                                    .clone()
                                    .unwrap_or("".to_string())
                                    .to_string(),
                            ).unwrap();
                            server.https(&conf.http, tls).unwrap()
                        }
                        None => Err(Error::MissingTLS("api server pfx".to_string())).unwrap(),
                    };
                });
                ui.end("Router ").unwrap();

                Some(thread)
            }
            None => {
                // error!("failed to wire the api, \ndatabase isn't ready. {:?}", ods);
                None
            }
        };

        &rg.start()?;

        if let Some(api_wired_thread) = api_wired_thread {
            api_wired_thread.join().unwrap();
        }
        Ok(())
    }
}
