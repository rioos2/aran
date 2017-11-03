use std::collections::BTreeMap;
use bodyparser;
use iron::prelude::*;
use iron::status;
use persistent;
use router::Router;
use ansi_term::Colour;

use protocol::net::{self, ErrCode};
use rio_net::http::controller::*;
use rio_net::http::middleware::PrometheusCli;
use node::node_ds::NodeDS;
use db::data_store::Broker;
use db;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body,not_found_error};
use error::{Result, Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};

use protocol::nodesrv::{Node, Spec, Status, Taints, Addresses, NodeInfo, Bridge};
use protocol::asmsrv::Condition;
use http::deployment_handler;
use common::ui;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeCreateReq {
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
    bridge_type: String,
}





pub fn node_create(req: &mut Request) -> AranResult<Response> {
    let mut node_create = Node::new();
    {
        match req.get::<bodyparser::Struct<NodeCreateReq>>() {
            Ok(Some(body)) => {
                let mut spec = Spec::new();
                spec.set_assembly_cidr(body.spec.assembly_cidr);
                spec.set_external_id(body.spec.external_id);
                spec.set_provider_id(body.spec.provider_id);
                spec.set_unschedulable(body.spec.unschedulable);

                let mut taints_collection = Vec::new();

                for data in body.spec.taints {
                    let mut taints = Taints::new();
                    taints.set_value(data.value);
                    taints.set_key(data.key);
                    taints.set_effect(data.effect);
                    taints.set_time_added(data.time_added);
                    taints_collection.push(taints);
                }
                spec.set_taints(taints_collection);
                node_create.set_spec(spec);

                let mut status = Status::new();

                status.set_capacity(body.status.capacity);
                status.set_allocatable(body.status.allocatable);
                status.set_phase(body.status.phase);

                let mut condition_collection = Vec::new();

                for conn in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_condition_type(conn.condition_type);
                    condition.set_last_probe_time(conn.last_probe_time);
                    condition.set_last_transition_time(conn.last_transition_time);
                    condition.set_reason(conn.reason);
                    condition.set_status(conn.status);
                    condition.set_message(conn.message);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);

                let mut addresse_collection = Vec::new();

                for addr in body.status.addresses {
                    let mut addresses = Addresses::new();
                    addresses.set_node_type(addr.node_type);
                    addresses.set_address(addr.address);
                    addresse_collection.push(addresses);
                }
                status.set_addresses(addresse_collection);

                let mut node_info = NodeInfo::new();
                node_info.set_machine_id(body.status.node_info.machine_id);
                node_info.set_system_uuid(body.status.node_info.system_uuid);
                node_info.set_kernel_version(body.status.node_info.kernel_version);
                node_info.set_os_image(body.status.node_info.os_image);
                node_info.set_architecture(body.status.node_info.architecture);
                let mut bridge_collection = Vec::new();
                for bridge in body.status.node_info.bridges {
                    let mut bri = Bridge::new();
                    bri.set_bridge_name(bridge.bridge_name);
                    bri.set_physical_device(bridge.physical_device);
                    bri.set_bridge_type(bridge.bridge_type);
                    bridge_collection.push(bri);
                }
                node_info.set_bridges(bridge_collection);
                status.set_node_info(node_info);
                node_create.set_status(status);
            }
            Err(err) => {
                return Err(malformed_body(&format!("{}, {:?}\n", err.detail, err.cause),));
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
        Err(err) => {
            Err(internal_error(&format!("{}\n", err)))
        }
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
        Err(err) => {
            Err(internal_error(&format!("{}\n", err)))
        }
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
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
                let mut status = Status::new();

                status.set_capacity(body.status.capacity);
                status.set_allocatable(body.status.allocatable);
                status.set_phase(body.status.phase);

                let mut condition_collection = Vec::new();

                for conn in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_condition_type(conn.condition_type);
                    condition.set_last_probe_time(conn.last_probe_time);
                    condition.set_last_transition_time(conn.last_transition_time);
                    condition.set_reason(conn.reason);
                    condition.set_status(conn.status);
                    condition.set_message(conn.message);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);

                let mut addresse_collection = Vec::new();

                for addr in body.status.addresses {
                    let mut addresses = Addresses::new();
                    addresses.set_node_type(addr.node_type);
                    addresses.set_address(addr.address);
                    addresse_collection.push(addresses);
                }
                status.set_addresses(addresse_collection);

                let mut node_info = NodeInfo::new();
                node_info.set_machine_id(body.status.node_info.machine_id);
                node_info.set_system_uuid(body.status.node_info.system_uuid);
                node_info.set_kernel_version(body.status.node_info.kernel_version);
                node_info.set_os_image(body.status.node_info.os_image);
                node_info.set_architecture(body.status.node_info.architecture);
                let mut bridge_collection = Vec::new();
                for bridge in body.status.node_info.bridges {
                    let mut bri = Bridge::new();
                    bri.set_bridge_name(bridge.bridge_name);
                    bri.set_physical_device(bridge.physical_device);
                    bri.set_bridge_type(bridge.bridge_type);
                    bridge_collection.push(bri);
                }
                node_info.set_bridges(bridge_collection);

                status.set_node_info(node_info);
                node_create.set_status(status);
            }
            Err(err) => {
                return Err(malformed_body(&format!("{}, {:?}\n", err.detail, err.cause),));
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
        Err(err) => {
            Err(internal_error(&format!("{}\n", err)))
        }
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
