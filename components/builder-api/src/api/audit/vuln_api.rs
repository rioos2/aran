use std::sync::Arc;

use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ParmsVerifier, QueryValidator};

use config::Config;
use error::Error;

use rio_net::http::controller::*;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{internal_error, not_found_error};

use db::error::Error::RecordsNotFound;
use db::data_store::DataStoreConn;
use rio_net::metrics::vulnerablity::AnchoreClient;

#[derive(Clone)]
pub struct VulnApi {
    anchore: Box<AnchoreClient>,
    conn: Box<DataStoreConn>,
}

/// VulnApi: VulnApi provides ability to check the image vulnerabilty
//
/// Vulnerable: URLs supported are.
/// GET: /image/:name/vulnerablity,
impl VulnApi {
    pub fn new(datastore: Box<DataStoreConn>, anchore: Box<AnchoreClient>) -> Self {
        VulnApi { anchore: anchore, conn: datastore }
    }

    /// list the log for all (container and machine)
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;

        match self.anchore.check_vulnerablity(&format!("{}", &params.get_id())) {
            Ok(Some(image)) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for VulnApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        router.get("/image/:name/vulnerablity", XHandler::new(C { inner: show }).before(basic.clone()).before(TrustAccessed {}), "show");
    }
}

impl ParmsVerifier for VulnApi {}

impl QueryValidator for VulnApi {}
