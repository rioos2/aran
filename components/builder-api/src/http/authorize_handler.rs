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
use protocol::authsrv::{Roles, Permissions, PermissionsGet, RolesGet};


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

pub fn roles_create(req: &mut Request) -> IronResult<Response> {
    let mut roles = Roles::new();
    {
        match req.get::<bodyparser::Struct<RolesCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                roles.set_name(body.name);
                roles.set_description(body.description);

            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = Broker::connect().unwrap();

    match AuthorizeDS::roles_create(&conn, &roles) {
        Ok(roles_create) => Ok(render_json(status::Ok, &roles_create)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn roles_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };
    let conn = Broker::connect().unwrap();

    let mut roles_get = RolesGet::new();
    roles_get.set_id(id.to_string());

    match AuthorizeDS::roles_show(&conn, &roles_get) {
        Ok(roles) => Ok(render_json(status::Ok, &roles)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

#[allow(unused_variables)]
pub fn roles_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();

    match AuthorizeDS::roles_list(&conn) {
        Ok(roles_list) => Ok(render_json(status::Ok, &roles_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn permissions_create(req: &mut Request) -> IronResult<Response> {
    let mut permissions = Permissions::new();
    {
        match req.get::<bodyparser::Struct<PermissionsCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                permissions.set_role_id(body.role_id);
                permissions.set_name(body.name);
                permissions.set_description(body.description);

            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = Broker::connect().unwrap();

    match AuthorizeDS::permissions_create(&conn, &permissions) {
        Ok(permissions_create) => Ok(render_json(status::Ok, &permissions_create)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

#[allow(unused_variables)]
pub fn permissions_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match AuthorizeDS::permissions_list(&conn) {
        Ok(permissions_list) => Ok(render_json(status::Ok, &permissions_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn get_rolebased_permissions(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut perm_get = PermissionsGet::new();
    perm_get.set_id(id.to_string());

    match AuthorizeDS::get_rolebased_permissions(&conn, &perm_get) {
        Ok(permission) => Ok(render_json(status::Ok, &permission)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn permissions_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut perms_get = PermissionsGet::new();
    perms_get.set_id(id.to_string());
    match AuthorizeDS::permissions_show(&conn, &perms_get) {
        Ok(perms) => Ok(render_json(status::Ok, &perms)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn get_specfic_permission_based_role(req: &mut Request) -> IronResult<Response> {

    let (perm_id, role_id) = {
        let params = req.extensions.get::<Router>().unwrap();
        let perm_id = params.find("id").unwrap().to_owned();
        let role_id = params.find("rid").unwrap().to_owned();

        (perm_id, role_id)
    };
    let conn = Broker::connect().unwrap();

    let mut perms_get = PermissionsGet::new();
    perms_get.set_id(perm_id);
    perms_get.set_role_id(role_id);
    match AuthorizeDS::get_specfic_permission_based_role(&conn, &perms_get) {
        Ok(perms) => Ok(render_json(status::Ok, &perms)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}
