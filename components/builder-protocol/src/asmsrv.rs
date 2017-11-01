// Copyright (c) 2017 RioCorp Inc.

use plansrv;
use servicesrv;
use std::collections::BTreeMap;
use DEFAULT_API_VERSION;

pub const ASSEMBLYLIST: &'static str = "AssemblyList";
pub const ASSEMBLYFACTORYLIST: &'static str = "AssemblyFactoryList";
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Assembly {
    id: String,
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    selector: Vec<String>,
    name: String,
    uri: String,
    description: String,
    parent_id: String,
    origin: String,
    tags: Vec<String>,
    node: String,
    urls: BTreeMap<String, String>,
    status: Status,
    volumes: Vec<Volume>,
    instance_id: String,
    spec: Option<AssemblyFactory>,
    endpoints: Option<servicesrv::EndPoints>,
    created_at: String,
}

impl Assembly {
    pub fn new() -> Assembly {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn get_parent_id(&self) -> ::std::string::String {
        self.parent_id.clone()
    }

    pub fn set_parent_id(&mut self, v: ::std::string::String) {
        self.parent_id = v;
    }
    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }

    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    pub fn get_uri(&self) -> ::std::string::String {
        self.uri.clone()
    }

    pub fn set_urls(&mut self, v: BTreeMap<String, String>) {
        self.urls = v;
    }

    pub fn get_urls(&self) -> &BTreeMap<String, String> {
        &self.urls
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }
    pub fn set_instance_id(&mut self, v: ::std::string::String) {
        self.instance_id = v;
    }

