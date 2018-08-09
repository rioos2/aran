// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [origin] for the HTTP server

use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
use bodyparser;
use bytes::Bytes;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use iron::prelude::*;
use iron::status;
use job::models::jobs;
use protocol::api::base::{IdGet, MetaFields, StatusUpdate};
use protocol::api::job::Jobs;
use protocol::api::schema::{dispatch, type_meta};
use router::Router;
use serde_json;
use std::sync::Arc;

#[derive(Clone)]
pub struct JobApi {
    conn: Box<DataStoreConn>,
}

/// Job api
/// - every instance of NetworkApi needs a DataStoreConn
/// POST: networks, GET: networks/:id, GET: networks
impl JobApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        JobApi { conn: datastore }
    }

    //POST: /jobs
    //This isn't under any origin ? why  ?
    //The body has the input jobs
    //Returns a mutated Jobs with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Jobs>>()?)?;
        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match jobs::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(jobs)) => Ok(render_json(status::Ok, &jobs)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /jobs
    //Blank origin: Returns all the Jobs (irrespective of namespaces)
    //Will need teams/permission to access this.
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match jobs::DataStore::new(&self.conn).list() {
            Ok(Some(jobs)) => Ok(render_json_list(status::Ok, dispatch(_req), &jobs)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /jobs/node?node_id="100192091010"
    //Returns all the Jobs for a particular node
    fn show_by_node(&self, req: &mut Request) -> AranResult<Response> {
        let query_pairs = self.default_validate(req)?;
        match jobs::DataStore::new(&self.conn).show_by_node(&IdGet::with_id(query_pairs.get("node_id"))) {
            Ok(Some(jobs_get)) => Ok(render_json_list(status::Ok, dispatch(req), &jobs_get)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //PUT: /jobs/status
    //Input status  as input and returns an updated Jobs
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<StatusUpdate>>()?,
        )?;
        unmarshall_body.set_id(params.get_id());

        match jobs::DataStore::new(&self.conn).status_update(&unmarshall_body) {
            Ok(Some(jobs)) => Ok(render_json(status::Ok, &jobs)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: /jobs/:id
    //Input id - u64 as input
    //Returns an jobs
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        let res = match jobs::DataStore::new(&self.conn).show(&idget) {
            Ok(Some(job)) => {
                let data = json!({
                            "type": typ,
                            "data": job,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for JobApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let show_by_node = move |req: &mut Request| -> AranResult<Response> { _self.show_by_node(req) };

        //origin less,
        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        router.post(
            "/jobs",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "jobs",
        );        
        router.put(
            "/jobs/:id/status",
            XHandler::new(C { inner: status_update }).before(basic.clone()),
            "job_status_update",
        );
        router.get(
            "/jobs/node",
            XHandler::new(C { inner: show_by_node }).before(basic.clone()),
            "job_show_by_node",
        );
        router.get(
            "/jobs",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "job_list_blank",
        );
    }
}

impl ApiValidator for JobApi {}

impl ParmsVerifier for JobApi {}

impl QueryValidator for JobApi {}

//Validates parsed Job from the body of the request.
//Checks for ....
impl Validator for Jobs {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_spec().get_node_id().len() <= 0 {
            s.push("node_id".to_string());
        }
        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.object_meta().account.len() <= 0 {
            s.push("account".to_string());
        }
        if self.object_meta().owner_references.len() <= 0 {
            s.push("owner_references".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
