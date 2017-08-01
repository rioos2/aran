// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use std::fmt;
use error::{Error, Result};
use std::str::FromStr;



#[derive(Debug, PartialEq, Clone, Default)]
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
    status: String,
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


    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }

    pub fn get_status(&self) -> ::std::string::String {
        self.status.clone()
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

#[derive(PartialEq, Clone, Default)]
pub struct AssemblysGetResponse {
    // message fields
    assembly: Vec<Assembly>,
}


impl AssemblysGetResponse {
    pub fn new() -> AssemblysGetResponse {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_assemblys(&mut self, v: Vec<Assembly>) {
        self.assembly = v;
    }

    pub fn get_assemblys(&self) -> &[Assembly] {
        &self.assembly
    }
}

#[derive(PartialEq, Clone, Debug, Default)]
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


#[derive(Debug, PartialEq, Clone, Default)]
pub struct AssemblyFactory {
    id: u64,
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    replicas: u64,
    properties: String,
    plan: String,
    external_management_resource: Vec<String>,
    component_collection: String,
    status: ::std::option::Option<AssemblyFactoryStatus>,
    opssettings: String,
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

    pub fn set_properties(&mut self, v: ::std::string::String) {
        self.properties = v;
    }

    pub fn get_properties(&self) -> ::std::string::String {
        self.properties.clone()
    }

    pub fn set_component_collection(&mut self, v: ::std::string::String) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> ::std::string::String {
        self.component_collection.clone()
    }


    pub fn set_status(&mut self, v: AssemblyFactoryStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> AssemblyFactoryStatus {
        self.status.unwrap_or(AssemblyFactoryStatus::Pending)
    }

    pub fn set_opssettings(&mut self, v: ::std::string::String) {
        self.opssettings = v;
    }

    pub fn get_opssettings(&self) -> ::std::string::String {
        self.opssettings.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}


#[derive(PartialEq, Clone, Debug, Default)]
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


#[derive(PartialEq, Clone, Default)]
pub struct AssemblyFactoryGetResponse {
    // message fields
    assembly_factory: Vec<AssemblyFactory>,
}


impl AssemblyFactoryGetResponse {
    pub fn new() -> AssemblyFactoryGetResponse {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_assemblys_factory(&mut self, v: Vec<AssemblyFactory>) {
        self.assembly_factory = v;
    }

    pub fn get_assemblys_factory(&self) -> &[AssemblyFactory] {
        &self.assembly_factory
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum AssemblyFactoryStatus {
    Pending,
    Processing,
    Complete,
    Rejected,
    Failed,
    Dispatched,
}

impl AssemblyFactoryStatus {
    pub fn from_str(value: String) -> AssemblyFactoryStatus {
        match &value[..] {
            "Dispatched" => AssemblyFactoryStatus::Dispatched,
            "Pending" => AssemblyFactoryStatus::Pending,
            "Processing" => AssemblyFactoryStatus::Processing,
            "Complete" => AssemblyFactoryStatus::Complete,
            "Rejected" => AssemblyFactoryStatus::Rejected,
            "Failed" => AssemblyFactoryStatus::Failed,
            _ => AssemblyFactoryStatus::Pending,
        }
    }
}

impl fmt::Display for AssemblyFactoryStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssemblyFactoryStatus::Dispatched => write!(f, "Dispatched"),
            AssemblyFactoryStatus::Pending => write!(f, "Pending"),
            AssemblyFactoryStatus::Processing => write!(f, "Processing"),
            AssemblyFactoryStatus::Rejected => write!(f, "Rejected"),
            AssemblyFactoryStatus::Complete => write!(f, "Complete"),
            AssemblyFactoryStatus::Failed => write!(f, "Failed"),
        }
    }
}
