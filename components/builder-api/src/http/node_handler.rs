use std::collections::BTreeMap;
use bodyparser;
use iron::prelude::*;
use iron::status;
use persistent;
use router::Router;
use ansi_term::Colour;

use rio_net::http::controller::*;
use rio_net::http::middleware::PrometheusCli;
use node::node_ds::NodeDS;
use db::data_store::Broker;
use db;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};
use error::{Error, BODYNOTFOUND, IDMUSTNUMBER};

use protocol::nodesrv::{Node, Spec, Status, Taints, Addresses, NodeInfo, Bridge};
use protocol::asmsrv::{Condition, IdGet};
use http::deployment_handler;
use common::ui;
use extract_query_value;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeCreateReq {
    node_ip: String,
    spec: SpecReq,
    status: StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpecReq {
    assembly_cidr: String,
    external_id: String,
    provider_id: String,
    unschedulable: bool,
    taints: Vec<TaintsReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TaintsReq {
    key: String,
    value: String,
    effect: String,
    time_added: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusReq {
    capacity: BTreeMap<String, String>,
    allocatable: BTreeMap<String, String>,
    phase: String,
    conditions: Vec<deployment_handler::ConditionReq>,
    addresses: Vec<AddressesReq>,
    node_info: NodeInfoReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AddressesReq {
    node_type: String,
    address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeInfoReq {
    machine_id: String,
    system_uuid: String,
    kernel_version: String,
    os_image: String,
    architecture: String,
    bridges: Vec<BridgeReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CommonStatusReq {
    status: StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BridgeReq {
    bridge_name: String,
    physical_device: String,
    network_type: String,
    bridge_type: String,
}

pub fn node_create(req: &mut Request) -> AranResult<Response> {
    let mut node_create = Node::new();
    {
        match req.get::<bodyparser::Struct<NodeCreateReq>>() {
            Ok(Some(body)) => {
                node_create.set_node_ip(body.node_ip);
                node_create.set_spec(Spec::new(
                    &body.spec.assembly_cidr,
                    &body.spec.external_id,
                    &body.spec.provider_id,
                    body.spec.unschedulable,
                    body.spec
                        .taints
                        .iter()
                        .map(|x| Taints::new(&x.key, &x.value, &x.effect, &x.time_added))
                        .collect::<Vec<_>>(),
                ));
                node_create.set_status(Status::new(
                    body.status.capacity,
                    body.status.allocatable,
                    &body.status.phase,
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
                    body.status
                        .addresses
                        .iter()
                        .map(|x| Addresses::new(&x.node_type, &x.address))
                        .collect::<Vec<_>>(),
                    NodeInfo::new(
                        &body.status.node_info.machine_id,
                        &body.status.node_info.system_uuid,
                        &body.status.node_info.kernel_version,
                        &body.status.node_info.os_image,
                        &body.status.node_info.architecture,
                        body.status
                            .node_info
                            .bridges
                            .iter()
                            .map(|x| {
                                Bridge::new(
                                    &x.bridge_name,
                                    &x.physical_device,
                                    &x.network_type,
                                    &x.bridge_type,
                                )
                            })
                            .collect::<Vec<_>>(),
                    ),
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

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", node_create),
    );

    let conn = Broker::connect().unwrap();

    match NodeDS::node_create(&conn, &node_create) {
        Ok(Some(node)) => Ok(render_json(status::Ok, &node)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }

    }
}


pub fn node_get_by_node_ip(req: &mut Request) -> AranResult<Response> {
    let node_ip = {
        match extract_query_value("node_ip", req) {
            Some(ip) => ip,
            None => {
                return Err(not_found_error(
                    &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
                ))
            }
        }
    };
    let conn = Broker::connect().unwrap();

    let mut node_get = IdGet::new();
    node_get.set_id(node_ip.to_string());
    match NodeDS::node_get_by_node_ip(&conn, &node_get) {
        Ok(Some(node_get)) => Ok(render_json(status::Ok, &node_get)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

#[allow(unused_variables)]
pub fn node_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match NodeDS::node_list(&conn) {
        Ok(Some(node_list)) => Ok(render_json(status::Ok, &node_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

pub fn node_get(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut node_get = IdGet::new();
    node_get.set_id(id.to_string());

    match NodeDS::node_get(&conn, &node_get) {
        Ok(Some(node)) => Ok(render_json(status::Ok, &node)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &node_get.get_id()
            )))
        }
    }
}

pub fn node_status_update(req: &mut Request) -> AranResult<Response> {
    let mut node_create = Node::new();
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    node_create.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<CommonStatusReq>>() {
            Ok(Some(body)) => {
                node_create.set_status(Status::new(
                    body.status.capacity,
                    body.status.allocatable,
                    &body.status.phase,
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
                    body.status
                        .addresses
                        .iter()
                        .map(|x| Addresses::new(&x.node_type, &x.address))
                        .collect::<Vec<_>>(),
                    NodeInfo::new(
                        &body.status.node_info.machine_id,
                        &body.status.node_info.system_uuid,
                        &body.status.node_info.kernel_version,
                        &body.status.node_info.os_image,
                        &body.status.node_info.architecture,
                        body.status
                            .node_info
                            .bridges
                            .iter()
                            .map(|x| {
                                Bridge::new(
                                    &x.bridge_name,
                                    &x.physical_device,
                                    &x.network_type,
                                    &x.bridge_type,
                                )
                            })
                            .collect::<Vec<_>>(),
                    ),
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

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", node_create),
    );

    let conn = Broker::connect().unwrap();

    match NodeDS::node_status_update(&conn, &node_create) {
        Ok(Some(node)) => Ok(render_json(status::Ok, &node)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &node_create.get_id()
            )))
        }

    }
}



pub fn healthz_all(req: &mut Request) -> AranResult<Response> {
    let promcli = req.get::<persistent::Read<PrometheusCli>>().unwrap();
    match NodeDS::healthz_all(&promcli) {
        Ok(Some(health_all)) => Ok(render_json(status::Ok, &health_all)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}
