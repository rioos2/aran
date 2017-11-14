// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory] for the HTTP server

use bodyparser;
use ansi_term::Colour;
use rio_core::event::*;
use rio_net::http::controller::*;
use deploy::deployment_ds::DeploymentDS;
use deploy::replicas::Replicas;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::asmsrv::{Assembly, IdGet, AssemblyFactory, Status, Condition, Properties, OpsSettings, Volume, TypeMeta, INITIAL_CONDITIONS, NEW_REPLICA_INITALIZING, INITIALIZING};
use protocol::plansrv::{Plan, Service};
use router::Router;
use db::data_store::Broker;
use std::collections::BTreeMap;
use common::ui;
use db;
use error::{Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};
const ASSEMBLYFACTORY: &'static str = "AssemblyFactory";
const ASSEMBLY: &'static str = "Assembly";
define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyCreateReq {
    name: String,
    uri: String,
    tags: Vec<String>,
    parent_id: String,
    origin: String,
    description: String,
    node: String,
    status: StatusReq,
    urls: BTreeMap<String, String>,
    volumes: Vec<VolumeReq>,
    instance_id: String,
    selector: Vec<String>,
    type_meta: TypeMetaReq,
    object_meta: ObjectMetaDataReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusReq {
    pub phase: String,
    pub message: String,
    pub reason: String,
    pub conditions: Vec<ConditionReq>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ObjectMetaDataReq {
    pub name: String,
    pub origin: String,
    pub uid: String,
    pub created_at: String,
    pub cluster_name: String,
    pub labels: BTreeMap<String, String>,
    pub annotations: BTreeMap<String, String>,
    pub owner_references: Vec<OwnerReferencesReq>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerReferencesReq {
    pub kind: String,
    pub api_version: String,
    pub name: String,
    pub uid: String,
    pub block_owner_deletion: bool,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConditionReq {
    pub message: String,
    pub reason: String,
    pub status: String,
    pub last_transition_time: String,
    pub last_probe_time: String,
    pub condition_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CommonStatusReq {
    status: StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeReq {
    id: u32,
    target: String,
    volume_type: String,
    size: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyFacCreateReq {
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    origin: String,
    properties: PropReq,
    replicas: u32,
    plan: String,
    external_management_resource: Vec<String>,
    component_collection: BTreeMap<String, String>,
    status: StatusReq,
    opssettings: OpsSettingsReq,
    type_meta: TypeMetaReq,
    object_meta: ObjectMetaDataReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PropReq {
    domain: String,
    cloudsetting: String,
    region: String,
    storage_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeMetaReq {
    pub kind: String,
    pub api_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpsSettingsReq {
    nodeselector: String,
    priority: String,
    nodename: String,
    restartpolicy: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlanCreateReq {
    group_name: String,
    url: String,
    description: String,
    tags: Vec<String>,
    origin: String,
    artifacts: Vec<String>,
    services: Vec<ServiceReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ServiceReq {
    name: String,
    description: String,
    href: String,
    characteristics: BTreeMap<String, String>,
}


pub fn assembly_create(req: &mut Request) -> AranResult<Response> {

    let mut assembly_create = Assembly::new();
    {
        match req.get::<bodyparser::Struct<AssemblyCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "name")));
                }
                if body.parent_id.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "parent_id")));
                }
                if body.origin.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "origin")));
                }

                assembly_create.set_name(body.name);
                assembly_create.set_uri(body.uri);
                assembly_create.set_description(body.description);
                assembly_create.set_tags(body.tags);
                assembly_create.set_selector(body.selector);
                assembly_create.set_parent_id(body.parent_id);
                assembly_create.set_origin(body.origin);
                assembly_create.set_status(Status::with_conditions(
                    INITIALIZING,
                    NEW_REPLICA_INITALIZING,
                    "",
                    INITIAL_CONDITIONS
                        .iter()
                        .map(|x| Condition::with_type("", "", "False", "", "", x))
                        .collect::<Vec<_>>(),
                ));
                assembly_create.set_urls(body.urls);
                assembly_create.set_type_meta(TypeMeta::new(ASSEMBLY));
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }

    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", assembly_create),
    );

    let conn = Broker::connect().unwrap();

    match DeploymentDS::assembly_create(&conn, &assembly_create) {
        Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn assembly_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut asm_get = IdGet::new();
    asm_get.set_id(id.to_string());

    match DeploymentDS::assembly_show(&conn, &asm_get) {
        Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &asm_get.get_id()
            )))
        }
    }
}

