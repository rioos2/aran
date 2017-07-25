// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use std::env;

use bodyparser;
use hab_core::event::*;
use hab_net::http::controller::*;
use deploy::deployment_ds::DeploymentDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;

use protocol::asmsrv::{Assembly, AssemblyGet, AssemblyFactory, AssemblyFactoryGet};
use protocol::sessionsrv;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::DataStoreBroker;

// For the initial release, Builder will only be enabled on the "core"
// origin. Later, we'll roll it out to other origins; at that point,
// we should consider other options, such as configurable middleware.

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyCreateReq {
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    representation_skew: String,
    external_management_resource: String,
    component_collection: Vec<String>,
    plan: String,
    operation_collection: Vec<String>,
    sensor_collection: Vec<String>,
    metadata: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyFacCreateReq {
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    representation_skew: String,
    total_items: u64,
    items_per_page: u64,
    start_index: u64,
    items: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyUpdateReq {
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    representation_skew: String,
    external_management_resource: String,
    component_collection: Vec<String>,
    plan: String,
    operation_collection: Vec<String>,
    sensor_collection: Vec<String>,
    metadata: String,
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
                assembly_create.set_representation_skew(body.representation_skew);
                assembly_create.set_external_management_resource(body.external_management_resource);
                assembly_create.set_component_collection(body.component_collection);
                assembly_create.set_plan(body.plan);
                assembly_create.set_operation_collection(body.operation_collection);
                assembly_create.set_sensor_collection(body.sensor_collection);
                assembly_create.set_metadata(body.metadata);
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
                assembly_factory_create.set_representation_skew(body.representation_skew);
                assembly_factory_create.set_total_items(body.total_items);
                assembly_factory_create.set_items_per_page(body.items_per_page);
                assembly_factory_create.set_start_index(body.start_index);
                assembly_factory_create.set_items(body.items);
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

pub fn assembly_update(req: &mut Request) -> IronResult<Response> {
    let mut assembly_create = Assembly::new();
    {
        match req.get::<bodyparser::Struct<AssemblyCreateReq>>() {
            Ok(Some(body)) => {
                //TO-DO Check for validity as per your need
                assembly_create.set_name(body.name)
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
