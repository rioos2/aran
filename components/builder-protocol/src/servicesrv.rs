// Copyright (c) 2017 RioCorp Inc.
use asmsrv;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Secret {
    id: String,
    secret_type: String,
    object_meta: ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    data: BTreeMap<String, String>,
    created_at: String,
}
impl Secret {
    pub fn new() -> Secret {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_secret_type(&mut self, v: ::std::string::String) {
        self.secret_type = v;
    }
    pub fn get_secret_type(&self) -> ::std::string::String {
        self.secret_type.clone()
    }

    pub fn set_data(&mut self, v: BTreeMap<String, String>) {
        self.data = v;
    }

    pub fn get_data(&self) -> &BTreeMap<String, String> {
        &self.data
    }
    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMetaData {
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
pub struct ObjectMetaData {
    name: String,
    origin: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
}

impl ObjectMetaData {
    pub fn new() -> ObjectMetaData {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
    }
    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }
    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }
    pub fn get_uid(&self) -> ::std::string::String {
        self.uid.clone()
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn set_cluster_name(&mut self, v: ::std::string::String) {
        self.cluster_name = v;
    }
    pub fn set_labels(&mut self, v: BTreeMap<String, String>) {
        self.labels = v;
    }

    pub fn set_annotations(&mut self, v: BTreeMap<String, String>) {
        self.annotations = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SecretGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Secret>,
}

impl SecretGetResponse {
    pub fn new() -> SecretGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_secret_collection(&mut self, v: Vec<Secret>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ServiceAccount {
    id: String,
    object_meta: ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    secrets: ObjectReference,
    created_at: String,
}
impl ServiceAccount {
    pub fn new() -> ServiceAccount {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_secrets(&mut self, v: ObjectReference) {
        self.secrets = v;
    }

    pub fn get_secrets(&self) -> &ObjectReference {
        &self.secrets
    }
    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMetaData {
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
pub struct ServiceAccountGetResponse {
    kind: String,
    api_version: String,
    items: Vec<ServiceAccount>,
}

impl ServiceAccountGetResponse {
    pub fn new() -> ServiceAccountGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_service_collection(&mut self, v: Vec<ServiceAccount>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectReference {
    kind: String,
    name: String,
    origin: String,
    uid: String,
}

impl ObjectReference {
    pub fn new() -> ObjectReference {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct EndPoints {
    id: String,
    target_ref: String,
    subsets: Subsets,
    object_meta: ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    created_at: String,
}

impl EndPoints {
    pub fn new() -> EndPoints {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_target_ref(&mut self, v: ::std::string::String) {
        self.target_ref = v;
    }
    pub fn get_target_ref(&self) -> ::std::string::String {
        self.target_ref.clone()
    }
    pub fn set_subsets(&mut self, v: Subsets) {
        self.subsets = v;
    }

    pub fn get_subsets(&self) -> &Subsets {
        &self.subsets
    }


    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMetaData {
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
pub struct Subsets {
    addresses: Vec<Addesses>,
    not_ready_addresses: Vec<Addesses>,
    ports: Vec<Ports>,
}

impl Subsets {
    pub fn new() -> Subsets {
        ::std::default::Default::default()
    }

    pub fn set_addresses(&mut self, v: Vec<Addesses>) {
        self.addresses = v;
    }

    pub fn get_addresses(&self) -> &Vec<Addesses> {
        &self.addresses
    }

    pub fn get_not_ready_addresses(&self) -> &Vec<Addesses> {
        &self.not_ready_addresses
    }

    pub fn set_not_ready_addresses(&mut self, v: Vec<Addesses>) {
        self.not_ready_addresses = v;
    }

    pub fn get_ports(&self) -> &Vec<Ports> {
        &self.ports
    }

    pub fn set_ports(&mut self, v: Vec<Ports>) {
        self.ports = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Addesses {
    name: String,
    protocol_version: String,
    ip: String,
}

impl Addesses {
    pub fn new() -> Addesses {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }
    pub fn set_protocol_version(&mut self, v: ::std::string::String) {
        self.protocol_version = v;
    }
    pub fn get_protocol_version(&self) -> ::std::string::String {
        self.protocol_version.clone()
    }
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }
    pub fn get_ip(&self) -> ::std::string::String {
        self.ip.clone()
    }

}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Ports {
    name: String,
    port: String,
    protocol: String,
}

impl Ports {
    pub fn new() -> Ports {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }
    pub fn set_protocol(&mut self, v: ::std::string::String) {
        self.protocol = v;
    }
    pub fn get_protocol(&self) -> ::std::string::String {
        self.protocol.clone()
    }
    pub fn set_port(&mut self, v: ::std::string::String) {
        self.port = v;
    }
    pub fn get_port(&self) -> ::std::string::String {
        self.port.clone()
    }

}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct EndpointsGetResponse {
    kind: String,
    api_version: String,
    items: Vec<EndPoints>,
}

impl EndpointsGetResponse {
    pub fn new() -> EndpointsGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_end_collection(&mut self, v: Vec<EndPoints>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.api_version = s;
    }
}
