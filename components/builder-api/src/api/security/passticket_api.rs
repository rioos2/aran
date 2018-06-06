// Copyright 2018 The Rio Advancement Inc

//! A collection of passticket [passticket - OTP] for the HTTP server

use api::Api;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use http_gateway::http::controller::*;
use http_gateway::util::errors::AranResult;
use http_gateway::util::errors::{internal_error, not_found_error};
use iron::prelude::*;
use iron::status;
use rand;
use router::Router;
use session::models::passticket;
use std::sync::Arc;

/// PassTicketApi : PassTicketApi provides ability to get passticket
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// PassTicketApi: URLs supported are.
/// GET: /passticket,
#[derive(Clone)]
pub struct PassTicketApi {
    conn: Box<DataStoreConn>,
}

impl PassTicketApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        PassTicketApi { conn: datastore }
    }
    //Get: /passtickets
    //Returns a mutated PassTicket  with
    //- id
    //- passticket: random number
    //- created_at
    fn create(&self, _req: &mut Request) -> AranResult<Response> {
        match passticket::DataStore::create_passticket(
            &self.conn,
            &rand::random::<u64>().to_string(),
        ) {
            Ok(Some(passticket)) => Ok(render_json(status::Ok, &passticket)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for PassTicketApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : PassTicket
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        //PassTicket API
        router.get(
            "/passtickets",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "passtickets",
        );
    }
}
