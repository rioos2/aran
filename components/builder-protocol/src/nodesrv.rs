#![allow(unknown_lints)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    id: String,
    spec: Spec,
    status: Status,
    created_at: String,
    object_meta: ObjectMeta,
    type_meta: TypeMeta,
}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &Spec {
        &self.spec
    }
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }
    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
    pub fn set_type_meta(&mut self, v: TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMeta {
        &self.object_meta
    }

}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Spec {
    assemblyCIDR: String,
    externalID: String,
    providerID: String,
    unschedulable: bool,
    taints: Vec<Taints>,
}

impl Spec {
    pub fn new() -> Spec {
        ::std::default::Default::default()
    }
    pub fn set_assemblyCIDR(&mut self, v: ::std::string::String) {
        self.assemblyCIDR = v;
    }
    pub fn set_externalID(&mut self, v: ::std::string::String) {
        self.externalID = v;
    }
    pub fn set_providerID(&mut self, v: ::std::string::String) {
        self.providerID = v;
    }

    pub fn set_unschedulable(&mut self, v: bool) {
        self.unschedulable = v;
    }

    pub fn set_taints(&mut self, v: Vec<Taints>) {
        self.taints = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Taints {
    key: String,
    value: String,
    effect: String,
    timeAdded: String,
}

impl Taints {
    pub fn new() -> Taints {
        ::std::default::Default::default()
    }
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }
    pub fn set_effect(&mut self, v: ::std::string::String) {
        self.effect = v;
    }
    pub fn set_timeAdded(&mut self, v: ::std::string::String) {
        self.timeAdded = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    capacity: Capacity,
    allocatable: Capacity,
    phase: String,
    conditions: Vec<Conditions>,
    addresses: Vec<Addresses>,
    nodeInfo: NodeInfo,
}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    pub fn set_capacity(&mut self, v: Capacity) {
        self.capacity = v;
    }
    pub fn set_allocatable(&mut self, v: Capacity) {
        self.allocatable = v;
    }
    pub fn set_nodeInfo(&mut self, v: NodeInfo) {
        self.nodeInfo = v;
    }
    pub fn set_phase(&mut self, v: ::std::string::String) {
        self.phase = v;
    }
    pub fn set_conditions(&mut self, v: Vec<Conditions>) {
        self.conditions = v;
    }
    pub fn set_addresses(&mut self, v: Vec<Addresses>) {
        self.addresses = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Capacity {
    cpu: Range,
    mem: Range,
    disk: Range,
}

impl Capacity {
    pub fn new() -> Capacity {
        ::std::default::Default::default()
    }

    pub fn set_cpu(&mut self, v: Range) {
        self.cpu = v;
    }
    pub fn set_mem(&mut self, v: Range) {
        self.mem = v;
    }
    pub fn set_disk(&mut self, v: Range) {
        self.disk = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Range {
    fixed_range: FixedRange,
    infinite_range: InfiniteRange,
    quantity: String,
    format: String,
}

impl Range {
    pub fn new() -> Range {
        ::std::default::Default::default()
    }

    pub fn set_fixed_range(&mut self, v: FixedRange) {
        self.fixed_range = v;
    }
    pub fn set_infinite_range(&mut self, v: InfiniteRange) {
        self.infinite_range = v;
    }
    pub fn set_quantity(&mut self, v: ::std::string::String) {
        self.quantity = v;
    }
    pub fn set_format(&mut self, v: ::std::string::String) {
        self.format = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct FixedRange {
    value: String,
    scale: String,
}

impl FixedRange {
    pub fn new() -> FixedRange {
        ::std::default::Default::default()
    }
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }
    pub fn set_scale(&mut self, v: ::std::string::String) {
        self.scale = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct InfiniteRange {
    unscale: String,
    scale: String,
}

impl InfiniteRange {
    pub fn new() -> InfiniteRange {
        ::std::default::Default::default()
    }
    pub fn set_unscale(&mut self, v: ::std::string::String) {
        self.unscale = v;
    }
    pub fn set_scale(&mut self, v: ::std::string::String) {
        self.scale = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Conditions {
    conditionType: String,
    status: String,
    lastHeartbeatTime: String,
    lastTransitionTime: String,
    reason: String,
    message: String,
}

impl Conditions {
    pub fn new() -> Conditions {
        ::std::default::Default::default()
    }
    pub fn set_conditionType(&mut self, v: ::std::string::String) {
        self.conditionType = v;
    }
    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }
    pub fn set_lastHeartbeatTime(&mut self, v: ::std::string::String) {
        self.lastHeartbeatTime = v;
    }
    pub fn set_lastTransitionTime(&mut self, v: ::std::string::String) {
        self.lastTransitionTime = v;
    }
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Addresses {
    nodeAddress: NodeAddress,
}

impl Addresses {
    pub fn new() -> Addresses {
        ::std::default::Default::default()
    }

    pub fn set_nodeAddress(&mut self, v: NodeAddress) {
        self.nodeAddress = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeAddress {
    nodeType: String,
    addresses: String,
}

impl NodeAddress {
    pub fn new() -> NodeAddress {
        ::std::default::Default::default()
    }

    pub fn set_nodeType(&mut self, v: ::std::string::String) {
        self.nodeType = v;
    }
    pub fn set_addresses(&mut self, v: ::std::string::String) {
        self.addresses = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeInfo {
    machineID: String,
    systemUUID: String,
    kernelVersion: String,
    oSImage: String,
    architecture: String,
}

impl NodeInfo {
    pub fn new() -> NodeInfo {
        ::std::default::Default::default()
    }
    pub fn set_machineID(&mut self, v: ::std::string::String) {
        self.machineID = v;
    }
    pub fn set_systemUUID(&mut self, v: ::std::string::String) {
        self.systemUUID = v;
    }
    pub fn set_kernelVersion(&mut self, v: ::std::string::String) {
        self.kernelVersion = v;
    }
    pub fn set_oSImage(&mut self, v: ::std::string::String) {
        self.oSImage = v;
    }
    pub fn set_architecture(&mut self, v: ::std::string::String) {
        self.architecture = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectMeta {
    name: String,
    namespace: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: Labels,
    annotations: Annotations,
    owner_references: Vec<OwnerReferences>,
}

impl ObjectMeta {
    pub fn new() -> ObjectMeta {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = v;
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn set_cluster_name(&mut self, v: ::std::string::String) {
        self.cluster_name = v;
    }
    pub fn set_labels(&mut self, v: Labels) {
        self.labels = v;
    }

    pub fn set_annotations(&mut self, v: Annotations) {
        self.annotations = v;
    }

    pub fn set_owner_references(&mut self, v: Vec<OwnerReferences>) {
        self.owner_references = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OwnerReferences {
    kind: String,
    api_version: String,
    name: String,
    uid: String,
    block_owner_deletion: bool,
}

impl OwnerReferences {
    pub fn new() -> OwnerReferences {
        ::std::default::Default::default()
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }
    pub fn set_block_owner_deletion(&mut self, v: bool) {
        self.block_owner_deletion = v;
    }

}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Labels {
    group: String,
    key2: String,
}

impl Labels {
    pub fn new() -> Labels {
        ::std::default::Default::default()
    }
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = v;
    }
    pub fn set_key2(&mut self, v: ::std::string::String) {
        self.key2 = v;
    }

}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Annotations {
    key1: String,
    key2: String,
}

impl Annotations {
    pub fn new() -> Annotations {
        ::std::default::Default::default()
    }
    pub fn set_key1(&mut self, v: ::std::string::String) {
        self.key1 = v;
    }
    pub fn set_key2(&mut self, v: ::std::string::String) {
        self.key2 = v;
    }

}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TypeMeta {
    kind: String,
    api_version: String,
}

impl TypeMeta {
    pub fn new() -> TypeMeta {
        ::std::default::Default::default()
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }

}






#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeGetResponse {
    results: Vec<Node>,
}

impl NodeGetResponse {
    pub fn new() -> NodeGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_node_collection(&mut self, v: Vec<Node>) {
        self.results = v;
    }
}