#[allow(unused_variables)]
pub fn assembly_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match DeploymentDS::assembly_list(&conn) {
        Ok(Some(assembly_list)) => Ok(render_json(status::Ok, &assembly_list)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn assemblys_show_by_origin(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };

    let conn = Broker::connect().unwrap();

    let mut assemblys_get = IdGet::new();
    assemblys_get.set_id(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", assemblys_get),
    );
    match DeploymentDS::assemblys_show_by_origin(&conn, &assemblys_get) {
        Ok(Some(assemblys)) => Ok(render_json(status::Ok, &assemblys)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &assemblys_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}", err))),
    }
}


pub fn assembly_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(IDMUSTNUMBER)),
        }
    };
    let mut assembly_create = Assembly::new();
    {
        match req.get::<bodyparser::Struct<AssemblyCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "name")));
                }
                if body.parent_id.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "parent_id")));
                }
                assembly_create.set_id(id.to_string());
                assembly_create.set_name(body.name);
                assembly_create.set_uri(body.uri);
                assembly_create.set_description(body.description);
                assembly_create.set_tags(body.tags);
                assembly_create.set_parent_id(body.parent_id);
                assembly_create.set_node(body.node);
                assembly_create.set_urls(body.urls);
                assembly_create.set_volumes(
                    body.volumes
                        .iter()
                        .map(|x| {
                            Volume::with_volumes(x.id, &x.target, &x.volume_type, &x.size)
                        })
                        .collect::<Vec<_>>(),
                );
                assembly_create.set_type_meta(TypeMeta::new(ASSEMBLY));
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match DeploymentDS::assembly_update(&conn, &assembly_create) {
        Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &assembly_create.get_id()
            )))
        }

    }
}

pub fn assembly_status_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(IDMUSTNUMBER)),
        }
    };
    let mut assembly = Assembly::new();
    assembly.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<CommonStatusReq>>() {
            Ok(Some(body)) => {
                assembly.set_status(Status::with_conditions(
                    &body.status.phase,
                    &body.status.message,
                    &body.status.reason,
                    body.status
                        .conditions
                        .iter()
                        .map(|x| {
                            Condition::with_type(
                                &x.message,
                                &x.reason,
                                &x.status,
                                &x.last_transition_time,
                                &x.last_probe_time,
                                &x.condition_type,
                            )
                        })
                        .collect::<Vec<_>>(),
                ));
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match DeploymentDS::assembly_status_update(&conn, &assembly) {
        Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &assembly.get_id()
            )))
        }

    }
}


pub fn assembly_factory_create(req: &mut Request) -> AranResult<Response> {
    let mut assembly_factory_create = AssemblyFactory::new();
    {
        match req.get::<bodyparser::Struct<AssemblyFacCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "name")));
                }
                if body.origin.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "origin")));
                }
                assembly_factory_create.set_name(body.name);
                assembly_factory_create.set_uri(body.uri);
                assembly_factory_create.set_description(body.description);
                assembly_factory_create.set_tags(body.tags);
                assembly_factory_create.set_origin(body.origin);
                assembly_factory_create.set_external_management_resource(body.external_management_resource);
                assembly_factory_create.set_plan(body.plan);
                assembly_factory_create.set_component_collection(body.component_collection);
                assembly_factory_create.set_status(Status::with_conditions(
                    &body.status.phase,
                    &body.status.message,
                    &body.status.reason,
                    body.status
                        .conditions
                        .iter()
                        .map(|x| {
                            Condition::with_type(
                                &x.message,
                                &x.reason,
                                &x.status,
                                &x.last_transition_time,
                                &x.last_probe_time,
                                &x.condition_type,
                            )
                        })
                        .collect::<Vec<_>>(),
                ));
                assembly_factory_create.set_replicas(body.replicas);
                assembly_factory_create.set_opssettings(OpsSettings::new(
                    &body.opssettings.nodeselector,
                    &body.opssettings.priority,
                    &body.opssettings.nodename,
                    &body.opssettings.restartpolicy,
                ));
                assembly_factory_create.set_properties(Properties::new(
                    &body.properties.cloudsetting,
                    &body.properties.domain,
                    &body.properties.region,
                    &body.properties.storage_type,
                ));
                assembly_factory_create.set_type_meta(TypeMeta::new(ASSEMBLYFACTORY));
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", assembly_factory_create),
    );

    let conn = Broker::connect().unwrap();

    match Replicas::new(
        &conn,
        0,
        assembly_factory_create.get_replicas(),
        &assembly_factory_create,
    ).new_desired() {
        Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}


