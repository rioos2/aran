use bodyparser;
use rio_net::http::controller::*;
use node::node_ds::NodeDS;
use iron::prelude::*;
use iron::status;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;
use protocol::nodesrv::{Node, Spec, Status, Conditions, Taints, Addresses, NodeInfo};
use protocol::asmsrv::{TypeMeta, ObjectMeta, OwnerReferences};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeCreateReq {
    spec: SpecReq,
    status: StatusReq,
    object_meta: ObjectMetaReq,
    type_meta: TypeMetaReq,
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
    conditions: Vec<ConditionsReq>,
    addresses: Vec<AddressesReq>,
    node_info: NodeInfoReq,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConditionsReq {
    condition_type: String,
    status: String,
    last_heartbeat_time: String,
    last_transition_time: String,
    reason: String,
    message: String,
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CommonStatusReq {
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
    owner_references: Vec<OwnerReferencesReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OwnerReferencesReq {
    kind: String,
    api_version: String,
    name: String,
    uid: String,
    block_owner_deletion: bool,
}


pub fn node_create(req: &mut Request) -> IronResult<Response> {
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
                    let mut condition = Conditions::new();
                    condition.set_condition_type(conn.condition_type);
                    condition.set_last_heartbeat_time(conn.last_heartbeat_time);
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
                status.set_node_info(node_info);
                node_create.set_status(status);

                let mut object_meta = ObjectMeta::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                let mut owner_references_collection = Vec::new();
                for data in body.object_meta.owner_references {
                    let mut owner_references = OwnerReferences::new();
                    owner_references.set_kind(data.kind);
                    owner_references.set_api_version(data.api_version);
                    owner_references.set_name(data.name);
                    owner_references.set_uid(data.uid);
                    owner_references.set_block_owner_deletion(data.block_owner_deletion);
                    owner_references_collection.push(owner_references);
                }
                object_meta.set_owner_references(owner_references_collection);
                node_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                node_create.set_type_meta(type_meta);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match NodeDS::node_create(&conn, &node_create) {
        Ok(node) => Ok(render_json(status::Ok, &node)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn node_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match NodeDS::node_list(&conn) {
        Ok(node_list) => Ok(render_json(status::Ok, &node_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}


pub fn node_status_update(req: &mut Request) -> IronResult<Response> {
    let mut node_create = Node::new();
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
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
                    let mut condition = Conditions::new();
                    condition.set_condition_type(conn.condition_type);
                    condition.set_last_heartbeat_time(conn.last_heartbeat_time);
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
                status.set_node_info(node_info);
                node_create.set_status(status);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match NodeDS::node_status_update(&conn, &node_create) {
        Ok(node) => Ok(render_json(status::Ok, &node)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn node_metrics(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match NodeDS::node_metrics(&conn) {
        Ok(node_list) => Ok(render_json(status::Ok, &node_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}
