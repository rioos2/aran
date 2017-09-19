// Copyright (c) 2017 RioCorp Inc.

//The protocol for the database marshall/unmarshall
//for deployment (assembly, assembly_factory).

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

use serde::{Serialize, Serializer};
use std::result;
use std::fmt;
use error::{Error, Result};
use std::str::FromStr;
use plansrv;
use std::collections::BTreeMap;



#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Assembly {
    id: String,
    name: String,
    uri: String,
    description: String,
    parent_id: String,
    tags: Vec<String>,
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    component_collection: String,
    node: String,
    ip: String,
    urls: String,
    status: Status,
    spec: Option<AssemblyFactory>,
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

    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    pub fn get_uri(&self) -> ::std::string::String {
        self.uri.clone()
    }

    pub fn set_urls(&mut self, v: ::std::string::String) {
        self.urls = v;
    }

    pub fn get_urls(&self) -> ::std::string::String {
        self.urls.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
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

    pub fn set_component_collection(&mut self, v: ::std::string::String) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> ::std::string::String {
        self.component_collection.clone()
    }

    pub fn set_node(&mut self, v: ::std::string::String) {
        self.node = v;
    }

    pub fn get_node(&self) -> ::std::string::String {
        self.node.clone()
    }

    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }

    pub fn get_ip(&self) -> ::std::string::String {
        self.ip.clone()
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
    apiVersion: String,
    items: Vec<Assembly>,
}


impl AssemblysGetResponse {
    pub fn new() -> AssemblysGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_assemblys(&mut self, v: Vec<Assembly>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.apiVersion = s;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyGet {
    id: String,
}

impl AssemblyGet {
    pub fn new() -> AssemblyGet {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactory {
    id: String,
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    replicas: u64,
    properties: Properties,
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    plan: String,
    plan_data: Option<plansrv::Plan>,
    external_management_resource: Vec<String>,
    component_collection: ComponentCollection,
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


    pub fn set_component_collection(&mut self, v: ComponentCollection) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> &ComponentCollection {
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
    namespace: String,
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
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = v;
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
pub struct ComponentCollection {
    flavor: String,
    network: String,
}

impl ComponentCollection {
    pub fn new() -> ComponentCollection {
        ::std::default::Default::default()
    }
    pub fn set_flavor(&mut self, v: ::std::string::String) {
        self.flavor = v;
    }
    pub fn set_network(&mut self, v: ::std::string::String) {
        self.network = v;
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
pub struct AssemblyFactoryGet {
    id: String,
}

impl AssemblyFactoryGet {
    pub fn new() -> AssemblyFactoryGet {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactoryGetResponse {
    // message fields
    kind: String,
    apiVersion: String,
    items: Vec<AssemblyFactory>,
}


impl AssemblyFactoryGetResponse {
    pub fn new() -> AssemblyFactoryGetResponse {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_assemblys_factory(&mut self, v: Vec<AssemblyFactory>, r: ::std::string::String, s: ::std::string::String) {
        self.items = v;
        self.kind = r;
        self.apiVersion = s;
    }
}
