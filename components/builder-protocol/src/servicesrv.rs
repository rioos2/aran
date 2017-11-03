// Copyright (c) 2017 RioCorp Inc.
use asmsrv;
use std::collections::BTreeMap;
use DEFAULT_API_VERSION;
pub const SECRETLIST: &'static str = "SecretList";
pub const SERVICELIST: &'static str = "ServiceList";
pub const ENDPOINTSLIST: &'static str = "EndpointsList";
pub const SERVICEACCOUNTLIST: &'static str = "ServiceAccountsList";

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
    unready_addresses: Vec<Addesses>,
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

    pub fn get_unready_addresses(&self) -> &Vec<Addesses> {
        &self.unready_addresses
    }

    pub fn set_unready_addresses(&mut self, v: Vec<Addesses>) {
        self.unready_addresses = v;
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
    pub fn new() -> Spec {
        ::std::default::Default::default()
    }
    pub fn set_selector(&mut self, v: BTreeMap<String, String>) {
        self.selector = v;
    }

    pub fn get_selector(&self) -> &BTreeMap<String, String> {
        &self.selector
    }
    pub fn set_service_type(&mut self, v: ::std::string::String) {
        self.service_type = v;
    }
    pub fn get_service_type(&self) -> ::std::string::String {
        self.service_type.clone()
    }

    pub fn set_loadbalancer_ip(&mut self, v: ::std::string::String) {
        self.loadbalancer_ip = v;
    }
    pub fn get_loadbalancer_ip(&self) -> ::std::string::String {
        self.loadbalancer_ip.clone()
    }
    pub fn set_names(&mut self, v: BTreeMap<String, String>) {
        self.names = v;
    }

    pub fn get_names(&self) -> &BTreeMap<String, String> {
        &self.names
    }
    pub fn set_external_names(&mut self, v: BTreeMap<String, String>) {
        self.external_names = v;
    }

    pub fn get_external_names(&self) -> &BTreeMap<String, String> {
        &self.external_names
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
