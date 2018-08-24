// Copyright 2018 The Rio Advancement Inc

use api::base::{Condition, MetaFields, ObjectMeta, TypeMeta, WhoAmITypeMeta};
use api::senseis::SENSEI_JOBS;
use chrono::naive::NaiveDateTime;

use serde_json;
use std::collections::BTreeMap;

// The constants to store status.capacity
pub const CAPACITY_CPU: &'static str = "cpu";
pub const CAPACITY_MEMORY: &'static str = "memory";
pub const CAPACITY_STORAGE: &'static str = "storage";
pub const MACHINE_CAPACITY_CPU: &'static str = "machine-cpu";
pub const CONTAINER_CAPACITY_CPU: &'static str = "container-cpu";
pub const CONTAINER_CAPACITY_MEMORY: &'static str = "container-memory";
pub const CONTAINER_CAPACITY_STORAGE: &'static str = "container-storage";

pub const NODE_JOBS: &'static str = "job=rioos-nodes";

pub const METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID: &'static str = "rioos_assemblyfactory_id";
pub const METRIC_LBL_RIOOS_ASSEMBLY_ID: &'static str = "rioos_assembly_id";

pub const NODES: [(&'static str, &'static str); 2] = [("senseis", SENSEI_JOBS), ("ninjas", NODE_JOBS)];

pub const NODES_METRIC_SOURCE: [&'static str; 3] = ["process", "disk", "network"];

pub type SpeedSummary = (String, i32, i32);

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the node
    node_ip: String, //ip address of the node
    object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    spec: Spec, //
    status: NodeStatus, //NodeStatus is information about the current status of a node.
    #[serde(default)]
    metadata: BTreeMap<String, String>,
    #[serde(default)]
    created_at: String,
}

impl Node {
    pub fn new() -> Node {
        ::std::default::Default::default()
    }
    //Create a new node with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Node {
        Node {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_node_ip(&mut self, v: ::std::string::String) {
        self.node_ip = v;
    }
    pub fn get_node_ip(&self) -> ::std::string::String {
        self.node_ip.clone()
    }
    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &Spec {
        &self.spec
    }
    pub fn set_status(&mut self, v: NodeStatus) {
        self.status = v;
    }
    pub fn get_status(&self) -> &NodeStatus {
        &self.status
    }
    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for Node {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}

impl WhoAmITypeMeta for Node {
    const MY_KIND: &'static str = "POST:nodes";
}

/// assembly_cidr:
//  external_id:
//  provider_id:
//  unschedulable:   True: Indicates .. False: .. Who is responsible for doing so ?
/// Taints:
/// Places a taint on node node1. The taint has key key, value value, and taint effect NoSchedule.
/// This means that no assembly will be able to schedule onto node1 unless it has a matching toleration.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Spec {
    assembly_cidr: String,
    external_id: String, //External ID of the node assigned by some machine database
    provider_id: String, //ID of the node assigned by the cloud provider
    unschedulable: bool, //Unschedulable controls node schedulability of new assemblys. By default, node is schedulable.
    taints: Vec<Taints>, //If specified, the node's taints.
}

impl Spec {
    pub fn new(assembly_cidr: &str, external_id: &str, provider_id: &str, unschedulable: bool, taints: Vec<Taints>) -> Spec {
        Spec {
            assembly_cidr: assembly_cidr.to_string(),
            external_id: external_id.to_string(),
            provider_id: provider_id.to_string(),
            unschedulable: unschedulable,
            taints: taints,
        }
    }

    pub fn get_unschedulable(&self) -> bool {
        self.unschedulable.clone()
    }
}

/// the node controller automatically taints a node when certain condition is true.
// The built-in taints currently include:
// node.rioos.sh/not-ready: Node is not ready. This corresponds to the NodeCondition Ready being “False”.
// node.rioos.sh/unreachable: Node is unreachable from the node controller. This corresponds to the NodeCondition Ready being “Unknown”.
// node.rioos.sh/out-of-disk: Node becomes out of disk.
// node.rioos.sh/memory-pressure: Node has memory pressure.
// node.rioos.sh/disk-pressure: Node has disk pressure.
// node.rioos.sh/network-unavailable: Node’s network is unavailable.
// node.cloudprovider.rioos.sh/uninitialized:
// When executor with the cloud provider, it sets this taint on a
// node to mark it as unusable. then when the fontroller from the cloud-controller-manager initializes
//this node, executor removes this taint.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Taints {
    key: String,
    value: String,
    effect: String,
    time_added: String,
}

