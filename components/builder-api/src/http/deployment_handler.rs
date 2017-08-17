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

use protocol::asmsrv::{Assembly, AssemblyGet, AssemblyFactory, AssemblyFactoryGet, Status, Condition, ComponentCollection, Properties, OpsSettings};
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
    parent_id: String,
    description: String,
    node: String,
    status: StatusReq,
    ip: String,
    urls: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusReq {
    phase: String,
    message: String,
    reason: String,
    conditions: Vec<ConditionReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConditionReq {
    message: String,
    reason: String,
    status: String,
    lastTransitionTime: String,
    lastProbeTime: String,
    conditionType: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CommonStatusReq {
    status: StatusReq,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyFacCreateReq {
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    properties: PropReq,
    replicas: u64,
    plan: String,
    external_management_resource: Vec<String>,
    component_collection: ComponentReq,
    status: StatusReq,
    opssettings: OpsSettingsReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PropReq {
    domain: String,
    cloudsetting: String,
    region: String,
    storage_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ComponentReq {
    flavor: String,
    network: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpsSettingsReq {
    nodeselector: String,
    priority: String,
    nodename: String,
    restartpolicy: String,
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
                if body.parent_id.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `parent_id`",
                    )));
                }
                assembly_create.set_name(body.name);
                assembly_create.set_uri(body.uri);
                assembly_create.set_description(body.description);
                assembly_create.set_tags(body.tags);
                assembly_create.set_parent_id(body.parent_id);
                assembly_create.set_node(body.node);
                let mut status = Status::new();
                status.set_phase(body.status.phase);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);

                let mut condition_collection = Vec::new();

                for data in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_message(data.message);
                    condition.set_reason(data.reason);
                    condition.set_status(data.status);
                    condition.set_lastTransitionTime(data.lastTransitionTime);
                    condition.set_lastProbeTime(data.lastProbeTime);
                    condition.set_conditionType(data.conditionType);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);
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
    asm_get.set_id(id.to_string());

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
    assembly.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<CommonStatusReq>>() {
            Ok(Some(body)) => {
                let mut status = Status::new();
                status.set_phase(body.status.phase);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);
                let mut condition_collection = Vec::new();
                for data in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_message(data.message);
                    condition.set_reason(data.reason);
                    condition.set_status(data.status);
                    condition.set_lastTransitionTime(data.lastTransitionTime);
                    condition.set_lastProbeTime(data.lastProbeTime);
                    condition.set_conditionType(data.conditionType);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);
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

                let mut component_collection = ComponentCollection::new();
                component_collection.set_flavor(body.component_collection.flavor);
                component_collection.set_network(body.component_collection.network);

                assembly_factory_create.set_component_collection(component_collection);

                let mut status = Status::new();
                status.set_phase(body.status.phase);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);
                let mut condition_collection = Vec::new();
                for data in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_message(data.message);
                    condition.set_reason(data.reason);
                    condition.set_status(data.status);
                    condition.set_lastTransitionTime(data.lastTransitionTime);
                    condition.set_lastProbeTime(data.lastProbeTime);
                    condition.set_conditionType(data.conditionType);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);
                assembly_factory_create.set_status(status);
                let mut opssettings = OpsSettings::new();
                opssettings.set_nodeselector(body.opssettings.nodeselector);
                opssettings.set_priority(body.opssettings.priority);
                opssettings.set_nodename(body.opssettings.nodename);
                opssettings.set_restartpolicy(body.opssettings.restartpolicy);
                assembly_factory_create.set_opssettings(opssettings);
                assembly_factory_create.set_replicas(body.replicas);
                let mut properties = Properties::new();
                properties.set_cloudsetting(body.properties.cloudsetting);
                properties.set_domain(body.properties.domain);
                properties.set_region(body.properties.region);
                properties.set_storage_type(body.properties.storage_type);
                assembly_factory_create.set_properties(properties);

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
    asm_fac_get.set_id(id.to_string());

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
    assembly_factory.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<CommonStatusReq>>() {
            Ok(Some(body)) => {
                let mut status = Status::new();
                status.set_phase(body.status.phase);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);
                let mut condition_collection = Vec::new();
                for data in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_message(data.message);
                    condition.set_reason(data.reason);
                    condition.set_status(data.status);
                    condition.set_lastTransitionTime(data.lastTransitionTime);
                    condition.set_lastProbeTime(data.lastProbeTime);
                    condition.set_conditionType(data.conditionType);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);
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
