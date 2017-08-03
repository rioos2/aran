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
pub struct HorizontalScaling {
    id: u64,
    name: String,
    description: String,
    tags: Vec<String>,
    hs_type: String,
    representation_skew: String,
    target_resource: String,
    metadata: Vec<String>,
    rules: Vec<String>,
    properties: Vec<String>,
    status: String,
    created_at: String,
}
impl HorizontalScaling {
    pub fn new() -> HorizontalScaling {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    pub fn get_id(&self) -> u64 {
        self.id
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

    pub fn set_hs_type(&mut self, v: ::std::string::String) {
        self.hs_type = v;
    }

    pub fn get_hs_type(&self) -> ::std::string::String {
        self.hs_type.clone()
    }

    pub fn set_representation_skew(&mut self, v: ::std::string::String) {
        self.representation_skew = v;
    }

    pub fn get_representation_skew(&self) -> ::std::string::String {
        self.representation_skew.clone()
    }

    pub fn set_target_resource(&mut self, v: ::std::string::String) {
        self.target_resource = v;
    }

    pub fn get_target_resource(&self) -> ::std::string::String {
        self.target_resource.clone()
    }

    pub fn set_metadata(&mut self, v: ::std::vec::Vec<String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> ::std::vec::Vec<String> {
        self.metadata.clone()
    }

    pub fn set_rules(&mut self, v: ::std::vec::Vec<String>) {
        self.rules = v;
    }

    pub fn get_rules(&self) -> ::std::vec::Vec<String> {
        self.rules.clone()
    }

    pub fn set_properties(&mut self, v: ::std::vec::Vec<String>) {
        self.properties = v;
    }

    pub fn get_properties(&self) -> ::std::vec::Vec<String> {
        self.properties.clone()
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
}
