// Copyright 2018 The Rio Advancement Inc
//

//! Node that serves the api.
//!

//TO-DO: Commented request_id by kishore.
//The objective of request_id is to provide a unique id to map a request
//mod request_id;

use router::Router;
use mount::Mount;
use iron::{Chain, Iron};
use std::sync::Arc;
use std::path::PathBuf;

use std::thread;
use persistent;
use rio_net::http::middleware::*;
use hyper_native_tls::NativeTlsServer;

use api::Api;
use config::Config;
use common::ui::UI;
use error::Result;
use rio_core::fs::rioconfig_config_path;

use super::api::{security, deploy};

use db::data_store::*;

// Iron defaults to a threadpool of size `8 * num_cpus`.
// See: http://172.16.2.131:9633/iron/prelude/struct.Iron.html#method.http
const HTTP_THREAD_COUNT: usize = 128;

// Node that contains handler (`RuntimeHandler`)`.
#[derive(Debug)]
pub struct Node {
    config: Arc<Config>,
}

impl Node {
    // Creates node for the given api and node configuration.
    pub fn new(config: Arc<Config>) -> Self {
        Node { config: config.clone() }
    }
    // A generic implementation that launches `Node` and optionally creates threads
    // for aran api handlers.
    // Aran api v1 prefix is `/api/v1`
    pub fn run(self, ui: &mut UI) -> Result<()> {
        let public_api_thread = match Some("dummy") {
            Some(_) => {
                let mut mount = Mount::new();
                let mut router = Router::new();
                let ds = DataStoreConn::new().unwrap();

                //plan Api
                let mut plan = deploy::marketplace::MarketPlaceApi::new(Box::new(ds.clone()));
                plan.wire(self.config.clone(), &mut router);

                //securer Api
                let mut securer = security::auth_api::AuthenticateApi::new(Box::new(ds.clone()));
                securer.wire(self.config.clone(), &mut router);

                //origin Api
                let mut origin = deploy::origin::OriginApi::new(Box::new(ds.clone()));
                origin.wire(self.config.clone(), &mut router);

                //package Api
                let mut package = deploy::package::PackageApi::new(Box::new(ds.clone()));
                package.wire(self.config.clone(), &mut router);

                mount.mount("/api/v1", router);

                let mut chain = Chain::new(mount);

                chain.link_after(Custom404);

                chain.link_after(Cors);

                chain.link(persistent::Read::<DataStoreBroker>::both(
                    ds.setup_marketplace(ui)?.clone(),
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
                        None => server.http(&conf.http).unwrap(),
                    };
                });
                ui.end("Router ").unwrap();

                Some(thread)
            }
            None => None,
        };

        if let Some(public_api_thread) = public_api_thread {
            public_api_thread.join().unwrap();
        }

        Ok(())
    }
}
