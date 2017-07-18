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
use deploy::deploy_ds::DeploymentDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use params::{Params, Value, FromValue};
use persistent;
//TO-DO change the protocol::jobsrv to protocol::deployment
//       add Assembly, AssemblyGet
use protocol::message::asmsrv::{Assembly, AssemblyGet};

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
    id: String,
    uri: String,
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
    updated_at: String,
    created_at: String
}

//TO-DO please change as per your datamodel when we activate update
#[derive(Clone, Serialize, Deserialize)]
struct AssemblyUpdateReq {
    plan_path: String,
    github: GitHubProject,
}


pub fn assembly_create(req: &mut Request) -> IronResult<Response> {
    let mut assembly_create = AssemblyCreate::new();
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
                assembly_create.set_name(body.name)
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = req.get::<DataStoreBroker>().unwrap();
    //This is needed as you'll need the email/token if any
    let session = req.extensions.get::<Authenticated>().unwrap().clone();

    match DeploymentDS::assembly_create(&conn, &assembly_create) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(&err)),
    }
}

pub fn assembly_show(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let id = match params.find("id").unwrap().parse::<u64>() {
        Ok(id) => id,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };

    let datastore = req.get::<DataStoreBroker>().unwrap();

    let mut request = AssemblyGet::new();
    request.set_id(id);

    match DeploymentDS::assembly_show(&datastore, &request) {
        Ok(assembly) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Ok(render_net_error(&err)),
    }
}


/// Endpoint for determining availability of builder-api components.
///
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
