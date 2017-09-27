// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [origin] for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use storage::storage_ds::StorageDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::net::{self, ErrCode};
use router::Router;
use protocol::servicesrv::ObjectMetaData;
use protocol::asmsrv::{TypeMeta, IdGet, Condition};
use protocol::storagesrv::{Storage, Status, DataCenter, DcStatus};

use db::data_store::Broker;
use std::collections::BTreeMap;

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StorageCreateReq {
    type_meta: TypeMetaReq,
    object_meta: ObjectMetaReq,
    name: String,
    host_ip: String,
    storage_type: String,
    parameters: BTreeMap<String, String>,
    status: StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TypeMetaReq {
    kind: String,
    api_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjectMetaReq {
    name: String,
    origin: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusReq {
    health_status: String,
    message: String,
    reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StorageStatusReq {
    status: StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DataCenterReq {
    type_meta: TypeMetaReq,
    object_meta: ObjectMetaReq,
    name: String,
    nodes: Vec<String>,
    networks: Vec<String>,
    storage: String,
    advanced_settings: BTreeMap<String, String>,
    flag: String,
    currency: String,
    status: DcStatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DcStatusReq {
    health_status: String,
    message: String,
    reason: String,
    conditions: Vec<ConditionReq>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConditionReq {
    message: String,
    reason: String,
    status: String,
    last_transition_time: String,
    last_probe_time: String,
    condition_type: String,
}

pub fn storage_create(req: &mut Request) -> IronResult<Response> {
    let mut storage_create = Storage::new();
    {
        match req.get::<bodyparser::Struct<StorageCreateReq>>() {
            Ok(Some(body)) => {
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                storage_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                storage_create.set_type_meta(type_meta);
                storage_create.set_name(body.name);
                storage_create.set_host_ip(body.host_ip);
                storage_create.set_storage_type(body.storage_type);
                storage_create.set_paramaters(body.parameters);
                let mut status = Status::new();
                status.set_health_status(body.status.health_status);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);
                storage_create.set_status(status);
            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match StorageDS::storage_create(&conn, &storage_create) {
        Ok(storage) => Ok(render_json(status::Ok, &storage)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

#[allow(unused_variables)]
pub fn storage_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match StorageDS::storage_list(&conn) {
        Ok(storage_list) => Ok(render_json(status::Ok, &storage_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}


pub fn storage_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut storage_get = IdGet::new();
    storage_get.set_id(id.to_string());

    match StorageDS::storage_show(&conn, &storage_get) {
        Ok(storage) => Ok(render_json(status::Ok, &storage)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn storage_update(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };
    let mut storage_create = Storage::new();
    storage_create.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<StorageCreateReq>>() {
            Ok(Some(body)) => {
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                storage_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                storage_create.set_type_meta(type_meta);
                storage_create.set_name(body.name);
                storage_create.set_host_ip(body.host_ip);
                storage_create.set_storage_type(body.storage_type);
                storage_create.set_paramaters(body.parameters);
            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match StorageDS::storage_update(&conn, &storage_create) {
        Ok(storage_create) => Ok(render_json(status::Ok, &storage_create)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn storage_status_update(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };
    let mut storage_create = Storage::new();
    storage_create.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<StorageStatusReq>>() {
            Ok(Some(body)) => {
                let mut status = Status::new();
                status.set_health_status(body.status.health_status);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);
                storage_create.set_status(status);
            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match StorageDS::storage_status_update(&conn, &storage_create) {
        Ok(storage_create) => Ok(render_json(status::Ok, &storage_create)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn data_center_create(req: &mut Request) -> IronResult<Response> {
    let mut dc_create = DataCenter::new();
    {
        match req.get::<bodyparser::Struct<DataCenterReq>>() {
            Ok(Some(body)) => {
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                dc_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                dc_create.set_type_meta(type_meta);
                dc_create.set_name(body.name);
                dc_create.set_networks(body.networks);
                dc_create.set_storage(body.storage);
                dc_create.set_advanced_settings(body.advanced_settings);
                dc_create.set_nodes(body.nodes);
                let mut status = DcStatus::new();
                status.set_health_status(body.status.health_status);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);
                let mut condition_collection = Vec::new();
                for data in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_message(data.message);
                    condition.set_reason(data.reason);
                    condition.set_status(data.status);
                    condition.set_last_transition_time(data.last_transition_time);
                    condition.set_last_probe_time(data.last_probe_time);
                    condition.set_condition_type(data.condition_type);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);
                dc_create.set_status(status);
            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match StorageDS::data_center_create(&conn, &dc_create) {
        Ok(dc_create) => Ok(render_json(status::Ok, &dc_create)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
