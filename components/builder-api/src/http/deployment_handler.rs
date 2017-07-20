// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use std::env;

use base64;
use bodyparser;
use hab_core::package::Plan;
use hab_core::event::*;
use hab_net;
use hab_net::http::controller::*;
use hab_net::routing::Broker;
use deploy::deployment_ds::DeploymentDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use params::{Params, Value, FromValue};
use persistent;

use protocol::asmsrv::{Assembly, AssemblyGet};
use protocol::sessionsrv;
use protocol::net::{self, NetOk, ErrCode};
use router::Router;
use db::data_store::DataStoreBroker;

// For the initial release, Builder will only be enabled on the "core"
// origin. Later, we'll roll it out to other origins; at that point,
// we should consider other options, such as configurable middleware.

define_event_log!();

#[derive(Clone, Serialize, Deserialize)]
struct AssemblyCreateReq {
    name: String,
    uri: String,
    description:String,
    tags: String,
    representation_skew: String,
    external_management_resource: String,
    component_collection: String,
    plan:String,
    operation_collection: String,
    sensor_collection: String,
    metadata: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct AssemblyUpdateReq {
    uri: u64,
    name: String,
    description:String,
    tags: String,
    representation_skew: String,
    external_management_resource: String,
    component_collection: String,
    plan:String,
    operation_collection: String,
    sensor_collection: String,
    metadata: String,
}

pub fn assembly_create(req: &mut Request) -> IronResult<Response> {
    let mut assembly_create = Assembly::new();
    {
        match req.get::<bodyparser::Struct<AssemblyCreateReq>>() {
            Ok(Some(body)) => {
                //TO-DO Check for validity as per your need
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
    let conn = req.extensions.get::<DataStoreBroker>().unwrap();
    //This is needed as you'll need the email/token if any
    // let session = req.extensions.get::<Authenticated>().unwrap().clone();

    match DeploymentDS::assembly_create(&conn, &assembly_create) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(&net::err(ErrCode::DATA_STORE, format!("{}\n", err)))),

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
    let conn = req.extensions.get::<DataStoreBroker>().unwrap();
    //This is needed as you'll need the email/token if any
    // let session = req.extensions.get::<Authenticated>().unwrap().clone();

    match DeploymentDS::assembly_create(&conn, &assembly_create) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(&net::err(ErrCode::DATA_STORE, format!("{}\n", err)))),

    }
}

pub fn assembly_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let id = match params.find("id").unwrap().parse::<u64>() {
        Ok(id) => id,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    let datastore = req.extensions.get::<DataStoreBroker>().unwrap();
    let mut request = AssemblyGet::new();
    request.set_id(id);

    match DeploymentDS::assembly_show(&datastore, &request) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(&net::err(ErrCode::ACCESS_DENIED, "err"))),
    }
}


/// Endpoint for determining availability of builder-api components.
///
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
