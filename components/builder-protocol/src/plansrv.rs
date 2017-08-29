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

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Plan {
    id: String,
    name: String,
    description: String,
    tags: Vec<String>,
    camp_version: String,
    url: String,
    origin: String,
    artifacts: Vec<String>,
    services: String,
    created_at: String,
}
impl Plan {
    pub fn new() -> Plan {
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

    pub fn set_tags(&mut self, v: ::std::vec::Vec<String>) {
        self.tags = v;
    }

    pub fn set_camp_version(&mut self, v: ::std::string::String) {
        self.camp_version = v;
    }

    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = v;
    }

    pub fn set_services(&mut self, v: ::std::string::String) {
        self.services = v;
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }

    pub fn set_artifacts(&mut self, v: ::std::vec::Vec<String>) {
        self.artifacts = v;
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }
}