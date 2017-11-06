// Copyright (c) 2017 RioCorp Inc.
use {asmsrv, servicesrv};
use std::collections::BTreeMap;
use DEFAULT_API_VERSION;
pub const STORAGELIST: &'static str = "StorageList";
pub const STOARGEPOOLLIST: &'static str = "StoragePoolList";
pub const DATACENTERLIST: &'static str = "DatacenterList";
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Storage {
    id: String,
    object_meta: servicesrv::ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    name: String,
    host_ip: String,
    storage_type: String,
    storage_info: Disks,
    parameters: BTreeMap<String, String>,
    status: asmsrv::Status,
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

    pub fn set_status(&mut self, v: asmsrv::Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &asmsrv::Status {
        &self.status
    }

    pub fn set_storage_info(&mut self, v: Disks) {
        self.storage_info = v;
    }

    pub fn get_storage_info(&self) -> &Disks {
        &self.storage_info
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
pub struct Disks {
    disks: Vec<Disk>,
}

impl Disks {
    pub fn new() -> Disks {
        ::std::default::Default::default()
    }
    pub fn set_disks(&mut self, v: Vec<Disk>) {
        self.disks = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Disk {
    disk: String,
    disk_type: String,
    point: String,
    size: String,
}

impl Disk {
    pub fn new() -> Disk {
        ::std::default::Default::default()
    }

    pub fn set_disk(&mut self, v: ::std::string::String) {
        self.disk = v;
    }

    pub fn set_disk_type(&mut self, v: ::std::string::String) {
        self.disk_type = v;
    }

    pub fn set_point(&mut self, v: ::std::string::String) {
        self.point = v;
    }
    pub fn set_size(&mut self, v: ::std::string::String) {
        self.size = v;
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
    pub fn set_storage_collection(&mut self, v: Vec<Storage>) {
        self.items = v;
        self.kind = STORAGELIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
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
    pub fn set_dc_collection(&mut self, v: Vec<DataCenter>) {
        self.items = v;
        self.kind = DATACENTERLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
    pub fn get_items(&self) -> Vec<DataCenter> {
        self.items.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct StoragePool {
    id: String,
    object_meta: servicesrv::ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    name: String,
    connector_id: String,
    storage_info: Disks,
    parameters: BTreeMap<String, String>,
    status: asmsrv::Status,
    created_at: String,
}
impl StoragePool {
    pub fn new() -> StoragePool {
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

    pub fn set_connector_id(&mut self, v: ::std::string::String) {
        self.connector_id = v;
    }
    pub fn get_connector_id(&self) -> ::std::string::String {
        self.connector_id.clone()
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

    pub fn set_storage_info(&mut self, v: Disks) {
        self.storage_info = v;
    }

    pub fn get_storage_info(&self) -> &Disks {
        &self.storage_info
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
pub struct StoragePoolGetResponse {
    kind: String,
    api_version: String,
    items: Vec<StoragePool>,
}

impl StoragePoolGetResponse {
    pub fn new() -> StoragePoolGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_storage_pool_collection(&mut self, v: Vec<StoragePool>) {
        self.items = v;
        self.kind = STOARGEPOOLLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
    pub fn get_items(&self) -> Vec<StoragePool> {
        self.items.clone()
    }
}
