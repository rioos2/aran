// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory] for the HTTP server

use std::env;

use bodyparser;
use hab_core::event::*;
use hab_net::http::controller::*;
use deploy::deployment_ds::DeploymentDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;

use protocol::asmsrv::{Assembly, AssemblyGet, AssemblyFactory, AssemblyFactoryGet, AssemblyFactoryStatus, AssemblyStatus};
use protocol::sessionsrv;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::DataStoreBroker;


define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyCreateReq {
    name: String,
    uri: String,
    tags: Vec<String>,
    parent_id: u64,
    description: String,
    node: String,
    status: String,
    ip: String,
    urls: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyFacCreateReq {
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    properties: String,
    replicas: u64,
    plan: String,
    external_management_resource: Vec<String>,
    component_collection: String,
    status: String,
    opssettings: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyUpdateReq {
    name: String,
    uri: String,
    description: String,
    parent_id: u64,
    tags: Vec<String>,
    node: String,
    ip: String,
    urls: String,
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyFacStatusReq {
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyStatusReq {
    status: String,
}



pub fn assembly_create(req: &mut Request) -> IronResult<Response> {
    let mut assembly_create = Assembly::new();
    {
        match req.get::<bodyparser::Struct<AssemblyCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                assembly_create.set_name(body.name);
                assembly_create.set_uri(body.uri);
                assembly_create.set_description(body.description);
                assembly_create.set_tags(body.tags);
                assembly_create.set_parent_id(body.parent_id);
                assembly_create.set_node(body.node);
                let status = AssemblyStatus::from_str(body.status);
                assembly_create.set_status(status);
                assembly_create.set_ip(body.ip);
                assembly_create.set_urls(body.urls);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();
    //This is needed as you'll need the email/token if any
    // let session = req.extensions.get::<Authenticated>().unwrap().clone();
    match DeploymentDS::assembly_create(&conn, &assembly_create) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn assembly_show(req: &mut Request) -> IronResult<Response> {

    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();

    let mut asm_get = AssemblyGet::new();
    asm_get.set_id(id);

    match DeploymentDS::assembly_show(&conn, &asm_get) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn assembly_list(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();
    match DeploymentDS::assembly_list(&conn) {
        Ok(assembly_list) => Ok(render_json(status::Ok, &assembly_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}


pub fn assembly_status_update(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };
    let mut assembly = Assembly::new();
    assembly.set_id(id);
    {
        match req.get::<bodyparser::Struct<AssemblyStatusReq>>() {
            Ok(Some(body)) => {
                let status = AssemblyStatus::from_str(body.status);
                assembly.set_status(status);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();

    //This is needed as you'll need the email/token if any
    // let session = req.extensions.get::<Authenticated>().unwrap().clone();

    match DeploymentDS::assembly_status_update(&conn, &assembly) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn assembly_factory_create(req: &mut Request) -> IronResult<Response> {
    let mut assembly_factory_create = AssemblyFactory::new();
    {
        match req.get::<bodyparser::Struct<AssemblyFacCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                assembly_factory_create.set_name(body.name);
                assembly_factory_create.set_uri(body.uri);
                assembly_factory_create.set_description(body.description);
                assembly_factory_create.set_tags(body.tags);
                assembly_factory_create.set_external_management_resource(body.external_management_resource);
                assembly_factory_create.set_plan(body.plan);
                assembly_factory_create.set_component_collection(body.component_collection);
                let status = AssemblyFactoryStatus::from_str(body.status);
                assembly_factory_create.set_status(status);
                assembly_factory_create.set_opssettings(body.opssettings);
                assembly_factory_create.set_replicas(body.replicas);
                assembly_factory_create.set_properties(body.properties);

            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();

    //This is needed as you'll need the email/token if any
    // let session = req.extensions.get::<Authenticated>().unwrap().clone();

    match DeploymentDS::assembly_factory_create(&conn, &assembly_factory_create) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn assembly_factory_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();

    let mut asm_fac_get = AssemblyFactoryGet::new();
    asm_fac_get.set_id(id);

    match DeploymentDS::assembly_factory_show(&conn, &asm_fac_get) {
        Ok(assembly_factory) => Ok(render_json(status::Ok, &assembly_factory)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn assembly_factory_status_update(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };
    let mut assembly_factory = AssemblyFactory::new();
    assembly_factory.set_id(id);
    {
        match req.get::<bodyparser::Struct<AssemblyFacStatusReq>>() {
            Ok(Some(body)) => {
                let status = AssemblyFactoryStatus::from_str(body.status);
                assembly_factory.set_status(status);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();

    //This is needed as you'll need the email/token if any
    // let session = req.extensions.get::<Authenticated>().unwrap().clone();

    match DeploymentDS::assembly_factory_status_update(&conn, &assembly_factory) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn assembly_factory_list(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();
    match DeploymentDS::assembly_factory_list(&conn) {
        Ok(assembly_list) => Ok(render_json(status::Ok, &assembly_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

/// Endpoint for determining availability of builder-api components.
///
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
