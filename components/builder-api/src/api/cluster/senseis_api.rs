// Copyright 2018 The Rio Advancement Inc

use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
use bodyparser;
use bytes::Bytes;
use clusters::models::senseis::DataStore;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::api::senseis::Senseis;
use protocol::api::schema::{dispatch, type_meta};
use router::Router;
use serde_json;
use std::sync::Arc;

#[derive(Clone)]
pub struct SenseisApi {
    conn: Box<DataStoreConn>,
}

//
/// Senseis: URLs supported are.
/// POST: /senseis,
/// GET: /senseis

impl SenseisApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        SenseisApi {
            conn: datastore
        }
    }

    //POST: /senseis
    //The body has the input cluster::Senseis
    //Returns a mutated senseis  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Senseis>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(sensei)) => Ok(render_json(status::Ok, &sensei)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
    // GET  / //GET: /senseis
    //Blank origin: Returns all the senseis (irrespective of namespaces)
    //Will need teams/permission to access this.
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match DataStore::new(&self.conn).list_blank() {
            Ok(Some(sensei_list)) => Ok(render_json_list(status::Ok, dispatch(_req), &sensei_list)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match DataStore::new(&self.conn).show(&idget) {
            Ok(Some(senseis)) => {
                let data = json!({
                            "type": typ,
                            "data": senseis,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

}
impl Api for SenseisApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        router.post(
            "/senseis",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "senseis",
        );
        router.get(
            "/senseis",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "senseis_list",
        );

    }
}

impl ApiValidator for SenseisApi {}

impl ParmsVerifier for SenseisApi {}

impl QueryValidator for SenseisApi {}

impl Validator for Senseis {
    //default implementation is to check for `name` and 'node_ip'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.get_node_ip().len() <= 0 {
            s.push("node_ip".to_string());
        }

        if self.get_status().get_phase().len() <= 0 {
            s.push("phase".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
