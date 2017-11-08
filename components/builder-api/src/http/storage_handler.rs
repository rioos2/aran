// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [origin] for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use storage::storage_ds::StorageDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use router::Router;
use protocol::asmsrv::{IdGet, Condition, Status};
use protocol::storagesrv::{Storage, DataCenter, Disks, Disk, StoragePool};

use db::data_store::Broker;
use db;
use std::collections::BTreeMap;
use http::deployment_handler;
use rio_net::util::errors::AranResult;
use error::{Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StorageCreateReq {
    name: String,
    host_ip: String,
    storage_type: String,
    parameters: BTreeMap<String, String>,
    storage_info: DisksReq,
    status: deployment_handler::StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DisksReq {
    disks: Vec<DiskReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DiskReq {
    disk: String,
    disk_type: String,
    point: String,
    size: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StorageStatusReq {
    status: deployment_handler::StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DataCenterReq {
    name: String,
    nodes: Vec<String>,
    networks: Vec<String>,
    storage: String,
    advanced_settings: BTreeMap<String, String>,
    flag: String,
    enabled: bool,
    currency: String,
    status: deployment_handler::StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoragePoolCreateReq {
    name: String,
    connector_id: String,
    parameters: BTreeMap<String, String>,
    storage_info: DisksReq,
    status: deployment_handler::StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoragePoolStatusReq {
    status: deployment_handler::StatusReq,
}

pub fn storage_create(req: &mut Request) -> AranResult<Response> {
    let mut storage_create = Storage::new();
    {
        match req.get::<bodyparser::Struct<StorageCreateReq>>() {
            Ok(Some(body)) => {
                storage_create.set_name(body.name);
                storage_create.set_host_ip(body.host_ip);
                storage_create.set_storage_type(body.storage_type);
                storage_create.set_paramaters(body.parameters);
                storage_create.set_status(Status::with_conditions(
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

                storage_create.set_storage_info(Disks::new(
                    body.storage_info
                        .disks
                        .iter()
                        .map(|x| Disk::new(&x.disk, &x.disk_type, &x.point, &x.size))
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

    match StorageDS::storage_create(&conn, &storage_create) {
        Ok(storage) => Ok(render_json(status::Ok, &storage)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}

#[allow(unused_variables)]
pub fn storage_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match StorageDS::storage_list(&conn) {
        Ok(Some(storage_list)) => Ok(render_json(status::Ok, &storage_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}


pub fn storage_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut storage_get = IdGet::new();
    storage_get.set_id(id.to_string());

    match StorageDS::storage_show(&conn, &storage_get) {
        Ok(Some(storage)) => Ok(render_json(status::Ok, &storage)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &storage_get.get_id()
            )))
        }
    }
}

pub fn storage_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    let mut storage_create = Storage::new();
    storage_create.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<StorageCreateReq>>() {
            Ok(Some(body)) => {
                storage_create.set_name(body.name);
                storage_create.set_host_ip(body.host_ip);
                storage_create.set_storage_type(body.storage_type);
                storage_create.set_paramaters(body.parameters);
                storage_create.set_storage_info(Disks::new(
                    body.storage_info
                        .disks
                        .iter()
                        .map(|x| Disk::new(&x.disk, &x.disk_type, &x.point, &x.size))
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

    match StorageDS::storage_update(&conn, &storage_create) {
        Ok(Some(storage_create)) => Ok(render_json(status::Ok, &storage_create)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &storage_create.get_id()
            )))
        }
    }
}

pub fn storage_status_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    let mut storage_create = Storage::new();
    storage_create.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<StorageStatusReq>>() {
            Ok(Some(body)) => {
                storage_create.set_status(Status::with_conditions(
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

    match StorageDS::storage_status_update(&conn, &storage_create) {
        Ok(Some(storage_create)) => Ok(render_json(status::Ok, &storage_create)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &storage_create.get_id()
            )))
        }
    }
}

pub fn data_center_create(req: &mut Request) -> AranResult<Response> {
    let mut dc_create = DataCenter::new();
    {
        match req.get::<bodyparser::Struct<DataCenterReq>>() {
            Ok(Some(body)) => {
                dc_create.set_name(body.name);
                dc_create.set_networks(body.networks);
                dc_create.set_flag(body.flag);
                dc_create.set_currency(body.currency);
                dc_create.set_storage(body.storage);
                dc_create.set_advanced_settings(body.advanced_settings);
                dc_create.set_nodes(body.nodes);
                dc_create.set_status(Status::with_conditions(
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
                dc_create.set_enabled(body.enabled);
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

    match StorageDS::data_center_create(&conn, &dc_create) {
        Ok(dc_create) => Ok(render_json(status::Ok, &dc_create)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),

    }
}


#[allow(unused_variables)]
pub fn data_center_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match StorageDS::data_center_list(&conn) {
        Ok(Some(data_center_list)) => Ok(render_json(status::Ok, &data_center_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}


pub fn data_center_show(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut dc_get = IdGet::new();
    dc_get.set_id(id.to_string());

    match StorageDS::data_center_show(&conn, &dc_get) {
        Ok(Some(dc)) => Ok(render_json(status::Ok, &dc)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &dc_get.get_id()
            )))
        }
    }
}


pub fn storage_pool_create(req: &mut Request) -> AranResult<Response> {
    let mut storage_create = StoragePool::new();
    {
        match req.get::<bodyparser::Struct<StoragePoolCreateReq>>() {
            Ok(Some(body)) => {
                if body.connector_id.len() <= 0 {
                    return Err(bad_request(
                        &format!("{} {}", MISSING_FIELD, "connector_id"),
                    ));
                }
                storage_create.set_name(body.name);
                storage_create.set_connector_id(body.connector_id);
                storage_create.set_paramaters(body.parameters);

                storage_create.set_status(Status::with_conditions(
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

                storage_create.set_storage_info(Disks::new(
                    body.storage_info
                        .disks
                        .iter()
                        .map(|x| Disk::new(&x.disk, &x.disk_type, &x.point, &x.size))
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

    match StorageDS::storage_pool_create(&conn, &storage_create) {
        Ok(Some(storage)) => Ok(render_json(status::Ok, &storage)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn storage_pool_status_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    let mut storage_pool_update = StoragePool::new();
    storage_pool_update.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<StoragePoolStatusReq>>() {
            Ok(Some(body)) => {
                storage_pool_update.set_status(Status::with_conditions(
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

    match StorageDS::storage_pool_status_update(&conn, &storage_pool_update) {
        Ok(Some(storage_pool_update)) => Ok(render_json(status::Ok, &storage_pool_update)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &storage_pool_update.get_id()
            )))
        }

    }
}


#[allow(unused_variables)]
pub fn storage_pool_list(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut storage_get = IdGet::new();
    storage_get.set_id(id.to_string());

    match StorageDS::storage_pool_list(&conn, &storage_get) {
        Ok(Some(storage)) => Ok(render_json(status::Ok, &storage)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &storage_get.get_id()
            )))
        }
    }
}

#[allow(unused_variables)]
pub fn storage_pool_list_all(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match StorageDS::storage_pool_list_all(&conn) {
        Ok(Some(storage_pool_list)) => Ok(render_json(status::Ok, &storage_pool_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}
