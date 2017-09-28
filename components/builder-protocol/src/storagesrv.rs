// Copyright (c) 2017 RioCorp Inc.
use {asmsrv, servicesrv};
use std::collections::BTreeMap;


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Storage {
    id: String,
    object_meta: servicesrv::ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    name: String,
    host_ip: String,
    storage_type: String,
    parameters: BTreeMap<String, String>,
    status: StorageStatus,
    created_at: String,
}
impl Storage {
    pub fn new() -> Storage {
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

    pub fn set_host_ip(&mut self, v: ::std::string::String) {
        self.host_ip = v;
    }
    pub fn get_host_ip(&self) -> ::std::string::String {
        self.host_ip.clone()
    }

    pub fn set_storage_type(&mut self, v: ::std::string::String) {
        self.storage_type = v;
    }
    pub fn get_storage_type(&self) -> ::std::string::String {
        self.storage_type.clone()
    }

    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_status(&mut self, v: StorageStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> &StorageStatus {
        &self.status
    }

    pub fn set_paramaters(&mut self, v: BTreeMap<String, String>) {
        self.parameters = v;
    }

    pub fn get_parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }


    pub fn set_object_meta(&mut self, v: servicesrv::ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &servicesrv::ObjectMetaData {
        &self.object_meta
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StorageStatus {
    health_status: String,
    reason: String,
    message: String,
}

impl StorageStatus {
    pub fn new() -> StorageStatus {
        ::std::default::Default::default()
    }

    pub fn set_health_status(&mut self, v: ::std::string::String) {
        self.health_status = v;
    }

    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }

    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StorageGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Storage>,
}

impl StorageGetResponse {
    pub fn new() -> StorageGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_storage_collection(&mut self, v: Vec<Storage>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DataCenter {
    id: String,
    object_meta: servicesrv::ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    name: String,
    nodes: Vec<String>,
    networks: Vec<String>,
    enabled: bool,
    storage: String,
    advanced_settings: BTreeMap<String, String>,
    flag: String,
    currency: String,
    status: asmsrv::Status,
    created_at: String,
}
impl DataCenter {
    pub fn new() -> DataCenter {
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

    pub fn set_flag(&mut self, v: ::std::string::String) {
        self.flag = v;
    }
    pub fn get_flag(&self) -> ::std::string::String {
        self.flag.clone()
    }

    pub fn set_currency(&mut self, v: ::std::string::String) {
        self.currency = v;
    }
    pub fn get_currency(&self) -> ::std::string::String {
        self.currency.clone()
    }

    pub fn set_networks(&mut self, v: ::std::vec::Vec<String>) {
        self.networks = v;
    }
    pub fn get_networks(&self) -> ::std::vec::Vec<String> {
        self.networks.clone()
    }

    pub fn set_nodes(&mut self, v: ::std::vec::Vec<String>) {
        self.nodes = v;
    }
    pub fn get_nodes(&self) -> ::std::vec::Vec<String> {
        self.nodes.clone()
    }

    pub fn set_storage(&mut self, v: ::std::string::String) {
        self.storage = v;
    }
    pub fn get_storage(&self) -> ::std::string::String {
        self.storage.clone()
    }

    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_status(&mut self, v: asmsrv::Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &asmsrv::Status {
        &self.status
    }

    pub fn set_advanced_settings(&mut self, v: BTreeMap<String, String>) {
        self.advanced_settings = v;
    }

    pub fn get_advanced_settings(&self) -> &BTreeMap<String, String> {
        &self.advanced_settings
    }

    pub fn set_object_meta(&mut self, v: servicesrv::ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &servicesrv::ObjectMetaData {
        &self.object_meta
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
    }
    pub fn get_enabled(&self) -> bool {
        self.enabled.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DcGetResponse {
    kind: String,
    api_version: String,
    items: Vec<DataCenter>,
}

impl DcGetResponse {
    pub fn new() -> DcGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_dc_collection(&mut self, v: Vec<DataCenter>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}
