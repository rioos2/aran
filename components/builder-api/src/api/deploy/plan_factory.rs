// Copyright 2018 The Rio Advancement Inc

//! A collection of plans for deployment
//! PlanFactory produces plans which are blueprint for deployment.
//These are pre built recipes that a customer can use (ready to cook).

use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use common::ui;
use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::{dispatch, type_meta};

use config::Config;
use error::Error;
use error::ErrorMessage::MissingParameter;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

use deploy::models::blueprint;
use protocol::api::blueprint::Plan;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use protocol::api::base::{MetaFields, StatusUpdate};
use bytes::Bytes;
use serde_json;
use protocol::api::base::IdGet;

#[derive(Clone)]
pub struct PlanFactory {
    conn: Box<DataStoreConn>,
}

/// PlanFactory Api:  provides ability to manage prebuilt custom blueprint suites.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
// This isn't under any origin yet.
//
/// PlanFactory
/// POST: /planfactory
/// GET: /planfactory/:id
/// GET: storagepools PUT: storagepools/status_update
impl PlanFactory {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        PlanFactory { conn: datastore }
    }

    //POST: /planfactory
    //The body has the input Plan
    //Returns a mutated Plan with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Plan>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match blueprint::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(Some(plan)) => Ok(render_json(status::Ok, &plan)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /planctorys
    //Blank origin: Returns all the PlanFactorys (irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match blueprint::DataStore::list_blank(&self.conn) {
            Ok(Some(plans)) => Ok(render_json_list(status::Ok, dispatch(_req), &plans)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /plans/:id
    //Input id - u64 as input and returns a Plan factory
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match blueprint::DataStore::show(&self.conn, &params) {
            Ok(Some(plan_factory)) => Ok(render_json(status::Ok, &plan_factory)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //PUT: /plans/:id/status
    //Input status  as input and returns an updated plan
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<StatusUpdate>>()?,
        )?;
        unmarshall_body.set_id(params.get_id());

        match blueprint::DataStore::status_update(&self.conn, &unmarshall_body) {
            Ok(Some(plan)) => Ok(render_json(status::Ok, &plan)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: /plans/:id
    //Input id - u64 as input
    //Returns an plan
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match blueprint::DataStore::show(&self.conn, &idget) {
            Ok(Some(plan_factory)) => {
                let data = json!({
                            "type": typ,
                            "data": plan_factory,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for PlanFactory {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures:planfactory
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };


        router.post(
            "/plans",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "plans",
        );

        router.get(
            "/plans",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "plan_list",
        );

        router.get(
            "/plans/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "plan_show",
        );
        router.put(
            "/plans/:id/status",
            XHandler::new(C { inner: status_update }).before(basic.clone()),
            "plan_status_update",
        );
    }
}

impl ApiValidator for PlanFactory {}

impl ParmsVerifier for PlanFactory {}

//Validates parsed AssemblyFactory from the body of the request.
//Checks for `....` in ...
//This is NoOp for now.
impl Validator for Plan {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_category().len() <= 0 {
            s.push("category".to_string());
        }
        if self.get_version().len() <= 0 {
            s.push("version".to_string());
        }
        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