impl Taints {
    pub fn new(key: &str, value: &str, effect: &str, time_added: &str) -> Taints {
        Taints {
            key: key.to_string(),
            value: value.to_string(),
            effect: effect.to_string(),
            time_added: time_added.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeStatus {
    capacity: BTreeMap<String, String>, //Capacity represents the total resources of a node.
    allocatable: BTreeMap<String, String>, // Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity.
    phase: String, //NodePhase is the recently observed lifecycle phase of the node.
    conditions: Vec<Condition>, //Conditions is an array of current observed node conditions.
    addresses: Vec<Addresses>, //List of addresses reachable to the node.
    node_info: NodeInfo, //Set of ids/uuids to uniquely identify the node.
}

impl NodeStatus {
    pub fn new() -> NodeStatus {
        ::std::default::Default::default()
    }

    pub fn with(capacity: BTreeMap<String, String>, allocatable: BTreeMap<String, String>, phase: &str, conditions: Vec<Condition>, addresses: Vec<Addresses>, node_info: NodeInfo) -> NodeStatus {
        NodeStatus {
            capacity: capacity,
            allocatable: allocatable,
            phase: phase.to_string(),
            conditions: conditions,
            addresses: addresses,
            node_info: node_info,
        }
    }

    pub fn set_phase(&mut self, v: ::std::string::String) {
        self.phase = v;
    }

    pub fn get_phase(&self) -> ::std::string::String {
        self.phase.clone()
    }

    pub fn get_conditions(&self) -> &Vec<Condition> {
        &self.conditions
    }

    pub fn set_addresses(&mut self, v: Vec<Addresses>) {
        self.addresses = v;
    }

    pub fn get_addresses(&self) -> &Vec<Addresses> {
        &self.addresses
    }

    pub fn set_node_info(&mut self, v: NodeInfo) {
        self.node_info = v;
    }

    pub fn get_node_info(&self) -> &NodeInfo {
        &self.node_info
    }

    pub fn set_capacity(&mut self, v: BTreeMap<String, String>) {
        self.capacity = v;
    }

    pub fn get_capacity(&self) -> &BTreeMap<String, String> {
        &self.capacity
    }
    pub fn get_allocatable(&self) -> &BTreeMap<String, String> {
        &self.allocatable
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeFilter {
    #[serde(default)]
    cidrs: Vec<CidrItem>,
    #[serde(default)]
    range_address_from: String,
    #[serde(default)]
    range_address_to: String,
    #[serde(default)]
    ip_type: String,
}

impl NodeFilter {
    pub fn get_cidrs(&self) -> Vec<CidrItem> {
        self.cidrs.clone()
    }

    pub fn get_range_address_from(&self) -> ::std::string::String {
        self.range_address_from.clone()
    }

    pub fn get_range_address_to(&self) -> ::std::string::String {
        self.range_address_to.clone()
    }

    pub fn get_ip_type(&self) -> ::std::string::String {
        self.ip_type.clone()
    }
}

///The status that is used to parse request in /status update of any api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CidrItem {
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub range: u8,
}

///The status that is used to parse request in /status update of any api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeStatusUpdate {
    pub status: NodeStatus,
    #[serde(default)]
    id: String,
}

impl NodeStatusUpdate {
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_status(&mut self, v: NodeStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> &NodeStatus {
        &self.status
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Addresses {
    node_type: String,
    address: String,
}

impl Addresses {
    pub fn new(node_type: &str, address: &str) -> Addresses {
        Addresses {
            node_type: node_type.to_string(),
            address: address.to_string(),
        }
    }
    pub fn get_node_type(&self) -> ::std::string::String {
        self.node_type.clone()
    }
    pub fn get_address(&self) -> ::std::string::String {
        self.address.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeInfo {
    machine_id: String, //MachineID reported by the node. For unique machine identification in the cluster this field is preferred.
    system_uuid: String, //SystemUUID reported by the node. For unique machine identification
    kernel_version: String, //Kernel Version reported by the node from 'uname -r' (e.g. 3.16.0-0.bpo.4-amd64).
    os_image: String, //OS Image reported by the node from /etc/os-release (e.g. Debian GNU/Linux 7 (wheezy)).
    architecture: String, //The Architecture reported by the node
    #[serde(default)]
    bridges: Vec<Bridge>, // List of virtual networking bridge that are created by RioOS
}

impl NodeInfo {
    pub fn new() -> Self {
        ::std::default::Default::default()
    }

    pub fn with(machine_id: &str, system_uuid: &str, kernel_version: &str, os_image: &str, architecture: &str, bridges: Vec<Bridge>) -> NodeInfo {
        NodeInfo {
            machine_id: machine_id.to_string(),
            system_uuid: system_uuid.to_string(),
            kernel_version: kernel_version.to_string(),
            os_image: os_image.to_string(),
            architecture: architecture.to_string(),
            bridges: bridges,
        }
    }

    pub fn set_machine_id(&mut self, v: ::std::string::String) {
        self.machine_id = v;
    }

    pub fn set_architecture(&mut self, v: ::std::string::String) {
        self.architecture = v;
    }

    pub fn get_architecture(&self) -> ::std::string::String {
        self.architecture.clone()
    }

    pub fn set_system_uuid(&mut self, v: ::std::string::String) {
        self.system_uuid = v;
    }

    pub fn set_kernel_version(&mut self, v: ::std::string::String) {
        self.kernel_version = v;
    }

    pub fn set_os_image(&mut self, v: ::std::string::String) {
        self.os_image = v;
    }

    pub fn get_os_image(&self) -> ::std::string::String {
        self.os_image.clone()
    }

    pub fn set_bridges(&mut self, v: Vec<Bridge>) {
        self.bridges = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Bridge {
    bridge_name: String, // Name of the bridge to be used for virtual networking
    physical_device: String, // Physical network interface that are connected to this bridge
    network_types: Vec<String>, //supported networks
    bridge_type: String, //Configured Which type of network to this bridge
}

impl Bridge {
    pub fn new(bridge_name: &str, physical_device: &str, network_types: Vec<String>, bridge_type: &str) -> Bridge {
        Bridge {
            bridge_name: bridge_name.to_string(),
            physical_device: physical_device.to_string(),
            network_types: network_types,
            bridge_type: bridge_type.to_string(),
        }
    }
}

// report data starts here
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HealthzAllGetResponse {
    kind: String,
    api_version: String,
    id: String,
    results: HealthzAllGet,
}

impl HealthzAllGetResponse {
    pub fn new() -> HealthzAllGetResponse {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
    pub fn set_results(&mut self, v: HealthzAllGet) {
        self.results = v;
    }
    pub fn get_results(&self) -> HealthzAllGet {
        self.results.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HealthzAllGet {
    title: String,
    guages: Guages, //average of the cpu ram disk values get from PromResponse
    statistics: Statistics, //ovarall cpu usage of the each node
    osusages: OSUsages, //overall cpu usage of the each os
}

impl HealthzAllGet {
    pub fn new() -> HealthzAllGet {
        ::std::default::Default::default()
    }
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }
    pub fn set_gauges(&mut self, v: Guages) {
        self.guages = v;
    }

    pub fn get_gauges(&self) -> Guages {
        self.guages.clone()
    }
    pub fn set_statistics(&mut self, v: Statistics) {
        self.statistics = v;
    }
    pub fn get_statistics(&mut self) -> Statistics {
        self.statistics.clone()
    }
    pub fn set_osusages(&mut self, v: OSUsages) {
        self.osusages = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Guages {
    title: String,
    counters: Vec<Counters>, //counters are the collection of ram cpu disk average values in percentage
}
impl Guages {
    pub fn new() -> Guages {
        ::std::default::Default::default()
    }
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }
    pub fn set_counters(&mut self, v: Vec<Counters>) {
        self.counters = v;
    }
    pub fn get_counters(&self) -> Vec<Counters> {
        self.counters.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Counters {
    name: String,
    description: String,
    cpu: String,
    counter: String, //average value in percetange
}
impl Counters {
    pub fn new() -> Counters {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }
    pub fn set_cpu(&mut self, v: ::std::string::String) {
        self.cpu = v;
    }
    pub fn set_counter(&mut self, v: ::std::string::String) {
        self.counter = v;
    }

    pub fn get_counter(&self) -> ::std::string::String {
        self.counter.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Statistics {
    title: String,
    ninjas: Vec<NodeStatistic>,
    senseis: Vec<NodeStatistic>,
}
impl Statistics {
    pub fn new() -> Statistics {
        ::std::default::Default::default()
    }
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }
    pub fn set_ninjas(&mut self, v: Vec<NodeStatistic>) {
        self.ninjas = v;
    }
    pub fn set_senseis(&mut self, v: Vec<NodeStatistic>) {
        self.senseis = v;
    }

    pub fn get_ninjas(&mut self) -> Vec<NodeStatistic> {
        self.ninjas.clone()
    }
    pub fn get_senseis(&mut self) -> Vec<NodeStatistic> {
        self.senseis.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeStatistic {
    id: String,
    kind: String,
    api_version: String,
    name: String,
    description: String,
    cpu: String,
    counter: String,
    cost_of_consumption: String,
    health: String,
    network: Vec<NetworkSpeed>,
    process: Vec<BTreeMap<String, Vec<BTreeMap<String, String>>>>,
    disk: Vec<BTreeMap<String, String>>,
}
impl NodeStatistic {
    pub fn new() -> NodeStatistic {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }
    pub fn set_cpu(&mut self, v: ::std::string::String) {
        self.cpu = v;
    }
    pub fn set_counter(&mut self, v: ::std::string::String) {
        self.counter = v;
    }
    pub fn set_cost_of_consumption(&mut self, v: ::std::string::String) {
        self.cost_of_consumption = v;
    }
    pub fn set_health(&mut self, v: ::std::string::String) {
        self.health = v;
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
    pub fn set_type_meta(&mut self, type_meta: TypeMeta) {
        self.kind = type_meta.kind;
        self.api_version = type_meta.api_version;
    }

    pub fn set_network_speed(&mut self, v: Vec<NetworkSpeed>) {
        self.network = v;
    }

    pub fn set_process(&mut self, v: Vec<BTreeMap<String, Vec<BTreeMap<String, String>>>>) {
        self.process = v;
    }

    pub fn set_disk(&mut self, v: Vec<BTreeMap<String, String>>) {
        self.disk = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NetworkSpeed {
    name: String,
    error: Vec<SpeedSummary>,
    throughput: Vec<SpeedSummary>,
}

impl NetworkSpeed {
    pub fn new() -> NetworkSpeed {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_throughput(&mut self, v: Vec<SpeedSummary>) {
        self.throughput = v;
    }
    pub fn set_error(&mut self, v: Vec<SpeedSummary>) {
        self.error = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NetworkDevice {
    pub name: String,
    pub error: Vec<MatrixItem>,
    pub throughput: Vec<MatrixItem>,
}

impl NetworkDevice {
    pub fn new() -> NetworkDevice {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_throughput(&mut self, v: Vec<MatrixItem>) {
        self.throughput = v;
    }
    pub fn set_error(&mut self, v: Vec<MatrixItem>) {
        self.error = v;
    }
}

impl WhoAmITypeMeta for NodeStatistic {
    const MY_KIND: &'static str = "POST:nodes";
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OSUsages {
    title: String,
    from_date: String,
    to_date: String,
    cumulative: Counters,
    items: Vec<Item>,
}
impl OSUsages {
    pub fn new() -> OSUsages {
        ::std::default::Default::default()
    }
    pub fn get_items(&self) -> Vec<Item> {
        self.items.clone()
    }
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }
    pub fn set_from_date(&mut self, v: ::std::string::String) {
        self.from_date = v;
    }
    pub fn set_to_date(&mut self, v: ::std::string::String) {
        self.to_date = v;
    }
    pub fn set_cumulative(&mut self, v: Counters) {
        self.cumulative = v;
    }
    pub fn set_items(&mut self, v: Vec<Item>) {
        self.items = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Cumulative {
    cpu: String,
    counter: String,
    alert: String,
}

impl Cumulative {
    pub fn new() -> Cumulative {
        ::std::default::Default::default()
    }
    pub fn set_cpu(&mut self, v: ::std::string::String) {
        self.cpu = v;
    }
    pub fn set_counter(&mut self, v: ::std::string::String) {
        self.counter = v;
    }
    pub fn set_alert(&mut self, v: ::std::string::String) {
        self.alert = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Item {
    id: String,
    name: String,
    values: Vec<ValueData>,
}

impl Item {
    pub fn new() -> Item {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_values(&mut self, v: Vec<ValueData>) {
        self.values = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ValueData {
    date: String,
    value: String,
}

impl ValueData {
    pub fn new() -> ValueData {
        ::std::default::Default::default()
    }
    pub fn set_date(&mut self, v: ::std::string::String) {
        self.date = v;
    }
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct QueryBuilder {
    name: String,
    query: String,
}
impl QueryBuilder {
    pub fn with_name_query(name: String, query: String) -> QueryBuilder {
        QueryBuilder {
            name: name,
            query: query,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PrometheusQuery {
    querys: Vec<QueryBuilder>,
}

impl PrometheusQuery {
    pub fn with_querys(querys: Vec<QueryBuilder>) -> PrometheusQuery {
        PrometheusQuery { querys: querys }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetricResponse {
    status: StatusData,
    pub data: Vec<PromResponse>,
}

type Timestamp = f64;
type Value = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusData {
    Success,
    Error,
}

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InvalidExpression(String),
    Timeout(String),
    InvalidResponse(serde_json::Error),
    Unexpected(u16),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixItem {
    pub metric: BTreeMap<String, String>,
    pub values: Vec<Scalar>,
}
pub type Matrix = Vec<MatrixItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantVecItem {
    pub metric: BTreeMap<String, String>,
    pub value: Scalar,
}
pub type InstantVec = Vec<InstantVecItem>;

pub type Scalar = (Timestamp, Value);

pub type Str = (Timestamp, String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType", content = "result")]
#[serde(rename_all = "lowercase")]
pub enum Data {
    Matrix(Matrix),
    Vector(InstantVec),
    Scalar(Scalar),
    String(Str),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromResponse {
    pub name: String,
    pub result: Data,
    #[serde(rename = "errorType")]
    #[serde(default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

impl PromResponse {
    pub fn new() -> PromResponse {
        PromResponse {
            name: "".to_string(),
            result: Data::String((0.0, "".to_string())),
            error_type: None,
            error: None,
        }
    }
}


//convert the PromResponse into Counters value
impl Into<Counters> for PromResponse {
    fn into(mut self) -> Counters {
        let mut counters = Counters::new();
        counters.set_name(self.name);
        if let Data::Vector(ref mut instancevec) = self.result {
            instancevec
                .into_iter()
                .map(|x| { counters.set_counter(x.value.1.to_owned()); })
                .collect::<Vec<_>>();
        }
        counters
    }
}


//convert the PromResponse into NodeStatistic value
impl Into<Vec<NodeStatistic>> for PromResponse {
    fn into(mut self) -> Vec<NodeStatistic> {
        let mut collections = Vec::new();
        if let Data::Vector(ref mut instancevec) = self.result {
            collections = instancevec
                .into_iter()
                .map(|x| {
                    let mut node = NodeStatistic::new();
                    let instance = x.metric
                        .get("instance")
                        .unwrap_or(&"".to_string())
                        .to_owned();
                    let ins: Vec<&str> = instance.split("-").collect();
                    node.set_name(ins[1].to_string());
                    node.set_counter(x.value.1.to_owned());
                    node.set_id(ins[0].to_string().replace(".", "_").to_string());

                    node.set_kind("Node".to_string());
                    node.set_api_version("v1".to_string());
                    node.set_health("up".to_string());
                    node
                })
                .collect::<Vec<_>>();
        }
        collections
    }
}
//convert the PromResponse into OSUsages value
/*impl Into<OSUsages> for PromResponse {
    fn into(mut self) -> OSUsages {
        let mut osusage = OSUsages::new();
        if let Data::Matrix(ref mut instancevec) = self.data {
            let item_collection = instancevec
                .into_iter()
                .map(|x| {
                    let mut item = Item::new();
                    item.set_id(
                        x.metric
                            .get("rioos_assemblyfactory_id")
                            .unwrap_or(&"none".to_string())
                            .to_owned(),
                    );
                    item.set_name(
                        x.metric
                            .get("rioos_os_name")
                            .unwrap_or(&"none".to_string())
                            .to_owned(),
                    );
                    let values = x.values
                        .clone()
                        .into_iter()
                        .map(|s| {
                            let mut value_data = ValueData::new();
                            value_data.set_date(
                                NaiveDateTime::from_timestamp(s.0.round() as i64, 0)
                                    .to_string()
                                    .to_owned(),
                            );
                            value_data.set_value(s.1.to_owned());
                            value_data
                        })
                        .collect::<Vec<_>>();
                    item.set_values(values);
                    item
                })
                .collect::<Vec<_>>();
            osusage.set_items(item_collection);
        }
        osusage
    }
}*/

impl Into<HealthzAllGetResponse> for HealthzAllGet {
    fn into(self) -> HealthzAllGetResponse {
        let mut health = HealthzAllGetResponse::new();
        health.set_results(self);
        health.set_kind("ReportsStatistics".to_string());
        health.set_api_version("v1".to_string());
        health.set_id("ReportsStatistics".to_string());
        health
    }
}

impl Into<Guages> for Vec<Counters> {
    fn into(self) -> Guages {
        let mut guages = Guages::new();
        guages.set_title("Cumulative operations counter".to_string());
        guages.set_counters(self);
        guages
    }
}

impl Into<NodeStatistic> for Node {
    fn into(self) -> NodeStatistic {
        let mut ns = NodeStatistic::new();
        ns.set_id(self.get_id());
        ns.set_kind(self.type_meta().kind);
        ns.set_api_version(self.type_meta().api_version);
        ns.set_name(self.get_name());
        ns.set_health("down".to_string());
        ns
    }
}

impl Into<BTreeMap<String, String>> for PromResponse {
    fn into(mut self) -> BTreeMap<String, String> {
        let mut data = BTreeMap::new();
        if let Data::Vector(ref mut instancevec) = self.result {
            instancevec
                .iter_mut()
                .map(|x| if x.metric
                    .get(METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID)
                    .is_some()
                {
                    data.insert(
                        x.metric
                            .get(METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID)
                            .unwrap_or(&"".to_string())
                            .to_string(),
                        x.value.1.clone(),
                    );
                } else {
                    data.insert(
                        x.metric
                            .get(METRIC_LBL_RIOOS_ASSEMBLY_ID)
                            .unwrap_or(&"".to_string())
                            .to_string(),
                        x.value.1.clone(),
                    );
                })
                .collect::<Vec<_>>();
        }
        data
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str as json_decode;
    #[test]
    fn decode_node() {
        let val = r#"
        {
        "node_ip": "private_ipv4",
        "status":{
            "capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,
            "allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},
            "phase": "pending",
            "conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],
            "addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"rajesh"}],
            "node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}
        },
        "type_meta":{"kind": "Node", "api_version": "V1"},
        "spec":{
            "assembly_cidr": "2345",
            "external_id": "87654",
            "provider_id": "7654",
            "unschedulable": false,
            "taints": [{
                "key": "key",
                "value": "value",
                "effect": "NoSchedule",
                "time_added": ""
            }]
        },
        "object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}
    }"#;
        let node: Node = json_decode(val).unwrap();
        assert_eq!(node.node_ip, "private_ipv4");
    }

    #[test]
    fn decode_taints() {
        let val = r#"{
            "key": "key",
            "value": "value",
            "effect": "NoSchedule",
            "time_added": ""
        }"#;
        let taints: Taints = json_decode(val).unwrap();
        assert_eq!(taints.key, "key");
        assert_eq!(taints.value, "value");
        assert_eq!(taints.effect, "NoSchedule");
        assert_eq!(taints.time_added, "");
    }

    #[test]
    fn decode_node_spec() {
        let val = r#"{
            "assembly_cidr": "2345",
            "external_id": "87654",
            "provider_id": "7654",
            "unschedulable": false,
            "taints": [{
                "key": "key",
                "value": "value",
                "effect": "NoSchedule",
                "time_added": ""
            }]
        }"#;
        let spec: Spec = json_decode(val).unwrap();
        assert_eq!(spec.assembly_cidr, "2345");
        assert_eq!(spec.external_id, "87654");
        assert_eq!(spec.provider_id, "7654");
        assert_eq!(spec.unschedulable, false);
        assert_eq!(spec.taints.len(), 1);
    }

    #[test]
    fn decode_node_status() {
        let val = r#"{
            "capacity": {"cpu":"4","memory":"16331164 MiB","assemblys":"110","storage":"1633 MiB"} ,
            "allocatable": {"cpu":"4","memory":"16228764 KiB","assemblys":"110","storage":"161 MiB"},
            "phase": "pending",
            "conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}],
            "addresses": [{"node_type":"InternalIP","address":"192.168.2.47"},{"node_type":"Hostname","address":"rajesh"}],
            "node_info":{"machine_id":"589f17c8cc084c078c5d364241433afc","system_uuid":"85EE9345-A1AF-11E3-BE7C-28E347559DE7","kernel_version":"4.4.0-93-generic","os_image":"Ubuntu 16.04.3 LTS","architecture":"amd64","bridges":[{"bridge_name":"riopriv","physical_device":"eth0","network_types":["private_ipv4"],"bridge_type":"linux"}]}
        }"#;
        let node_status: NodeStatus = json_decode(val).unwrap();
        assert_eq!(node_status.phase, "pending");
        assert_eq!(node_status.addresses.len(), 2);
        assert_eq!(node_status.conditions.len(), 1);
        assert_eq!(node_status.capacity.len(), 4);
        assert!(node_status.capacity.contains_key("cpu"));
        assert!(node_status.capacity.contains_key("memory"));
        assert!(node_status.capacity.contains_key("storage"));
        assert_eq!(node_status.allocatable.len(), 4);
        assert!(node_status.allocatable.contains_key("cpu"));
        assert!(node_status.allocatable.contains_key("memory"));
        assert!(node_status.allocatable.contains_key("storage"));
    }

    #[test]
    fn decode_address() {
        let val = r#"{
            "node_type": "InternalIP",
            "address": "192.168.2.47"
        }"#;
        let addr: Addresses = json_decode(val).unwrap();
        assert_eq!(addr.node_type, "InternalIP");
        assert_eq!(addr.address, "192.168.2.47");
    }
    #[test]
    fn decode_node_info() {
        let val = r#"{
            "machine_id": "589f17c8cc084c078c5d364241433afc",
            "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7",
            "kernel_version": "4.4.0-93-generic",
            "os_image": "Ubuntu 16.04.3 LTS",
            "architecture": "amd64",
            "bridges": [{
                "bridge_name": "riopriv",
                "physical_device": "eth0",
                "network_types": ["private_ipv4"],
                "bridge_type": "linux"
            }]
        }"#;
        let addr: NodeInfo = json_decode(val).unwrap();
        assert_eq!(addr.machine_id, "589f17c8cc084c078c5d364241433afc");
        assert_eq!(addr.system_uuid, "85EE9345-A1AF-11E3-BE7C-28E347559DE7");
        assert_eq!(addr.kernel_version, "4.4.0-93-generic");
        assert_eq!(addr.os_image, "Ubuntu 16.04.3 LTS");
        assert_eq!(addr.architecture, "amd64");
        assert_eq!(addr.bridges.len(), 1);
    }

    #[test]
    fn decode_bridge_info() {
        let val = r#"{
            "bridge_name": "riopriv",
            "physical_device": "eth0",
            "network_types": ["private_ipv4"],
            "bridge_type": "linux"
        }"#;
        let bridge: Bridge = json_decode(val).unwrap();
        assert_eq!(bridge.bridge_name, "riopriv");
        assert_eq!(bridge.physical_device, "eth0");
        assert_eq!(bridge.network_types.len(), 1);
        assert_eq!(bridge.bridge_type, "linux");
    }

}
