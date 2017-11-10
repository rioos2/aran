// Copyright (c) 2017 RioCorp Inc.

use asmsrv;
use std::collections::BTreeMap;
use serde_json;
use constants::*;


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
pub struct Status {
    capacity: BTreeMap<String, String>,
    allocatable: BTreeMap<String, String>,
    phase: String,
    conditions: Vec<asmsrv::Condition>,
    addresses: Vec<Addresses>,
    node_info: NodeInfo,
}

impl Status {

    pub fn new(capacity: BTreeMap<String, String>, allocatable: BTreeMap<String, String>, phase: &str, conditions: Vec<asmsrv::Condition>, addresses: Vec<Addresses>, node_info: NodeInfo) -> Status {
        Status {
            capacity: capacity,
            allocatable: allocatable,
            phase: phase.to_string(),
            conditions: conditions,
            addresses: addresses,
            node_info: node_info,
        }
    }
    pub fn get_phase(&self) -> ::std::string::String {
        self.phase.clone()
    }

    pub fn get_conditions(&self) -> &Vec<asmsrv::Condition> {
        &self.conditions
    }

    pub fn get_addresses(&self) -> &Vec<Addresses> {
        &self.addresses
    }
    pub fn get_node_info(&self) -> &NodeInfo {
        &self.node_info
    }
    pub fn get_capacity(&self) -> &BTreeMap<String, String> {
        &self.capacity
    }
    pub fn get_allocatable(&self) -> &BTreeMap<String, String> {
        &self.allocatable
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
    pub fn get_node_type(&self) ->  ::std::string::String {
        self.node_type.clone()
    }
    pub fn get_address(&self) ->  ::std::string::String {
        self.address.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NodeInfo {
    machine_id: String,
    system_uuid: String,
    kernel_version: String,
    os_image: String,
    architecture: String,
    bridges: Vec<Bridge>,
}

impl NodeInfo {

    pub fn new(machine_id: &str, system_uuid: &str, kernel_version: &str, os_image: &str, architecture: &str, bridges: Vec<Bridge>) -> NodeInfo {
        NodeInfo {
            machine_id: machine_id.to_string(),
            system_uuid: system_uuid.to_string(),
            kernel_version: kernel_version.to_string(),
            os_image: os_image.to_string(),
            architecture: architecture.to_string(),
            bridges: bridges,
        }
    }
    pub fn get_architecture(&self) -> ::std::string::String {
        self.architecture.clone()
    }
    pub fn get_os_image(&self) -> ::std::string::String {
        self.os_image.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Bridge {
    bridge_name: String,
    physical_device: String,
    bridge_type: String,
}

impl Bridge {
    pub fn new(bridge_name: &str, physical_device: &str, bridge_type: &str) -> Bridge {
        Bridge {
            bridge_name: bridge_name.to_string(),
            physical_device: physical_device.to_string(),
            bridge_type: bridge_type.to_string(),
        }
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
    pub fn set_node_collection(&mut self, v: Vec<Node>) {
        self.items = v;
        self.kind = NODELIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
    pub fn get_items(&self) -> Vec<Node> {
        self.items.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HealthzAllGet {
    title: String,
    guages: Guages,
    statistics: Statistics,
    osusages: Osusages,
    from_date: String,
    to_date: String,
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
    id: String,
    kind: String,
    api_version: String,
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

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
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
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Osusages {
    title: String,
    from_date: String,
    to_date: String,
    cumulative: Cumulative,
    items: Vec<Item>,
}
impl Osusages {
    pub fn new() -> Osusages {
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
    pub fn set_cumulative(&mut self, v: Cumulative) {
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
                node.set_name(data.metric.get("node").unwrap().to_owned());
                node.set_counter(data.value.1.to_owned());
                node.set_id(
                    data.metric
                        .get("node")
                        .unwrap()
                        .replace(".", "_")
                        .to_string(),
                );
                node.set_kind("Node".to_string());
                node.set_api_version(DEFAULT_API_VERSION.to_string());
                collections.push(node);
            }
        }
        collections
    }
}

impl Into<Osusages> for PromResponse {
    fn into(mut self) -> Osusages {
        let mut osusage = Osusages::new();
        let mut item = Item::new();
        //    item.set_id(self.data.metric.get("rioos_assemblyfactory_id").unwrap_or("none").to_owned()); //make them as constants in prometheus.rs
        //    item.set_name(self.data.metric.get("__name__").unwrap_or("none").to_owned()); //make them as constants in prometheus.rs

        if let Data::Vector(ref mut instancevec) = self.data {
            let mut values = Vec::new();
            for data in instancevec.into_iter() {
                let mut value_data = ValueData::new();
                value_data.set_date(data.value.0.to_string().to_owned());
                value_data.set_value(data.value.1.to_owned());
                values.push(value_data);
            }
            item.set_values(values)
        }
        osusage.set_items(vec![item]);
        osusage
    }
}





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
}


impl Into<HealthzAllGetResponse> for HealthzAllGet {
    fn into(self) -> HealthzAllGetResponse {
        let mut health = HealthzAllGetResponse::new();
        health.set_results(self);
        health.set_kind("ReportsStatistics".to_string());
        health.set_api_version(DEFAULT_API_VERSION.to_string());
        health.set_id("ReportsStatistics".to_string());
        health
    }
}
