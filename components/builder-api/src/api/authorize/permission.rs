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

use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};

/// TO_DO: Should be named  (authorize::models::teams, authorize::models::permission)
use authorize::models::permission;
use protocol::api::{authorize::Permissions, base::IdGet, schema::dispatch};
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_PERMISSION};

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::{MissingParameter, MustBeNumeric};

/// permission api: PermissionApi provides ability to declare the Permissions
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// POST: /permissions,,
/// GET: /permissions,
/// GET: /permissions/:id,
/// GET: /permissions/policies/:policy_id
/// GET: /permissions/:id/policies/:policy_id
/// GET: /permissions/email/:name
#[derive(Clone)]
pub struct PermissionApi {
    conn: Box<DataStoreConn>,
}

impl PermissionApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        PermissionApi { conn: datastore }
    }
    //POST: /permissions
    //The body has the input cluster::permissions
    //Returns a mutated Permissions  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body =
            self.validate::<Permissions>(req.get::<bodyparser::Struct<Permissions>>()?)?;
        debug!("{} ✓",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match permission::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(permissions_create)) => Ok(render_json(status::Ok, &permissions_create)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /permissions
    //Returns all the permissions(irrespective of namespaces)
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match permission::DataStore::new(&self.conn).list_blank() {
            Ok(Some(permissions_list)) => Ok(render_json_list(
                status::Ok,
                dispatch(_req),
                &permissions_list,
            )),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //Send in the policy id and get all the list the permissions for the policy.
    pub fn list_by_policy(&self, req: &mut Request) -> AranResult<Response> {
        let policy_id = {
            let params = req.extensions.get::<Router>().unwrap();
            match params.find("policy_id").unwrap().parse::<u64>() {
                Ok(policy_id) => policy_id,
                Err(_) => return Err(bad_request(&MustBeNumeric("policy_id".to_string()))),
            }
        };

        match permission::DataStore::new(&self.conn)
            .list_by_policy(&IdGet::with_id(policy_id.clone().to_string()))
        {
            Ok(Some(permissions_list)) => Ok(render_json_list(
                status::Ok,
                dispatch(req),
                &permissions_list,
            )),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                policy_id
            ))),
        }
    }
    //GET: /permission/:id
    //Input id - u64 as input and returns a permission
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match permission::DataStore::new(&self.conn).show(&params) {
            Ok(Some(perms)) => Ok(render_json(status::Ok, &perms)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //Permission applied for a policy
    //Don't know the reason we use this.
    fn show_by_policy(&self, req: &mut Request) -> AranResult<Response> {
        let (perm_id, policy_id) = {
            let params = req.extensions.get::<Router>().unwrap();
            let perm_id = params.find("id").unwrap().to_owned();
            let policy_id = params.find("policy_id").unwrap().to_owned();

            (perm_id, policy_id)
        };
        let mut perms_get = IdGet::new();
        perms_get.set_id(perm_id);
        perms_get.set_name(policy_id);

        match permission::DataStore::new(&self.conn).show_by_policy(&perms_get) {
            Ok(Some(perms)) => Ok(render_json(status::Ok, &perms)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &perms_get.get_id()
            ))),
        }
    }
}

impl Api for PermissionApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);
        self.with_cache();

        //closures : permissions
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let list_by_policy =
            move |req: &mut Request| -> AranResult<Response> { _self.list_by_policy(req) };

        let _self = self.clone();
        let show_by_policy =
            move |req: &mut Request| -> AranResult<Response> { _self.show_by_policy(req) };

        //Routes:  Authorization : Permissions
        router.post(
            "/permissions",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "permissions",
        );
        router.get(
            "/permissions",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "permission_list",
        );
        router.get(
            "/permissions/policies/:policy_id",
            XHandler::new(C {
                inner: list_by_policy,
            }).before(basic.clone()),
            "list_permissions_by_policy",
        );
        router.get(
            "/permissions/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "permission_show",
        );
        router.get(
            "/permissions/:id/policies/:policy_id",
            XHandler::new(C {
                inner: show_by_policy,
            }).before(basic.clone()),
            "show_permissions_applied_for",
        );
    }
}
use serde_json;

impl ExpanderSender for PermissionApi {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let permission_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_PERMISSION.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                permission::DataStore::new(&_conn)
                    .show_by_policy(&id)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));

        &self.conn.expander.with(permission_service);
    }
}

impl ApiValidator for PermissionApi {}

impl ParmsVerifier for PermissionApi {}

impl Validator for Permissions {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_name().len() <= 0 {
            s.push("name".to_string());
        }

        if self.get_description().len() <= 0 {
            s.push("description".to_string());
        }

        if self.get_policy_id().len() <= 0 {
            s.push("policy_id".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