    pub fn get_instance_id(&self) -> ::std::string::String {
        self.instance_id.clone()
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

    pub fn set_selector(&mut self, v: ::std::vec::Vec<String>) {
        self.selector = v;
    }

    pub fn get_selector(&self) -> ::std::vec::Vec<String> {
        self.selector.clone()
    }
    pub fn set_type_meta(&mut self, v: TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMeta {
        &self.object_meta
    }

    pub fn set_node(&mut self, v: ::std::string::String) {
        self.node = v;
    }

    pub fn get_node(&self) -> ::std::string::String {
        self.node.clone()
    }

    pub fn set_volumes(&mut self, v: Vec<Volume>) {
        self.volumes = v;
    }

    pub fn get_volumes(&self) -> &Vec<Volume> {
        &self.volumes
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_spec(&mut self, v: Option<AssemblyFactory>) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> Option<AssemblyFactory> {
        self.spec.clone()
    }
    pub fn set_endpoints(&mut self, v: Option<servicesrv::EndPoints>) {
        self.endpoints = v;
    }

    pub fn get_endpoints(&self) -> Option<servicesrv::EndPoints> {
        self.endpoints.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    phase: String,
    message: String,
    reason: String,
    conditions: Vec<Condition>,
}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }
    pub fn set_phase(&mut self, v: ::std::string::String) {
        self.phase = v;
    }
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }
    pub fn set_conditions(&mut self, v: Vec<Condition>) {
        self.conditions = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Volume {
    id: String,
    target: String,
    volume_type: String,
}

impl Volume {
    pub fn new() -> Volume {
        ::std::default::Default::default()
    }
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = v;
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn set_volume_type(&mut self, v: ::std::string::String) {
        self.volume_type = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Condition {
    message: String,
    reason: String,
    status: String,
    last_transition_time: String,
    last_probe_time: String,
    condition_type: String,
}

impl Condition {
    pub fn new() -> Condition {
        ::std::default::Default::default()
    }
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = v;
    }
    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }
    pub fn set_last_transition_time(&mut self, v: ::std::string::String) {
        self.last_transition_time = v;
    }
    pub fn set_last_probe_time(&mut self, v: ::std::string::String) {
        self.last_probe_time = v;
    }
    pub fn set_condition_type(&mut self, v: ::std::string::String) {
        self.condition_type = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblysGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Assembly>,
}


impl AssemblysGetResponse {
    pub fn new() -> AssemblysGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_assemblys(&mut self, v: Vec<Assembly>) {
        self.items = v;
        self.kind = ASSEMBLYLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct IdGet {
    id: String,
    name: String,
}

impl IdGet {
    pub fn new() -> IdGet {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
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
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactory {
    id: String,
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    origin: String,
    replicas: u64,
    properties: Properties,
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    plan: String,
    plan_data: Option<plansrv::Plan>,
    external_management_resource: Vec<String>,
    component_collection: BTreeMap<String, String>,
    status: Status,
    opssettings: OpsSettings,
    created_at: String,
}

impl AssemblyFactory {
    pub fn new() -> AssemblyFactory {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_replicas(&mut self, v: u64) {
        self.replicas = v;
    }

    pub fn get_replicas(&self) -> u64 {
        self.replicas
    }

    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    pub fn get_uri(&self) -> ::std::string::String {
        self.uri.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }

    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
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

    pub fn set_external_management_resource(&mut self, v: ::std::vec::Vec<String>) {
        self.external_management_resource = v;
    }

    pub fn get_external_management_resource(&self) -> ::std::vec::Vec<String> {
        self.external_management_resource.clone()
    }

    pub fn set_plan(&mut self, v: ::std::string::String) {
        self.plan = v;
    }

    pub fn get_plan(&self) -> ::std::string::String {
        self.plan.clone()
    }

    pub fn set_properties(&mut self, v: Properties) {
        self.properties = v;
    }

    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn set_type_meta(&mut self, v: TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMeta {
        &self.object_meta
    }


    pub fn set_component_collection(&mut self, v: BTreeMap<String, String>) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> &BTreeMap<String, String> {
        &self.component_collection
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_opssettings(&mut self, v: OpsSettings) {
        self.opssettings = v;
    }

    pub fn get_opssettings(&self) -> &OpsSettings {
        &self.opssettings
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_plan_data(&mut self, v: Option<plansrv::Plan>) {
        self.plan_data = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectMeta {
    name: String,
    origin: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
    owner_references: Vec<OwnerReferences>,
}

impl ObjectMeta {
    pub fn new() -> ObjectMeta {
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

    pub fn set_owner_references(&mut self, v: Vec<OwnerReferences>) {
        self.owner_references = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OwnerReferences {
    kind: String,
    api_version: String,
    name: String,
    uid: String,
    block_owner_deletion: bool,
}

impl OwnerReferences {
    pub fn new() -> OwnerReferences {
        ::std::default::Default::default()
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }
    pub fn set_block_owner_deletion(&mut self, v: bool) {
        self.block_owner_deletion = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Properties {
    domain: String,
    cloudsetting: String,
    region: String,
    storage_type: String,
}

impl Properties {
    pub fn new() -> Properties {
        ::std::default::Default::default()
    }
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = v;
    }
    pub fn set_cloudsetting(&mut self, v: ::std::string::String) {
        self.cloudsetting = v;
    }
    pub fn set_region(&mut self, v: ::std::string::String) {
        self.region = v;
    }
    pub fn set_storage_type(&mut self, v: ::std::string::String) {
        self.storage_type = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TypeMeta {
    kind: String,
    api_version: String,
}

impl TypeMeta {
    pub fn new() -> TypeMeta {
        ::std::default::Default::default()
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OpsSettings {
    nodeselector: String,
    priority: String,
    nodename: String,
    restartpolicy: String,
}

impl OpsSettings {
    pub fn new() -> OpsSettings {
        ::std::default::Default::default()
    }
    pub fn set_nodeselector(&mut self, v: ::std::string::String) {
        self.nodeselector = v;
    }
    pub fn set_priority(&mut self, v: ::std::string::String) {
        self.priority = v;
    }
    pub fn set_nodename(&mut self, v: ::std::string::String) {
        self.nodename = v;
    }
    pub fn set_restartpolicy(&mut self, v: ::std::string::String) {
        self.restartpolicy = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactoryGetResponse {
    // message fields
    kind: String,
    api_version: String,
    items: Vec<AssemblyFactory>,
}


impl AssemblyFactoryGetResponse {
    pub fn new() -> AssemblyFactoryGetResponse {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_assemblys_factory(&mut self, v: Vec<AssemblyFactory>) {
        self.items = v;
        self.kind = ASSEMBLYFACTORYLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
}
