use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use common::ui;
use api::{Api, ApiValidator, Validator, ParmsVerifier};
use protocol::api::schema::{dispatch, type_meta};
use config::Config;
use error::Error;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

use deploy::models::endpoint;

use protocol::api::base::MetaFields;
use protocol::api::endpoints::EndPoints;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;
use bytes::Bytes;
use serde_json;
use protocol::api::base::IdGet;

#[derive(Clone)]
pub struct EndpointApi {
    conn: Box<DataStoreConn>,
}

/// Endpoint api: EndpointsApi manages Endpoints of an Assembly.
/// Needs a Datastore mapper to manage state, a DataStoreConn needs to be sent in.
//
/// Endpoint: URLs supported are.
/// POST: /endpoints
/// GET: /endpoint/:id
/// GET: /endpoint/assembly/:id
/// GET: /endpoint
impl EndpointApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        EndpointApi { conn: datastore }
    }

    //POST: /endpoints
    //The body has the input Endpoint
    //Returns a mutated Endpoint with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<EndPoints>>()?)?;

        let m = unmarshall_body.mut_meta(unmarshall_body.object_meta(), unmarshall_body.get_name(), unmarshall_body.get_account());

        unmarshall_body.set_meta(type_meta(req), m);

        ui::rawdumpln(Colour::White, '✓', format!("======= parsed {:?} ", unmarshall_body));

        match endpoint::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(Some(endpoints)) => Ok(render_json(status::Ok, &endpoints)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /endpoint/:id
    //Input id - u64 as input and returns a AssemblyFactory
    pub fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match endpoint::DataStore::show(&self.conn, &params) {
            Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
            Ok(None) => Err(not_found_error(&format!("{} for {}", Error::Db(RecordsNotFound), &params.get_id()))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    //GET: /endpoint/:id
    //Input id - u64 as input
    //Returns an endpoint
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match endpoint::DataStore::show(&self.conn, &idget) {
            Ok(Some(end)) => {
                let data = json!({
                            "type": typ,
                            "data": end,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    //GET: /endpoint/assembly/:id
    //Input assembly_id Returns list of endpoints for an assembly
    //Will need roles/permission to access others origin
    pub fn show_by_assembly(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match endpoint::DataStore::show_by_assembly(&self.conn, &params) {
            Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
            Ok(None) => Err(not_found_error(&format!("{} for {}", Error::Db(RecordsNotFound), &params.get_id()))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    //GET: /endpoint
    //Global: Returns all the Endpoints (irrespective of origin)
    //Will need roles/permission to access this.
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match endpoint::DataStore::list_blank(&self.conn) {
            Ok(Some(endpoints)) => Ok(render_json_list(status::Ok, dispatch(req), &endpoints)),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }
}

impl Api for EndpointApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures: assemblys
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let show_by_assembly = move |req: &mut Request| -> AranResult<Response> { _self.show_by_assembly(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        router.post("/endpoints", XHandler::new(C { inner: create }).before(basic.clone()), "endpoints");
        router.get("/endpoints/:id", XHandler::new(C { inner: show }).before(basic.clone()), "endpoint_show");
        router.get("/endpoints/assembly/:id", XHandler::new(C { inner: show_by_assembly }).before(basic.clone()), "endpoint_show_by_assembly");
        router.get("/endpoints", XHandler::new(C { inner: list_blank }).before(basic.clone()), "endpoint_list_blank");
    }
}

impl ApiValidator for EndpointApi {}

impl ParmsVerifier for EndpointApi {}

impl Validator for EndPoints {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.subsets.is_empty() {
            s.push("subsets".to_string());
        }

        if self.object_meta().owner_references.len() <= 0 {
            s.push("owner_references".to_string());
        } else {
            self.object_meta()
                .owner_references
                .iter()
                .map(|x| {
                    if x.uid.len() <= 0 {
                        s.push("uid".to_string());
                    }
                })
                .collect::<Vec<_>>();
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
