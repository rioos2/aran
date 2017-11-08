// Copyright (c) 2017 RioCorp Inc.
use std::collections::BTreeMap;
use constants::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Plan {
    id: String,
    group_name: String,
    description: String,
    tags: Vec<String>,
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
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_group_name(&mut self, v: ::std::string::String) {
        self.group_name = v;
    }
    pub fn get_group_name(&self) -> ::std::string::String {
        self.group_name.clone()
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

    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = v;
    }

    pub fn get_url(&self) -> ::std::string::String {
        self.url.clone()
    }

    pub fn set_services(&mut self, v: Vec<Service>) {
        self.services = v;
    }

    pub fn get_services(&self) -> &Vec<Service> {
        &self.services
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }

    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
    }

    pub fn set_artifacts(&mut self, v: ::std::vec::Vec<String>) {
        self.artifacts = v;
    }

    pub fn get_artifacts(&self) -> ::std::vec::Vec<String> {
        self.artifacts.clone()
    }


    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
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
    pub fn new(name: &str, description: &str, href: &str, characteristics: BTreeMap<String, String>) -> Service {
        Service {
            name: name.to_string(),
            description: description.to_string(),
            href: href.to_string(),
            characteristics: characteristics,
        }
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
    pub fn set_plan_collection(&mut self, v: Vec<Plan>) {
        self.items = v;
        self.kind = PLANLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
    pub fn get_items(&self) -> Vec<Plan> {
        self.items.clone()
    }
}
