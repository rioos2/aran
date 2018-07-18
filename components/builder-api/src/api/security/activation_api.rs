// Copyright 2018 The Rio Advancement Inc

use api::Api;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{internal_error, not_found_error};
use http_gateway::util::errors::AranResult;
use iron::prelude::*;
use iron::status;
use protocol::api::base::IdGet;
use router::Router;
use activate::models::activation::DataStore;
use entitle::models::license;

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
        match license::DataStore::new(&self.conn).list_blank() {
        Ok(Some(license)) => {
            let status = license.into_iter().map(|l|{l.get_status()}).collect::<_>();
            match DataStore::new(&self.conn).wizard(IdGet::with_id(status)) {
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


        router.get(
            "/wizards",
            XHandler::new(C {
                inner: wizard,
            }),
            "wizard",
        );
    }
}
