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
use asmsrv;
use std::collections::BTreeMap;
use serde_json;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    id: String,
    object_meta: asmsrv::ObjectMeta,
    type_meta: asmsrv::TypeMeta,
    spec: Spec,
    status: Status,
    created_at: String,
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
    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: asmsrv::ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &asmsrv::ObjectMeta {
        &self.object_meta
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Spec {
    assembly_cidr: String,
    external_id: String,
    provider_id: String,
    unschedulable: bool,
    taints: Vec<Taints>,
}

impl Spec {
    pub fn new() -> Spec {
        ::std::default::Default::default()
    }
    pub fn set_assembly_cidr(&mut self, v: ::std::string::String) {
        self.assembly_cidr = v;
    }
    pub fn set_external_id(&mut self, v: ::std::string::String) {
        self.external_id = v;
    }
    pub fn set_provider_id(&mut self, v: ::std::string::String) {
        self.provider_id = v;
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
    time_added: String,
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
    pub fn set_time_added(&mut self, v: ::std::string::String) {
        self.time_added = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    capacity: BTreeMap<String, String>,
    allocatable: BTreeMap<String, String>,
    phase: String,
    conditions: Vec<asmsrv::Condition>,
    addresses: Vec<Addresses>,
    node_info: NodeInfo,
}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    pub fn set_capacity(&mut self, v: BTreeMap<String, String>) {
        self.capacity = v;
    }
    pub fn set_allocatable(&mut self, v: BTreeMap<String, String>) {
        self.allocatable = v;
    }
    pub fn set_node_info(&mut self, v: NodeInfo) {
        self.node_info = v;
    }
    pub fn set_phase(&mut self, v: ::std::string::String) {
        self.phase = v;
    }
    pub fn set_conditions(&mut self, v: Vec<asmsrv::Condition>) {
        self.conditions = v;
    }
    pub fn set_addresses(&mut self, v: Vec<Addresses>) {
        self.addresses = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Addresses {
    node_type: String,
    address: String,
}

impl Addresses {
    pub fn new() -> Addresses {
        ::std::default::Default::default()
    }

    pub fn set_node_type(&mut self, v: ::std::string::String) {
        self.node_type = v;
    }
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeInfo {
    machine_id: String,
    system_uuid: String,
    kernel_version: String,
    os_image: String,
    architecture: String,
}

impl NodeInfo {
    pub fn new() -> NodeInfo {
        ::std::default::Default::default()
    }
    pub fn set_machine_id(&mut self, v: ::std::string::String) {
        self.machine_id = v;
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
    pub fn set_architecture(&mut self, v: ::std::string::String) {
        self.architecture = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Node>,
}

impl NodeGetResponse {
    pub fn new() -> NodeGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_node_collection(&mut self, v: Vec<Node>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HealthzAllGetResponse {
    title: String,
    guages: Guages,
    statistics: Statistics,
    osusages: Osusages,
    from_date: String,
    to_date: String,
}

impl HealthzAllGetResponse {
    pub fn new() -> HealthzAllGetResponse {
        ::std::default::Default::default()
    }
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }
    pub fn set_gauges(&mut self, v: Guages) {
        self.guages = v;
    }
    pub fn set_statistics(&mut self, v: Statistics) {
        self.statistics = v;
    }
    pub fn set_osusages(&mut self, v: Osusages) {
        self.osusages = v;
    }
    pub fn set_from_date(&mut self, v: ::std::string::String) {
        self.from_date = v;
    }
    pub fn set_to_date(&mut self, v: ::std::string::String) {
        self.to_date = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Guages {
    title: String,
    counters: Vec<Counters>,
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
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Counters {
    name: String,
    description: String,
    cpu: String,
    counter: String,
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
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Statistics {
    title: String,
    nodes: Vec<NodeStatistic>,
}
impl Statistics {
    pub fn new() -> Statistics {
        ::std::default::Default::default()
    }
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }
    pub fn set_nodes(&mut self, v: Vec<NodeStatistic>) {
        self.nodes = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeStatistic {
    name: String,
    description: String,
    cpu: String,
    counter: String,
    cost_of_consumption: String,
    health: String,
}
impl NodeStatistic {
    pub fn new() -> NodeStatistic {
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
    pub fn set_cost_of_consumption(&mut self, v: ::std::string::String) {
        self.cost_of_consumption = v;
    }
    pub fn set_health(&mut self, v: ::std::string::String) {
        self.health = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Osusages {
    title: String,
    from_date: String,
    to_date: String,
    cumulative: Cumulative,
    item: Vec<Item>,
}
impl Osusages {
    pub fn new() -> Osusages {
        ::std::default::Default::default()
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
    pub fn set_cumulative(&mut self, v: Cumulative) {
        self.cumulative = v;
    }
    pub fn set_item(&mut self, v: Vec<Item>) {
        self.item = v;
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
    value: Vec<ValueData>,
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
    pub fn set_value(&mut self, v: Vec<ValueData>) {
        self.value = v;
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
    pub status: StatusData,
    pub data: Data,
    #[serde(rename = "errorType")]
    #[serde(default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}



impl Into<Counters> for PromResponse {
    fn into(mut self) -> Counters {
        let mut counters = Counters::new();
        if let Data::Vector(ref mut instancevec) = self.data {
            for data in instancevec.into_iter() {
                counters.set_name(data.metric.get("__name__").unwrap().to_owned());
                counters.set_counter(data.value.1.to_owned());
            }
        }
        counters
    }
}

impl Into<Vec<NodeStatistic>> for PromResponse {
    fn into(mut self) -> Vec<NodeStatistic> {
        let mut collections = Vec::new();
        if let Data::Vector(ref mut instancevec) = self.data {
            for data in instancevec.into_iter() {
                let mut node = NodeStatistic::new();
                node.set_name(data.metric.get("instance").unwrap().to_owned());
                node.set_counter(data.value.1.to_owned());
                collections.push(node);
            }
        }
        collections
    }
}
