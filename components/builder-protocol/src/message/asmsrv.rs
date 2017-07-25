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


#[derive(Debug, PartialEq, Clone, Default)]
pub struct Assembly {
    id: u64,
    uri: String,
    name: String,
    description: String,
    tags: Vec<String>,
    representation_skew: String,
    external_management_resource: String,
    component_collection: Vec<String>,
    plan: String,
    operation_collection: Vec<String>,
    sensor_collection: Vec<String>,
    metadata: String,
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

    pub fn set_plan(&mut self, v: ::std::string::String) {
        self.plan = v;
    }

    pub fn get_plan(&self) -> ::std::string::String {
        self.plan.clone()
    }

    pub fn set_tags(&mut self, v: ::std::vec::Vec<String>) {
        self.tags = v;
    }

    pub fn get_tags(&self) -> ::std::vec::Vec<String> {
        self.tags.clone()
    }

    pub fn set_representation_skew(&mut self, v: ::std::string::String) {
        self.representation_skew = v;
    }

    pub fn get_representation_skew(&self) -> ::std::string::String {
        self.representation_skew.clone()
    }

    pub fn set_external_management_resource(&mut self, v: ::std::string::String) {
        self.external_management_resource = v;
    }

    pub fn get_external_management_resource(&self) -> ::std::string::String {
        self.external_management_resource.clone()
    }

    pub fn set_component_collection(&mut self, v: ::std::vec::Vec<String>) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> ::std::vec::Vec<String> {
        self.component_collection.clone()
    }

    pub fn set_operation_collection(&mut self, v: ::std::vec::Vec<String>) {
        self.operation_collection = v;
    }

    pub fn get_operation_collection(&self) -> ::std::vec::Vec<String> {
        self.operation_collection.clone()
    }

    pub fn set_sensor_collection(&mut self, v: ::std::vec::Vec<String>) {
        self.sensor_collection = v;
    }

    pub fn get_sensor_collection(&self) -> ::std::vec::Vec<String> {
        self.sensor_collection.clone()
    }

    pub fn set_metadata(&mut self, v: ::std::string::String) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> ::std::string::String {
        self.metadata.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
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
    uri: String,
    name: String,
    description: String,
    tags: Vec<String>,
    representation_skew: String,
    total_items: u64,
    items_per_page: u64,
    start_index: u64,
    items: String,
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

    pub fn set_representation_skew(&mut self, v: ::std::string::String) {
        self.representation_skew = v;
    }

    pub fn get_representation_skew(&self) -> ::std::string::String {
        self.representation_skew.clone()
    }

    pub fn set_total_items(&mut self, v: u64) {
        self.total_items = v;
    }

    pub fn get_total_items(&self) -> u64 {
        self.total_items
    }

    pub fn set_items_per_page(&mut self, v: u64) {
        self.items_per_page = v;
    }

    pub fn get_items_per_page(&self) -> u64 {
        self.items_per_page
    }

    pub fn set_start_index(&mut self, v: u64) {
        self.start_index = v;
    }

    pub fn get_start_index(&self) -> u64 {
        self.start_index
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }


    pub fn set_items(&mut self, v: ::std::string::String) {
        self.items = v;
    }

    pub fn get_items(&self) -> ::std::string::String {
        self.items.clone()
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
