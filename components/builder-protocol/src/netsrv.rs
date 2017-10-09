// Copyright (c) 2017 RioCorp Inc.
use std::collections::BTreeMap;
use {asmsrv, servicesrv};
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Network {
    id: String,
    name: String,
    network_type: String,
    subnet_ip: String,
    netmask: String,
    gateway: String,
    bridge_hosts: BTreeMap<String, String>,
    status: asmsrv::Status,
    created_at: String,
    object_meta: servicesrv::ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
}
impl Network {
    pub fn new() -> Network {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_subnet_ip(&mut self, v: ::std::string::String) {
        self.subnet_ip = v;
    }
    pub fn get_subnet_ip(&self) -> ::std::string::String {
        self.subnet_ip.clone()
    }

    pub fn set_network_type(&mut self, v: ::std::string::String) {
        self.network_type = v;
    }
    pub fn get_network_type(&self) -> ::std::string::String {
        self.network_type.clone()
    }

    pub fn set_netmask(&mut self, v: ::std::string::String) {
        self.netmask = v;
    }
    pub fn get_netmask(&self) -> ::std::string::String {
        self.netmask.clone()
    }

    pub fn set_gateway(&mut self, v: ::std::string::String) {
        self.gateway = v;
    }
    pub fn get_gateway(&self) -> ::std::string::String {
        self.gateway.clone()
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

    pub fn set_bridge_hosts(&mut self, v: BTreeMap<String, String>) {
        self.bridge_hosts = v;
    }

    pub fn get_bridge_hosts(&self) -> &BTreeMap<String, String> {
        &self.bridge_hosts
    }

    pub fn set_object_meta(&mut self, v: servicesrv::ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &servicesrv::ObjectMetaData {
        &self.object_meta
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct NetworkGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Network>,
}

impl NetworkGetResponse {
    pub fn new() -> NetworkGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_network_collection(&mut self, v: Vec<Network>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}
