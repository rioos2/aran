// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use bodyparser;
use rio_net::http::controller::*;
use authorize::authorize_ds::AuthorizeDS;
use iron::prelude::*;
use iron::status;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;
use protocol::authsrv::{Roles, Permissions};
use protocol::asmsrv::IdGet;
use error::{Result, Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};
use db;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesCreateReq {
    name: String,
    description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsCreateReq {
    role_id: String,
    name: String,
    description: String,
}

pub fn roles_create(req: &mut Request) -> AranResult<Response> {
    let mut roles = Roles::new();
    {
        match req.get::<bodyparser::Struct<RolesCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "name")));

                }
                roles.set_name(body.name);
                roles.set_description(body.description);

            }
            Err(err) => {
                return Err(malformed_body(&format!("{}, {:?}\n", err.detail, err.cause),));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }
    let conn = Broker::connect().unwrap();

    match AuthorizeDS::roles_create(&conn, &roles) {
        Ok(Some(roles_create)) => Ok(render_json(status::Ok, &roles_create)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }


    }
}


pub fn roles_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    let conn = Broker::connect().unwrap();

    let mut roles_get = IdGet::new();
    roles_get.set_id(id.to_string());

    match AuthorizeDS::roles_show(&conn, &roles_get) {
        Ok(Some(roles)) => Ok(render_json(status::Ok, &roles)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &roles_get.get_id()
            )))
        }
    }
}

#[allow(unused_variables)]
pub fn roles_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();

    match AuthorizeDS::roles_list(&conn) {
        Ok(Some(roles_list)) => Ok(render_json(status::Ok, &roles_list)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn permissions_create(req: &mut Request) -> AranResult<Response> {
    let mut permissions = Permissions::new();
    {
        match req.get::<bodyparser::Struct<PermissionsCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "name")));

                }

                if body.role_id.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "role id")));

                }
                permissions.set_role_id(body.role_id);
                permissions.set_name(body.name);
                permissions.set_description(body.description);

            }
            Err(err) => {
                return Err(malformed_body(&format!("{}, {:?}\n", err.detail, err.cause),));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }
    let conn = Broker::connect().unwrap();

    match AuthorizeDS::permissions_create(&conn, &permissions) {
        Ok(Some(permissions_create)) => Ok(render_json(status::Ok, &permissions_create)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }

    }
}

#[allow(unused_variables)]
pub fn permissions_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match AuthorizeDS::permissions_list(&conn) {
        Ok(Some(permissions_list)) => Ok(render_json(status::Ok, &permissions_list)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn get_rolebased_permissions(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut perm_get = IdGet::new();
    perm_get.set_id(id.to_string());

    match AuthorizeDS::get_rolebased_permissions(&conn, &perm_get) {
        Ok(Some(permission)) => Ok(render_json(status::Ok, &permission)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &perm_get.get_id()
            )))
        }
    }
}

pub fn permissions_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut perms_get = IdGet::new();
    perms_get.set_id(id.to_string());
    match AuthorizeDS::permissions_show(&conn, &perms_get) {
        Ok(Some(perms)) => Ok(render_json(status::Ok, &perms)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &perms_get.get_id()
            )))
        }
    }
}

pub fn get_specfic_permission_based_role(req: &mut Request) -> AranResult<Response> {

    let (perm_id, role_id) = {
        let params = req.extensions.get::<Router>().unwrap();
        let perm_id = params.find("id").unwrap().to_owned();
        let role_id = params.find("rid").unwrap().to_owned();

        (perm_id, role_id)
    };
    let conn = Broker::connect().unwrap();

    let mut perms_get = IdGet::new();
    perms_get.set_id(perm_id);
    perms_get.set_name(role_id);
    match AuthorizeDS::get_specfic_permission_based_role(&conn, &perms_get) {
        Ok(Some(perms)) => Ok(render_json(status::Ok, &perms)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &perms_get.get_id()
            )))
        }
    }
}
