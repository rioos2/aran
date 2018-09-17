// Copyright 2018 The Rio Advancement Inc

use activate::models::activation::DataStore;
use api::Api;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use entitle::models::license;
use error::Error;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{internal_error, not_found_error};
use http_gateway::util::errors::AranResult;
use iron::prelude::*;
use iron::status;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use router::Router;


use std::sync::Arc;

#[derive(Clone)]
pub struct ActivationApi {
    conn: Box<DataStoreConn>,
}

///
/// GET: /wizards
///

impl ActivationApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        ActivationApi { conn: datastore }
    }

    //GET : wizards
    fn wizard(&self, _req: &mut Request) -> AranResult<Response> {
        let params = IdGet::with_id("senseis".to_string());
        match license::DataStore::new(&self.conn).license_show_by_name(&params) {
            Ok(Some(license)) => {
                match DataStore::new(&self.conn).wizard(license.get_activation_completed()) {
                    Ok(wizard) => Ok(render_json(status::Ok, &wizard)),
                    Err(err) => Err(internal_error(&format!("{}\n", err))),
                }
            }
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }

    }
}

impl Api for ActivationApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let wizard = move |req: &mut Request| -> AranResult<Response> { _self.wizard(req) };


        router.get("/wizards", XHandler::new(C { inner: wizard }), "wizard");
    }
}
