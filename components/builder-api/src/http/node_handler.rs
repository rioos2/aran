use bodyparser;
use rio_net::http::controller::*;
use node::node_ds::NodeDS;
use iron::prelude::*;
use iron::status;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;
use protocol::nodesrv::{Node, Spec, Status, Capacity, Range, FixedRange, InfiniteRange, Conditions, Taints, Addresses, NodeAddress, NodeInfo};
use protocol::asmsrv::{TypeMeta, ObjectMeta, Labels, Annotations, OwnerReferences};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeCreateReq {
    spec: SpecReq,
    status: StatusReq,
    object_meta: ObjectMetaReq,
    type_meta: TypeMetaReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TypeMetaReq {
    kind: String,
    api_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjectMetaReq {
    name: String,
    namespace: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: LabelsReq,
    annotations: AnnotationsReq,
    owner_references: Vec<OwnerReferencesReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LabelsReq {
    group: String,
    key2: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AnnotationsReq {
    key1: String,
    key2: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OwnerReferencesReq {
    kind: String,
    api_version: String,
    name: String,
    uid: String,
    block_owner_deletion: bool,
}
#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpecReq {
    assemblyCIDR: String,
    externalID: String,
    providerID: String,
    unschedulable: bool,
    taints: Vec<TaintsReq>,
}
#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct TaintsReq {
    key: String,
    value: String,
    effect: String,
    timeAdded: String,
}
#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusReq {
    capacity: CapacityReq,
    allocatable: CapacityReq,
    phase: String,
    conditions: Vec<ConditionsReq>,
    addresses: Vec<AddressesReq>,
    nodeInfo: NodeInfoReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CapacityReq {
    cpu: RangeReq,
    mem: RangeReq,
    disk: RangeReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RangeReq {
    fixed_range: FixedRangeReq,
    infinite_range: InfiniteRangeReq,
    quantity: String,
    format: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct FixedRangeReq {
    value: String,
    scale: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct InfiniteRangeReq {
    unscale: String,
    scale: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ConditionsReq {
    conditionType: String,
    status: String,
    lastHeartbeatTime: String,
    lastTransitionTime: String,
    reason: String,
    message: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct AddressesReq {
    nodeAddress: NodeAddressReq,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeAddressReq {
    nodeType: String,
    addresses: String,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeInfoReq {
    machineID: String,
    systemUUID: String,
    kernelVersion: String,
    oSImage: String,
    architecture: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CommonStatusReq {
    status: StatusReq,
}


pub fn node_create(req: &mut Request) -> IronResult<Response> {
    let mut node_create = Node::new();
    {
        match req.get::<bodyparser::Struct<NodeCreateReq>>() {
            Ok(Some(body)) => {
                let mut spec = Spec::new();
                spec.set_assemblyCIDR(body.spec.assemblyCIDR);
                spec.set_externalID(body.spec.externalID);
                spec.set_providerID(body.spec.providerID);
                spec.set_unschedulable(body.spec.unschedulable);

                let mut taints_collection = Vec::new();

                for data in body.spec.taints {
                    let mut taints = Taints::new();
                    taints.set_value(data.value);
                    taints.set_key(data.key);
                    taints.set_effect(data.effect);
                    taints.set_timeAdded(data.timeAdded);
                    taints_collection.push(taints);
                }
                spec.set_taints(taints_collection);
                node_create.set_spec(spec);

                let mut status = Status::new();

                let mut capacity = Capacity::new();

                let mut cpu_capacity = Range::new();
                let mut cpu_capacity_fixed_range = FixedRange::new();
                cpu_capacity_fixed_range.set_value(body.status.capacity.cpu.fixed_range.value);
                cpu_capacity_fixed_range.set_scale(body.status.capacity.cpu.fixed_range.scale);
                cpu_capacity.set_fixed_range(cpu_capacity_fixed_range);
                let mut cpu_capacity_infinite_range = InfiniteRange::new();
                cpu_capacity_infinite_range.set_unscale(body.status.capacity.cpu.infinite_range.unscale);
                cpu_capacity_infinite_range.set_scale(body.status.capacity.cpu.infinite_range.scale);
                cpu_capacity.set_infinite_range(cpu_capacity_infinite_range);
                cpu_capacity.set_quantity(body.status.capacity.cpu.quantity);
                cpu_capacity.set_format(body.status.capacity.cpu.format);


                let mut mem_capacity = Range::new();
                let mut mem_capacity_fixed_range = FixedRange::new();
                mem_capacity_fixed_range.set_value(body.status.capacity.mem.fixed_range.value);
                mem_capacity_fixed_range.set_scale(body.status.capacity.mem.fixed_range.scale);
                mem_capacity.set_fixed_range(mem_capacity_fixed_range);
                let mut mem_capacity_infinite_range = InfiniteRange::new();
                mem_capacity_infinite_range.set_unscale(body.status.capacity.mem.infinite_range.unscale);
                mem_capacity_infinite_range.set_scale(body.status.capacity.mem.infinite_range.scale);
                mem_capacity.set_infinite_range(mem_capacity_infinite_range);
                mem_capacity.set_quantity(body.status.capacity.mem.quantity);
                mem_capacity.set_format(body.status.capacity.mem.format);


                let mut disk_capacity = Range::new();
                let mut disk_capacity_fixed_range = FixedRange::new();
                disk_capacity_fixed_range.set_value(body.status.capacity.disk.fixed_range.value);
                disk_capacity_fixed_range.set_scale(body.status.capacity.disk.fixed_range.scale);
                disk_capacity.set_fixed_range(disk_capacity_fixed_range);
                let mut disk_capacity_infinite_range = InfiniteRange::new();
                disk_capacity_infinite_range.set_unscale(body.status.capacity.disk.infinite_range.unscale);
                disk_capacity_infinite_range.set_scale(body.status.capacity.disk.infinite_range.scale);
                disk_capacity.set_infinite_range(disk_capacity_infinite_range);
                disk_capacity.set_quantity(body.status.capacity.disk.quantity);
                disk_capacity.set_format(body.status.capacity.disk.format);

                capacity.set_cpu(cpu_capacity);
                capacity.set_mem(mem_capacity);
                capacity.set_disk(disk_capacity);

                status.set_capacity(capacity);

                let mut allocatable = Capacity::new();

                let mut cpu_allocatable = Range::new();
                let mut cpu_allocatable_fixed_range = FixedRange::new();
                cpu_allocatable_fixed_range.set_value(body.status.allocatable.cpu.fixed_range.value);
                cpu_allocatable_fixed_range.set_scale(body.status.allocatable.cpu.fixed_range.scale);
                cpu_allocatable.set_fixed_range(cpu_allocatable_fixed_range);
                let mut cpu_allocatable_infinite_range = InfiniteRange::new();
                cpu_allocatable_infinite_range.set_unscale(body.status.allocatable.cpu.infinite_range.unscale);
                cpu_allocatable_infinite_range.set_scale(body.status.allocatable.cpu.infinite_range.scale);
                cpu_allocatable.set_infinite_range(cpu_allocatable_infinite_range);
                cpu_allocatable.set_quantity(body.status.allocatable.cpu.quantity);
                cpu_allocatable.set_format(body.status.allocatable.cpu.format);


                let mut mem_allocatable = Range::new();
                let mut mem_allocatable_fixed_range = FixedRange::new();
                mem_allocatable_fixed_range.set_value(body.status.allocatable.mem.fixed_range.value);
                mem_allocatable_fixed_range.set_scale(body.status.allocatable.mem.fixed_range.scale);
                mem_allocatable.set_fixed_range(mem_allocatable_fixed_range);
                let mut mem_allocatable_infinite_range = InfiniteRange::new();
                mem_allocatable_infinite_range.set_unscale(body.status.allocatable.mem.infinite_range.unscale);
                mem_allocatable_infinite_range.set_scale(body.status.allocatable.mem.infinite_range.scale);
                mem_allocatable.set_infinite_range(mem_allocatable_infinite_range);
                mem_allocatable.set_quantity(body.status.allocatable.mem.quantity);
                mem_allocatable.set_format(body.status.allocatable.mem.format);


                let mut disk_allocatable = Range::new();
                let mut disk_allocatable_fixed_range = FixedRange::new();
                disk_allocatable_fixed_range.set_value(body.status.allocatable.disk.fixed_range.value);
                disk_allocatable_fixed_range.set_scale(body.status.allocatable.disk.fixed_range.scale);
                disk_allocatable.set_fixed_range(disk_allocatable_fixed_range);
                let mut disk_allocatable_infinite_range = InfiniteRange::new();
                disk_allocatable_infinite_range.set_unscale(body.status.allocatable.disk.infinite_range.unscale);
                disk_allocatable_infinite_range.set_scale(body.status.allocatable.disk.infinite_range.scale);
                disk_allocatable.set_infinite_range(disk_allocatable_infinite_range);
                disk_allocatable.set_quantity(body.status.allocatable.disk.quantity);
                disk_allocatable.set_format(body.status.allocatable.disk.format);

                allocatable.set_cpu(cpu_allocatable);
                allocatable.set_mem(mem_allocatable);
                allocatable.set_disk(disk_allocatable);

                status.set_allocatable(allocatable);
                status.set_phase(body.status.phase);

                let mut condition_collection = Vec::new();

                for conn in body.status.conditions {
                    let mut condition = Conditions::new();
                    condition.set_conditionType(conn.conditionType);
                    condition.set_lastHeartbeatTime(conn.lastHeartbeatTime);
                    condition.set_lastTransitionTime(conn.lastTransitionTime);
                    condition.set_reason(conn.reason);
                    condition.set_status(conn.status);
                    condition.set_message(conn.message);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);

                let mut addresse_collection = Vec::new();

                for addr in body.status.addresses {
                    let mut addresses = Addresses::new();
                    let mut node_addr = NodeAddress::new();
                    node_addr.set_nodeType(addr.nodeAddress.nodeType);
                    node_addr.set_addresses(addr.nodeAddress.addresses);
                    addresses.set_nodeAddress(node_addr);
                    addresse_collection.push(addresses);
                }
                status.set_addresses(addresse_collection);

                let mut node_info = NodeInfo::new();
                node_info.set_machineID(body.status.nodeInfo.machineID);
                node_info.set_systemUUID(body.status.nodeInfo.systemUUID);
                node_info.set_kernelVersion(body.status.nodeInfo.kernelVersion);
                node_info.set_oSImage(body.status.nodeInfo.oSImage);
                node_info.set_architecture(body.status.nodeInfo.architecture);
                status.set_nodeInfo(node_info);
                node_create.set_status(status);

                let mut object_meta = ObjectMeta::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_namespace(body.object_meta.namespace);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                let mut labels = Labels::new();
                labels.set_group(body.object_meta.labels.group);
                labels.set_key2(body.object_meta.labels.key2);
                object_meta.set_labels(labels);
                let mut annotations = Annotations::new();
                annotations.set_key1(body.object_meta.annotations.key1);
                annotations.set_key2(body.object_meta.annotations.key2);
                object_meta.set_annotations(annotations);
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

                let mut capacity = Capacity::new();

                let mut cpu_capacity = Range::new();
                let mut cpu_capacity_fixed_range = FixedRange::new();
                cpu_capacity_fixed_range.set_value(body.status.capacity.cpu.fixed_range.value);
                cpu_capacity_fixed_range.set_scale(body.status.capacity.cpu.fixed_range.scale);
                cpu_capacity.set_fixed_range(cpu_capacity_fixed_range);
                let mut cpu_capacity_infinite_range = InfiniteRange::new();
                cpu_capacity_infinite_range.set_unscale(body.status.capacity.cpu.infinite_range.unscale);
                cpu_capacity_infinite_range.set_scale(body.status.capacity.cpu.infinite_range.scale);
                cpu_capacity.set_infinite_range(cpu_capacity_infinite_range);
                cpu_capacity.set_quantity(body.status.capacity.cpu.quantity);
                cpu_capacity.set_format(body.status.capacity.cpu.format);


                let mut mem_capacity = Range::new();
                let mut mem_capacity_fixed_range = FixedRange::new();
                mem_capacity_fixed_range.set_value(body.status.capacity.mem.fixed_range.value);
                mem_capacity_fixed_range.set_scale(body.status.capacity.mem.fixed_range.scale);
                mem_capacity.set_fixed_range(mem_capacity_fixed_range);
                let mut mem_capacity_infinite_range = InfiniteRange::new();
                mem_capacity_infinite_range.set_unscale(body.status.capacity.mem.infinite_range.unscale);
                mem_capacity_infinite_range.set_scale(body.status.capacity.mem.infinite_range.scale);
                mem_capacity.set_infinite_range(mem_capacity_infinite_range);
                mem_capacity.set_quantity(body.status.capacity.mem.quantity);
                mem_capacity.set_format(body.status.capacity.mem.format);


                let mut disk_capacity = Range::new();
                let mut disk_capacity_fixed_range = FixedRange::new();
                disk_capacity_fixed_range.set_value(body.status.capacity.disk.fixed_range.value);
                disk_capacity_fixed_range.set_scale(body.status.capacity.disk.fixed_range.scale);
                disk_capacity.set_fixed_range(disk_capacity_fixed_range);
                let mut disk_capacity_infinite_range = InfiniteRange::new();
                disk_capacity_infinite_range.set_unscale(body.status.capacity.disk.infinite_range.unscale);
                disk_capacity_infinite_range.set_scale(body.status.capacity.disk.infinite_range.scale);
                disk_capacity.set_infinite_range(disk_capacity_infinite_range);
                disk_capacity.set_quantity(body.status.capacity.disk.quantity);
                disk_capacity.set_format(body.status.capacity.disk.format);

                capacity.set_cpu(cpu_capacity);
                capacity.set_mem(mem_capacity);
                capacity.set_disk(disk_capacity);

                status.set_capacity(capacity);

                let mut allocatable = Capacity::new();

                let mut cpu_allocatable = Range::new();
                let mut cpu_allocatable_fixed_range = FixedRange::new();
                cpu_allocatable_fixed_range.set_value(body.status.allocatable.cpu.fixed_range.value);
                cpu_allocatable_fixed_range.set_scale(body.status.allocatable.cpu.fixed_range.scale);
                cpu_allocatable.set_fixed_range(cpu_allocatable_fixed_range);
                let mut cpu_allocatable_infinite_range = InfiniteRange::new();
                cpu_allocatable_infinite_range.set_unscale(body.status.allocatable.cpu.infinite_range.unscale);
                cpu_allocatable_infinite_range.set_scale(body.status.allocatable.cpu.infinite_range.scale);
                cpu_allocatable.set_infinite_range(cpu_allocatable_infinite_range);
                cpu_allocatable.set_quantity(body.status.allocatable.cpu.quantity);
                cpu_allocatable.set_format(body.status.allocatable.cpu.format);


                let mut mem_allocatable = Range::new();
                let mut mem_allocatable_fixed_range = FixedRange::new();
                mem_allocatable_fixed_range.set_value(body.status.allocatable.mem.fixed_range.value);
                mem_allocatable_fixed_range.set_scale(body.status.allocatable.mem.fixed_range.scale);
                mem_allocatable.set_fixed_range(mem_allocatable_fixed_range);
                let mut mem_allocatable_infinite_range = InfiniteRange::new();
                mem_allocatable_infinite_range.set_unscale(body.status.allocatable.mem.infinite_range.unscale);
                mem_allocatable_infinite_range.set_scale(body.status.allocatable.mem.infinite_range.scale);
                mem_allocatable.set_infinite_range(mem_allocatable_infinite_range);
                mem_allocatable.set_quantity(body.status.allocatable.mem.quantity);
                mem_allocatable.set_format(body.status.allocatable.mem.format);


                let mut disk_allocatable = Range::new();
                let mut disk_allocatable_fixed_range = FixedRange::new();
                disk_allocatable_fixed_range.set_value(body.status.allocatable.disk.fixed_range.value);
                disk_allocatable_fixed_range.set_scale(body.status.allocatable.disk.fixed_range.scale);
                disk_allocatable.set_fixed_range(disk_allocatable_fixed_range);
                let mut disk_allocatable_infinite_range = InfiniteRange::new();
                disk_allocatable_infinite_range.set_unscale(body.status.allocatable.disk.infinite_range.unscale);
                disk_allocatable_infinite_range.set_scale(body.status.allocatable.disk.infinite_range.scale);
                disk_allocatable.set_infinite_range(disk_allocatable_infinite_range);
                disk_allocatable.set_quantity(body.status.allocatable.disk.quantity);
                disk_allocatable.set_format(body.status.allocatable.disk.format);

                allocatable.set_cpu(cpu_allocatable);
                allocatable.set_mem(mem_allocatable);
                allocatable.set_disk(disk_allocatable);

                status.set_allocatable(allocatable);
                status.set_phase(body.status.phase);

                let mut condition_collection = Vec::new();

                for conn in body.status.conditions {
                    let mut condition = Conditions::new();
                    condition.set_conditionType(conn.conditionType);
                    condition.set_lastHeartbeatTime(conn.lastHeartbeatTime);
                    condition.set_lastTransitionTime(conn.lastTransitionTime);
                    condition.set_reason(conn.reason);
                    condition.set_status(conn.status);
                    condition.set_message(conn.message);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);

                let mut addresse_collection = Vec::new();

                for addr in body.status.addresses {
                    let mut addresses = Addresses::new();
                    let mut node_addr = NodeAddress::new();
                    node_addr.set_nodeType(addr.nodeAddress.nodeType);
                    node_addr.set_addresses(addr.nodeAddress.addresses);
                    addresses.set_nodeAddress(node_addr);
                    addresse_collection.push(addresses);
                }
                status.set_addresses(addresse_collection);

                let mut node_info = NodeInfo::new();
                node_info.set_machineID(body.status.nodeInfo.machineID);
                node_info.set_systemUUID(body.status.nodeInfo.systemUUID);
                node_info.set_kernelVersion(body.status.nodeInfo.kernelVersion);
                node_info.set_oSImage(body.status.nodeInfo.oSImage);
                node_info.set_architecture(body.status.nodeInfo.architecture);
                status.set_nodeInfo(node_info);
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