pub fn assembly_factory_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut asm_fac_get = IdGet::new();
    asm_fac_get.set_id(id.to_string());

    match DeploymentDS::assembly_factory_show(&conn, &asm_fac_get) {
        Ok(Some(assembly_factory)) => Ok(render_json(status::Ok, &assembly_factory)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &asm_fac_get.get_id()
            )))
        }
    }
}

pub fn assembly_factory_status_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    let mut assembly_factory = AssemblyFactory::new();
    assembly_factory.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<CommonStatusReq>>() {
            Ok(Some(body)) => {
                assembly_factory.set_status(Status::with_conditions(
                    &body.status.phase,
                    &body.status.message,
                    &body.status.reason,
                    body.status
                        .conditions
                        .iter()
                        .map(|x| {
                            Condition::with_type(
                                &x.message,
                                &x.reason,
                                &x.status,
                                &x.last_transition_time,
                                &x.last_probe_time,
                                &x.condition_type,
                            )
                        })
                        .collect::<Vec<_>>(),
                ));
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match DeploymentDS::assembly_factory_status_update(&conn, &assembly_factory) {
        Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &assembly_factory.get_id()
            )))
        }

    }
}

#[allow(unused_variables)]
pub fn assembly_factory_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match DeploymentDS::assembly_factory_list(&conn) {
        Ok(Some(assembly_list)) => Ok(render_json(status::Ok, &assembly_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn assemblyfactorys_list_by_origin(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };

    let conn = Broker::connect().unwrap();

    let mut assemblyfactory_get = IdGet::new();
    assemblyfactory_get.set_id(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", assemblyfactory_get),
    );
    match DeploymentDS::assemblyfactorys_show_by_origin(&conn, &assemblyfactory_get) {
        Ok(Some(assemblyfac)) => Ok(render_json(status::Ok, &assemblyfac)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &assemblyfactory_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}

pub fn assembly_factorys_describe(req: &mut Request) -> AranResult<Response> {

    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut assemblydes_get = IdGet::new();
    assemblydes_get.set_id(id.to_string());

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", assemblydes_get),
    );
    match DeploymentDS::assembly_factorys_describe(&conn, &assemblydes_get) {
        Ok(Some(assembly)) => Ok(render_json(status::Ok, &assembly)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &assemblydes_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}


pub fn plan_factory_create(req: &mut Request) -> AranResult<Response> {
    let mut plan_create = Plan::new();
    {
        match req.get::<bodyparser::Struct<PlanCreateReq>>() {
            Ok(Some(body)) => {
                if body.group_name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "group_name")));
                }
                plan_create.set_group_name(body.group_name);
                plan_create.set_url(body.url);
                plan_create.set_description(body.description);
                plan_create.set_tags(body.tags);
                plan_create.set_origin(body.origin);
                plan_create.set_artifacts(body.artifacts);
                plan_create.set_services(
                    body.services
                        .iter()
                        .map(|x| {
                            Service::new(&x.name, &x.description, &x.href, x.characteristics.clone())
                        })
                        .collect::<Vec<_>>(),
                );
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match DeploymentDS::plan_create(&conn, &plan_create) {
        Ok(Some(plan)) => Ok(render_json(status::Ok, &plan)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

#[allow(unused_variables)]
pub fn plan_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match DeploymentDS::plan_list(&conn) {
        Ok(Some(plan_list)) => Ok(render_json(status::Ok, &plan_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

/// Endpoint for determining availability of builder-api components.
/// Returns a status 200 on success. Any non-200 responses are an outage or a partial outage.
pub fn status(_req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
