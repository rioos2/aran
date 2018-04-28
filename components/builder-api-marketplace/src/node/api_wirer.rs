// Copyright 2018 The Rio Advancement Inc
//

//! Api gets wired here for the node api server.
//!

use std::sync::Arc;
use std::thread;

use router::Router;
use mount::Mount;
use iron;
use error::{Error, Result};

use persistent;

use http_gateway;
use http_gateway::http::pack;
use http_gateway::app::prelude::*;

use api::Api;
use config::Config;

use api::{deploy, security};

use db::data_store::*;

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
    pub fn start(self) -> Result<()> {
        let ds = DataStoreConn::new()?;

        let dev = persistent::Read::<DataStoreBroker>::both(Arc::new(ds));
        // the thread exits faster, tries wrapping in on ther thread. So this actually runs like
        // [Main Thread] ----> Thread[0] - HttpGateway  ----> Thread[1] AppWirer(below thread)
        // But still the thread wasn't blocked
        thread::spawn(move || http_gateway::app::start::<Wirer, _, _>(dev, self.config.clone()).map_err(Error::HttpsGateway));

        //Added park on the current thread, which is [Main Thread] in the above illustration.
        //
        thread::park();

        Ok(())
    }
}

struct Wirer;

impl HttpGateway for Wirer {
    const APP_NAME: &'static str = "rio.marketplaces-api";

    type Config = Config;

    fn add_middleware(_config: Arc<Self::Config>, chain: &mut iron::Chain) {
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
                //plan Api
                let mut plan = deploy::marketplace::MarketPlaceApi::new(Box::new(ds.clone()));
                plan.wire(config.clone(), &mut router);

                //securer Api
                let mut securer = security::auth_api::AuthenticateApi::new(Box::new(ds.clone()));
                securer.wire(config.clone(), &mut router);

                //origin Api
                let mut origin = deploy::origin::OriginApi::new(Box::new(ds.clone()));
                origin.wire(config.clone(), &mut router);

                //package Api
                let mut package = deploy::package::PackageApi::new(Box::new(ds.clone()));
                package.wire(config.clone(), &mut router);
            }
            None => {
                error!("Failed to wire the router, \ndatabase isn't ready.");
            }
        }

        router
    }
}
