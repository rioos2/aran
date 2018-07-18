// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, ParmsVerifier, Validator};
use config::Config;
use error::Error;
use protocol::api::schema::dispatch;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};

/// TO_DO: Should be named  (authorize::models::roles, authorize::models::permission)
use authorize::models::role;

use protocol::api::authorize::Roles;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;

/// role api: RoleApi provides ability to declare the roles
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Secret: URLs supported are.
/// POST: /roles,,
/// GET: /roles,
/// GET: /roles/:id,
//GET: /roles/:name
#[derive(Clone)]
pub struct RoleApi {
    conn: Box<DataStoreConn>,
}

impl RoleApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        RoleApi { conn: datastore }
    }
    //POST: /roles
    //The body has the input cluster::roles
    //Returns a mutated Roles  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn role_create(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate::<Roles>(req.get::<bodyparser::Struct<Roles>>()?)?;
        debug!("{} ✓",
            format!("======= parsed {:?} ", unmarshall_body),
        );
        match role::DataStore::roles_create(&self.conn, &unmarshall_body) {
            Ok(roles_create) => Ok(render_json(status::Ok, &roles_create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /roles/:id
    //Input id - u64 as input and returns a roles
    fn role_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match role::DataStore::roles_show(&self.conn, &params) {
            Ok(Some(roles)) => Ok(render_json(status::Ok, &roles)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /roles/:name
    //Input as string input and returns a roles
    fn role_show_by_name(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;
        match role::DataStore::role_show_by_name(&self.conn, &params) {
            Ok(Some(roles)) => Ok(render_json(status::Ok, &roles)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /roles
    //Returns all the roles(irrespective of namespaces)
    fn role_list(&self, req: &mut Request) -> AranResult<Response> {
        match role::DataStore::roles_list(&self.conn) {
            Ok(Some(roles_list)) => Ok(render_json_list(status::Ok, dispatch(req), &roles_list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for RoleApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : roles
        let _self = self.clone();
        let role_create =
            move |req: &mut Request| -> AranResult<Response> { _self.role_create(req) };

        let _self = self.clone();
        let role_list = move |req: &mut Request| -> AranResult<Response> { _self.role_list(req) };

        let _self = self.clone();
        let role_show = move |req: &mut Request| -> AranResult<Response> { _self.role_show(req) };

        let _self = self.clone();
        let role_show_by_name =
            move |req: &mut Request| -> AranResult<Response> { _self.role_show_by_name(req) };

        //Routes:  Authorization : Roles
        router.post(
            "/roles",
            XHandler::new(C { inner: role_create }).before(basic.clone()),
            "roles",
        );
        router.get(
            "/roles",
            XHandler::new(C { inner: role_list }).before(basic.clone()),
            "role_list",
        );
        router.get(
            "/roles/:id",
            XHandler::new(C { inner: role_show }).before(basic.clone()),
            "role_show",
        );

        router.get(
            "/roles/name/:name",
            XHandler::new(C {
                inner: role_show_by_name,
            }).before(basic.clone()),
            "role_show_by_name",
        );
    }
}

impl ApiValidator for RoleApi {}

impl ParmsVerifier for RoleApi {}

impl Validator for Roles {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_name().len() <= 0 {
            s.push("name".to_string());
        }

        if self.get_description().len() <= 0 {
            s.push("description".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
