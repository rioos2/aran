// Copyright (c) 2017 RioCorp Inc.
use asmsrv;
use std::collections::BTreeMap;
use constants::*;


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
    pub fn set_secret_collection(&mut self, v: Vec<Secret>) {
        self.items = v;
        self.kind = SECRETLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
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
    pub fn set_service_collection(&mut self, v: Vec<ServiceAccount>) {
        self.items = v;
        self.kind = SERVICEACCOUNTLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
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
    pub fn new(kind: &str, name: &str, origin: &str, uid: &str) -> ObjectReference {
        ObjectReference {
            kind: kind.to_string(),
            name: name.to_string(),
            origin: origin.to_string(),
            uid: uid.to_string(),
        }
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
    unready_addresses: Vec<Addesses>,
    ports: Vec<Ports>,
}

impl Subsets {
    pub fn new(addresses: Vec<Addesses>, unready_addresses: Vec<Addesses>, ports: Vec<Ports>) -> Subsets {
        Subsets {
            addresses: addresses,
            unready_addresses: unready_addresses,
            ports: ports,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Addesses {
    name: String,
    protocol_version: String,
    ip: String,
}

impl Addesses {
    pub fn new(name: &str, protocol_version: &str, ip: &str) -> Addesses {
        Addesses {
            name: name.to_string(),
            protocol_version: protocol_version.to_string(),
            ip: ip.to_string(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Ports {
    name: String,
    port: String,
    protocol: String,
}

impl Ports {
    pub fn new(name: &str, port: &str, protocol: &str) -> Ports {
        Ports {
            name: name.to_string(),
            port: port.to_string(),
            protocol: protocol.to_string(),
        }
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
    pub fn set_end_collection(&mut self, v: Vec<EndPoints>) {
        self.items = v;
        self.kind = ENDPOINTSLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Services {
    id: String,
    spec: Spec,
    status: asmsrv::Status,
    object_meta: ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    created_at: String,
}

impl Services {
    pub fn new() -> Services {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> &Spec {
        &self.spec
    }
    pub fn set_status(&mut self, v: asmsrv::Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &asmsrv::Status {
        &self.status
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
pub struct Spec {
    selector: BTreeMap<String, String>,
    service_type: String,
    loadbalancer_ip: String,
    names: BTreeMap<String, String>,
    external_names: BTreeMap<String, String>,
}

impl Spec {
    pub fn new(selector: BTreeMap<String, String>, service_type: &str, loadbalancer_ip: &str, names: BTreeMap<String, String>, external_names: BTreeMap<String, String>) -> Spec {
        Spec {
            selector: selector,
            service_type: service_type.to_string(),
            loadbalancer_ip: loadbalancer_ip.to_string(),
            names: names,
            external_names: external_names,
        }

    }
    pub fn get_selector(&self) -> &BTreeMap<String, String> {
        &self.selector
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ServicesGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Services>,
}

impl ServicesGetResponse {
    pub fn new() -> ServicesGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_services_collection(&mut self, v: Vec<Services>) {
        self.items = v;
        self.kind = SERVICELIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
}
