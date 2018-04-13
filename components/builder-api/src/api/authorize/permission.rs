// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::dispatch;
use config::Config;
use error::Error;
use common::ui;
use ansi_term::Colour;


use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

/// TO_DO: Should be named  (authorize::models::roles, authorize::models::permission)
use authorize::models::permission;
use protocol::api::authorize::Permissions;
use protocol::api::base::IdGet;

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
//GET: /permissions/roles/:role_id
//GET: /permissions/:id/roles/:role_id
//GET: /permissions/email/:name
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
    fn permission_create(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate::<Permissions>(
            req.get::<bodyparser::Struct<Permissions>>()?,
        )?;
        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match permission::DataStore::permissions_create(&self.conn, &unmarshall_body) {
            Ok(Some(permissions_create)) => Ok(render_json(status::Ok, &permissions_create)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /permissions
    //Returns all the permissions(irrespective of namespaces)
    fn permission_list(&self, _req: &mut Request) -> AranResult<Response> {
        match permission::DataStore::permissions_list(&self.conn) {
            Ok(Some(permissions_list)) => Ok(render_json_list(
                status::Ok,
                dispatch(_req),
                &permissions_list,
            )),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //Send in the role id and get all the list the permissions for the role.
    pub fn list_permissions_by_role(&self, req: &mut Request) -> AranResult<Response> {
        let role_id = {
            let params = req.extensions.get::<Router>().unwrap();
            match params.find("role_id").unwrap().parse::<u64>() {
                Ok(role_id) => role_id,
                Err(_) => return Err(bad_request(&MustBeNumeric("role_id".to_string()))),
            }
        };

        match permission::DataStore::get_rolebased_permissions(&self.conn, &IdGet::with_id(role_id.clone().to_string())) {
            Ok(Some(permissions_list)) => Ok(render_json_list(
                status::Ok,
                dispatch(req),
                &permissions_list,
            )),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(
                &format!("{} for {}", Error::Db(RecordsNotFound), role_id),
            )),
        }
    }
    //GET: /permission/:id
    //Input id - u64 as input and returns a permission
    fn permission_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match permission::DataStore::permissions_show(&self.conn, &params) {
            Ok(Some(perms)) => Ok(render_json(status::Ok, &perms)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //Permission applied for a role
    //Don't know the reason we use this.
    fn show_permissions_applied_for(&self, req: &mut Request) -> AranResult<Response> {
        let (perm_id, role_id) = {
            let params = req.extensions.get::<Router>().unwrap();
            let perm_id = params.find("id").unwrap().to_owned();
            let role_id = params.find("role_id").unwrap().to_owned();

            (perm_id, role_id)
        };
        let mut perms_get = IdGet::new();
        perms_get.set_id(perm_id);
        perms_get.set_name(role_id);

        match permission::DataStore::get_specfic_permission_based_role(&self.conn, &perms_get) {
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

        //closures : permissions
        let _self = self.clone();
        let permission_create = move |req: &mut Request| -> AranResult<Response> { _self.permission_create(req) };

        let _self = self.clone();
        let permission_list = move |req: &mut Request| -> AranResult<Response> { _self.permission_list(req) };

        let _self = self.clone();
        let permission_show = move |req: &mut Request| -> AranResult<Response> { _self.permission_show(req) };

        let _self = self.clone();
        let list_permissions_by_role = move |req: &mut Request| -> AranResult<Response> { _self.list_permissions_by_role(req) };

        let _self = self.clone();
        let show_permissions_applied_for = move |req: &mut Request| -> AranResult<Response> { _self.show_permissions_applied_for(req) };

        //Routes:  Authorization : Permissions
        router.post(
            "/permissions",
            XHandler::new(C { inner: permission_create }).before(basic.clone()),
            "permissions",
        );
        router.get(
            "/permissions",
            XHandler::new(C { inner: permission_list }).before(basic.clone()),
            "permission_list",
        );
        router.get(
            "/permissions/roles/:role_id",
            XHandler::new(C { inner: list_permissions_by_role }).before(basic.clone()),
            "list_permissions_by_role",
        );
        router.get(
            "/permissions/:id",
            XHandler::new(C { inner: permission_show }).before(basic.clone()),
            "permission_show",
        );
        router.get(
            "/permissions/:id/roles/:role_id",
            XHandler::new(C { inner: show_permissions_applied_for }).before(basic.clone()),
            "show_permissions_applied_for",
        );

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

        if self.get_role_id().len() <= 0 {
            s.push("role_id".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
