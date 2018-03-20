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

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

/// TO_DO: Should be named  (authorize::models::roles, authorize::models::permission)
use authorize::models::authorize;
use protocol::api::authorize::{Roles, Permissions};
use protocol::api::base::IdGet;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::{MustBeNumeric, MissingParameter};

/// Authorize api: AuthorizeApi provides ability to declare the roles and Permissions
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Secret: URLs supported are.
/// POST: /roles,,
/// GET: /roles,
/// GET: /roles/:id,
#[derive(Clone)]
pub struct AuthorizeApi {
    conn: Box<DataStoreConn>,
}

impl AuthorizeApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        AuthorizeApi { conn: datastore }
    }
    //POST: /roles
    //The body has the input cluster::roles
    //Returns a mutated Roles  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn role_create(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate(req.get::<bodyparser::Struct<Roles>>()?)?;

        match authorize::DataStore::roles_create(&self.conn, &unmarshall_body) {
            Ok(Some(roles_create)) => Ok(render_json(status::Ok, &roles_create)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /roles/:id
    //Input id - u64 as input and returns a roles
    fn role_show(&self, req: &mut Request) -> AranResult<Response> {
        let id = {
            let params = req.extensions.get::<Router>().unwrap();
            match params.find("id").unwrap().parse::<u64>() {
                Ok(id) => id,
                Err(_) => return Err(bad_request(&MustBeNumeric("id".to_string()))),
            }
        };

        match authorize::DataStore::roles_show(&self.conn, &IdGet::with_id(id.clone().to_string())) {
            Ok(Some(roles)) => Ok(render_json(status::Ok, &roles)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                id
            ))),
        }
    }
    //GET: /roles
    //Returns all the roles(irrespective of namespaces)
    fn role_list(&self, req: &mut Request) -> AranResult<Response> {
        match authorize::DataStore::roles_list(&self.conn) {
            Ok(Some(roles_list)) => Ok(render_json_list(status::Ok, dispatch(req), &roles_list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //POST: /permissions
    //The body has the input cluster::permissions
    //Returns a mutated Permissions  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn permission_create(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate(req.get::<bodyparser::Struct<Permissions>>()?)?;
        match authorize::DataStore::permissions_create(&self.conn, &unmarshall_body) {
            Ok(Some(permissions_create)) => Ok(render_json(status::Ok, &permissions_create)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /permission
    //Returns all the permissions(irrespective of namespaces)
    fn permission_list(&self, _req: &mut Request) -> AranResult<Response> {
        match authorize::DataStore::permissions_list(&self.conn) {
            Ok(Some(permissions_list)) => Ok(render_json_list(
                status::Ok,
                dispatch(_req),
                &permissions_list,
            )),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //Send in the role id and get the permissions.
    pub fn show_permissions_by_role(&self, req: &mut Request) -> AranResult<Response> {
        let id = {
            let params = req.extensions.get::<Router>().unwrap();
            match params.find("id").unwrap().parse::<u64>() {
                Ok(id) => id,
                Err(_) => return Err(bad_request(&MustBeNumeric("id".to_string()))),
            }
        };

        match authorize::DataStore::get_rolebased_permissions(&self.conn, &IdGet::with_id(id.clone().to_string())) {
            Ok(Some(permission)) => Ok(render_json(status::Ok, &permission)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                id
            ))),
        }
    }
    //GET: /permission/:id
    //Input id - u64 as input and returns a permission
    fn permission_show(&self, req: &mut Request) -> AranResult<Response> {
        let id = {
            let params = req.extensions.get::<Router>().unwrap();
            match params.find("id").unwrap().parse::<u64>() {
                Ok(id) => id,
                Err(_) => return Err(bad_request(&MustBeNumeric("id".to_string()))),
            }
        };

        match authorize::DataStore::permissions_show(&self.conn, &IdGet::with_id(id.clone().to_string())) {
            Ok(Some(perms)) => Ok(render_json(status::Ok, &perms)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                id
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

        match authorize::DataStore::get_specfic_permission_based_role(&self.conn, &perms_get) {
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

impl Api for AuthorizeApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : roles
        let _self = self.clone();
        let role_create = move |req: &mut Request| -> AranResult<Response> { _self.role_create(req) };

        let _self = self.clone();
        let role_list = move |req: &mut Request| -> AranResult<Response> { _self.role_list(req) };

        let _self = self.clone();
        let role_show = move |req: &mut Request| -> AranResult<Response> { _self.role_show(req) };

        //closures : permissions
        let _self = self.clone();
        let permission_create = move |req: &mut Request| -> AranResult<Response> { _self.permission_create(req) };

        let _self = self.clone();
        let permission_list = move |req: &mut Request| -> AranResult<Response> { _self.permission_list(req) };

        let _self = self.clone();
        let permission_show = move |req: &mut Request| -> AranResult<Response> { _self.permission_show(req) };

        let _self = self.clone();
        let show_permissions_by_role = move |req: &mut Request| -> AranResult<Response> { _self.show_permissions_by_role(req) };

        let _self = self.clone();
        let show_permissions_applied_for = move |req: &mut Request| -> AranResult<Response> { _self.show_permissions_applied_for(req) };

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

        //Routes:  Authorization : Permissions
        router.post(
            "/permissions",
            XHandler::new(C {
                inner: permission_create,
            }).before(basic.clone()),
            "permissions",
        );
        router.get(
            "/permissions",
            XHandler::new(C {
                inner: permission_list,
            }).before(basic.clone()),
            "permission_list",
        );
        router.get(
            "/permissions/roles/:role_id",
            XHandler::new(C {
                inner: show_permissions_by_role,
            }).before(basic.clone()),
            "show_permissions_by_role",
        );
        router.get(
            "/permissions/:id",
            XHandler::new(C {
                inner: permission_show,
            }).before(basic.clone()),
            "permission_show",
        );
        router.get(
            "/permissions/:id/roles/:role_id",
            XHandler::new(C {
                inner: show_permissions_applied_for,
            }).before(basic.clone()),
            "show_permissions_applied_for",
        );
    }
}

impl ApiValidator for AuthorizeApi {}

impl ParmsVerifier for AuthorizeApi {}

impl Validator for Permissions {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for Roles {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
