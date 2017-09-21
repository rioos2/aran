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

use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::result;
use std::str::FromStr;
use asmsrv;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Secret {
    id: String,
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
pub struct SecretGet {
    id: String,
}

impl SecretGet {
    pub fn new() -> SecretGet {
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
pub struct ServiceAccountGet {
    name: String,
    origin: String,
}

impl ServiceAccountGet {
    pub fn new() -> ServiceAccountGet {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
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
