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

use message::{Persistable, Routable};
use protobuf::{ProtobufEnum, RepeatedField};
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use sharding::InstaId;
use std::result;
use std::fmt;
use error::{Error, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Assembly {
    id: u64,
    name: String,
    uri: String,
    description: String,
    pub parent_id: u64,
    tags: Vec<String>,
    component_collection: String,
    node: String,
    ip: String,
    urls: String,
    status: Status,
    spec: Option<AssemblyFactory>,
    created_at: String,
}

impl Assembly {
    pub fn new() -> Assembly {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_parent_id(&self) -> u64 {
        self.parent_id
    }

    pub fn set_parent_id(&mut self, v: u64) {
        self.parent_id = v;
    }

    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    pub fn get_uri(&self) -> ::std::string::String {
        self.uri.clone()
    }

    pub fn set_urls(&mut self, v: ::std::string::String) {
        self.urls = v;
    }

    pub fn get_urls(&self) -> ::std::string::String {
        self.urls.clone()
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

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_tags(&mut self, v: ::std::vec::Vec<String>) {
        self.tags = v;
    }

    pub fn get_tags(&self) -> ::std::vec::Vec<String> {
        self.tags.clone()
    }

    pub fn set_component_collection(&mut self, v: ::std::string::String) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> ::std::string::String {
        self.component_collection.clone()
    }

    pub fn set_node(&mut self, v: ::std::string::String) {
        self.node = v;
    }

    pub fn get_node(&self) -> ::std::string::String {
        self.node.clone()
    }

    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }

    pub fn get_ip(&self) -> ::std::string::String {
        self.ip.clone()
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

    pub fn set_spec(&mut self, v: Option<AssemblyFactory>) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> Option<AssemblyFactory> {
        self.spec.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    phase: String,
    message: String,
    reason: String,
    conditions: Vec<Condition>,
}
impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }
    pub fn set_phase(&mut self, v: ::std::string::String) {
        self.phase = v;
    }
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }
    pub fn set_conditions(&mut self, v: Vec<Condition>) {
        self.conditions = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Condition {
    message: String,
    reason: String,
    status: String,
    lastTransitionTime: String,
    lastProbeTime: String,
    conditionType: String,
}

impl Condition {
    pub fn new() -> Condition {
        ::std::default::Default::default()
    }
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }
    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }
    pub fn set_lastTransitionTime(&mut self, v: ::std::string::String) {
        self.lastTransitionTime = v;
    }
    pub fn set_lastProbeTime(&mut self, v: ::std::string::String) {
        self.lastProbeTime = v;
    }
    pub fn set_conditionType(&mut self, v: ::std::string::String) {
        self.conditionType = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblysGetResponse {
    results: Vec<Assembly>,
}


impl AssemblysGetResponse {
    pub fn new() -> AssemblysGetResponse {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_assemblys(&mut self, v: Vec<Assembly>) {
        self.results = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyGet {
    id: ::std::option::Option<u64>,
}

impl AssemblyGet {
    pub fn new() -> AssemblyGet {
        ::std::default::Default::default()
    }

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactory {
    id: u64,
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    replicas: u64,
    properties: Properties,
    plan: String,
    external_management_resource: Vec<String>,
    component_collection: ComponentCollection,
    status: Status,
    opssettings: OpsSettings,
    created_at: String,
}

impl AssemblyFactory {
    pub fn new() -> AssemblyFactory {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn set_replicas(&mut self, v: u64) {
        self.replicas = v;
    }

    pub fn get_replicas(&self) -> u64 {
        self.replicas
    }

    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    pub fn get_uri(&self) -> ::std::string::String {
        self.uri.clone()
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

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_tags(&mut self, v: ::std::vec::Vec<String>) {
        self.tags = v;
    }

    pub fn get_tags(&self) -> ::std::vec::Vec<String> {
        self.tags.clone()
    }

    pub fn set_external_management_resource(&mut self, v: ::std::vec::Vec<String>) {
        self.external_management_resource = v;
    }

    pub fn get_external_management_resource(&self) -> ::std::vec::Vec<String> {
        self.external_management_resource.clone()
    }

    pub fn set_plan(&mut self, v: ::std::string::String) {
        self.plan = v;
    }

    pub fn get_plan(&self) -> ::std::string::String {
        self.plan.clone()
    }

    pub fn set_properties(&mut self, v: Properties) {
        self.properties = v;
    }

    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn set_component_collection(&mut self, v: ComponentCollection) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> &ComponentCollection {
        &self.component_collection
    }


    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_opssettings(&mut self, v: OpsSettings) {
        self.opssettings = v;
    }

    pub fn get_opssettings(&self) -> &OpsSettings {
        &self.opssettings
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ComponentCollection {
    flavor: String,
    network: String,
}

impl ComponentCollection {
    pub fn new() -> ComponentCollection {
        ::std::default::Default::default()
    }
    pub fn set_flavor(&mut self, v: ::std::string::String) {
        self.flavor = v;
    }
    pub fn set_network(&mut self, v: ::std::string::String) {
        self.network = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Properties {
    domain: String,
    cloudsetting: String,
    region: String,
    storage_type: String,
}

impl Properties {
    pub fn new() -> Properties {
        ::std::default::Default::default()
    }
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = v;
    }
    pub fn set_cloudsetting(&mut self, v: ::std::string::String) {
        self.cloudsetting = v;
    }
    pub fn set_region(&mut self, v: ::std::string::String) {
        self.region = v;
    }
    pub fn set_storage_type(&mut self, v: ::std::string::String) {
        self.storage_type = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OpsSettings {
    nodeselector: String,
    priority: String,
    nodename: String,
    restartpolicy: String,
}

impl OpsSettings {
    pub fn new() -> OpsSettings {
        ::std::default::Default::default()
    }
    pub fn set_nodeselector(&mut self, v: ::std::string::String) {
        self.nodeselector = v;
    }
    pub fn set_priority(&mut self, v: ::std::string::String) {
        self.priority = v;
    }
    pub fn set_nodename(&mut self, v: ::std::string::String) {
        self.nodename = v;
    }
    pub fn set_restartpolicy(&mut self, v: ::std::string::String) {
        self.restartpolicy = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactoryGet {
    id: ::std::option::Option<u64>,
}

impl AssemblyFactoryGet {
    pub fn new() -> AssemblyFactoryGet {
        ::std::default::Default::default()
    }

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactoryGetResponse {
    // message fields
    results: Vec<AssemblyFactory>,
}


impl AssemblyFactoryGetResponse {
    pub fn new() -> AssemblyFactoryGetResponse {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_assemblys_factory(&mut self, v: Vec<AssemblyFactory>) {
        self.results = v;
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommonStatus {
    Pending,
    Processing,
    Complete,
    Rejected,
    Failed,
    Dispatched,
}

impl CommonStatus {
    pub fn from_str(value: String) -> CommonStatus {
        match &value[..] {
            "Dispatched" => CommonStatus::Dispatched,
            "Pending" => CommonStatus::Pending,
            "Processing" => CommonStatus::Processing,
            "Complete" => CommonStatus::Complete,
            "Rejected" => CommonStatus::Rejected,
            "Failed" => CommonStatus::Failed,
            _ => CommonStatus::Pending,
        }
    }
}

impl fmt::Display for CommonStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommonStatus::Dispatched => write!(f, "Dispatched"),
            CommonStatus::Pending => write!(f, "Pending"),
            CommonStatus::Processing => write!(f, "Processing"),
            CommonStatus::Rejected => write!(f, "Rejected"),
            CommonStatus::Complete => write!(f, "Complete"),
            CommonStatus::Failed => write!(f, "Failed"),
        }
    }
}
