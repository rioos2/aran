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
use std::collections::BTreeMap;


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Plan {
    id: String,
    group_name: String,
    description: String,
    tags: Vec<String>,
    camp_version: String,
    url: String,
    origin: String,
    artifacts: Vec<String>,
    services: Vec<Service>,
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
        self.group_name = v;
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

    pub fn set_services(&mut self, v: Vec<Service>) {
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Service {
    name: String,
    description: String,
    href: String,
    characteristics: BTreeMap<String, String>,
}

impl Service {
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn set_href(&mut self, v: ::std::string::String) {
        self.href = v;
    }

    pub fn set_characteristics(&mut self, v: BTreeMap<String, String>) {
        self.characteristics = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PlanGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Plan>,
}

impl PlanGetResponse {
    pub fn new() -> PlanGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_plan_collection(&mut self, v: Vec<Plan>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}
