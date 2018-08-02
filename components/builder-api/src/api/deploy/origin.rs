// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [origin] for the HTTP server
use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, ParmsVerifier, Validator};
use protocol::api::schema::{dispatch, type_meta};

use config::Config;
use error::Error;
use error::ErrorMessage::{MissingParameter, MustBeNumeric};

use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};

use protocol::api::base::MetaFields;
use protocol::api::origin::Origin;
use session::origin_ds::OriginDS;

use bytes::Bytes;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use protocol::api::base::IdGet;
use serde_json;

#[derive(Clone)]
pub struct OriginApi {
    conn: Box<DataStoreConn>,
}

/// Origin api: OriginApi provides ability to create sandboxed or spaces for
/// users.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Orgins: URLs supported are.
/// POST: /origin,
/// GET: /origin/:id
impl OriginApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        OriginApi { conn: datastore }
    }

    //POST: /origins
    //The body has the input origins
    //Returns a mutated Origins with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    pub fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Origin>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            match unmarshall_body.get_account().parse::<u64>() {
                Ok(id) => id.to_string(),
                Err(_) => return Err(bad_request(&MustBeNumeric("account".to_string()))),
            },
        );

        unmarshall_body.set_meta(type_meta(req), m);

        match OriginDS::create(&self.conn, &unmarshall_body) {
            Ok(Some(origin)) => Ok(render_json(status::Ok, &origin)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /origins
    //Every user will be able to list their own origin.
    //Will need roles/permission to access others origin.
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match OriginDS::list_blank(&self.conn) {
            Ok(Some(origins)) => Ok(render_json_list(status::Ok, dispatch(req), &origins)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //Input account id as input and returns a origins
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_account(req)?;

        match OriginDS::list(&self.conn, &params) {
            Ok(Some(origins)) => Ok(render_json_list(status::Ok, dispatch(req), &origins)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_name()
            ))),
        }
    }

    //GET: /origins/:origins
    //Input id - u64 as input and returns a AssemblyFactory
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;

        match OriginDS::show(&self.conn, &params) {
            Ok(Some(origin)) => Ok(render_json(status::Ok, &origin)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_name()
            ))),
        }
    }

    //GET: /origins/:id
    //Input id - u64 as input
    //Returns an origins
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match OriginDS::show(&self.conn, &idget) {
            Ok(Some(origin)) => {
                let data = json!({
                            "type": typ,
                            "data": origin,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for OriginApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        //Origin API
        router.post(
            "/origins",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "origins",
        );

        //TODO
        //without authentication
        router.get("/origins", XHandler::new(C { inner: list_blank }), "origin_list");

        router.get(
            "accounts/:account_id/origins",
            XHandler::new(C { inner: list }).before(basic.clone()),
            "origin_list_by_account"
         );


        //TODO
        //without authentication
        router.get(
            "/origins/:name",
            XHandler::new(C { inner: show }),
            "origin_show",
        );
    }
}

impl ApiValidator for OriginApi {}

impl ParmsVerifier for OriginApi {}

//Validates parsed Origin from the body of the request.
//Checks for `....` in .....
//This is a NoOp for now.
impl Validator for Origin {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_name().len() <= 0 {
            s.push("name".to_string());
        }

        if self.get_account().len() <= 0 {
            s.push("account".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
