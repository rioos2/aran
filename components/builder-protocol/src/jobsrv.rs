// Copyright (c) 2017 RioCorp Inc.
use {asmsrv, servicesrv,DEFAULT_API_VERSION};
use std::collections::BTreeMap;
pub const JOBSLIST: &'static str = "JobsList";


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Jobs {
    id: String,
    type_meta: asmsrv::TypeMeta,
    object_meta: servicesrv::ObjectMetaData,
    spec: SpecData,
    status: asmsrv::Status,
    created_at: String,
}

impl Jobs {
    pub fn new() -> Jobs {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_spec(&mut self, v: SpecData) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &SpecData {
        &self.spec
    }
    pub fn set_status(&mut self, v: asmsrv::Status) {
        self.status = v;
    }
    pub fn get_status(&self) -> &asmsrv::Status {
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

    pub fn set_object_meta(&mut self, v: servicesrv::ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &servicesrv::ObjectMetaData {
        &self.object_meta
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SpecData {
node_id: String,
target_ref: String,
selector: BTreeMap<String, String>,
}

impl SpecData {
    pub fn new() -> SpecData {
        ::std::default::Default::default()
    }
    pub fn set_node_id(&mut self, v: ::std::string::String) {
        self.node_id = v;
    }
    pub fn get_node_id(&self) -> ::std::string::String {
        self.node_id.clone()
    }
    pub fn set_target_ref(&mut self, v: ::std::string::String) {
        self.target_ref = v;
    }
    pub fn get_target_ref(&self) -> ::std::string::String {
        self.target_ref.clone()
    }
    pub fn set_selector(&mut self, v: BTreeMap<String, String>) {
        self.selector = v;
    }
    pub fn get_selector(&self) -> &BTreeMap<String, String> {
        &self.selector
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct JobGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Jobs>,
}

impl JobGetResponse {
    pub fn new() -> JobGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_jobs_collection(&mut self, v: Vec<Jobs>) {
        self.items = v;
        self.kind = JOBSLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
}
