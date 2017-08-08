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

use message::{Persistable, Routable};
use protobuf::{ProtobufEnum, RepeatedField};
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use sharding::InstaId;
use std::result;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HorizontalScaling {
    id: u64,
    name: String,
    description: String,
    tags: Vec<String>,
    hs_type: String,
    representation_skew: String,
    target_resource: String,
    metadata: Vec<String>,
    spec: Spec,
    status: String,
    created_at: String,
}
impl HorizontalScaling {
    pub fn new() -> HorizontalScaling {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    pub fn get_id(&self) -> u64 {
        self.id
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

    pub fn set_hs_type(&mut self, v: ::std::string::String) {
        self.hs_type = v;
    }

    pub fn get_hs_type(&self) -> ::std::string::String {
        self.hs_type.clone()
    }

    pub fn set_representation_skew(&mut self, v: ::std::string::String) {
        self.representation_skew = v;
    }

    pub fn get_representation_skew(&self) -> ::std::string::String {
        self.representation_skew.clone()
    }

    pub fn set_target_resource(&mut self, v: ::std::string::String) {
        self.target_resource = v;
    }

    pub fn get_target_resource(&self) -> ::std::string::String {
        self.target_resource.clone()
    }

    pub fn set_metadata(&mut self, v: ::std::vec::Vec<String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> ::std::vec::Vec<String> {
        self.metadata.clone()
    }


    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> &Spec {
        &self.spec
    }

    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }

    pub fn get_status(&self) -> ::std::string::String {
        self.status.clone()
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
    scale_target_ref: String,
    min_replicas: u64,
    max_replicas: u64,
    metrics: Vec<Metrics>,
}

impl Spec {
    pub fn new() -> Spec {
        ::std::default::Default::default()
    }
    pub fn set_scale_target_ref(&mut self, v: ::std::string::String) {
        self.scale_target_ref = v;
    }

    pub fn set_min_replicas(&mut self, v: u64) {
        self.min_replicas = v;
    }

    pub fn set_max_replicas(&mut self, v: u64) {
        self.max_replicas = v;
    }

    pub fn set_metrics(&mut self, v: Vec<Metrics>) {
        self.metrics = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Metrics {
    metric_type: String,
    object: MetricObject,
    // resource: MetricResource,
}

impl Metrics {
    pub fn new() -> Metrics {
        ::std::default::Default::default()
    }
    pub fn set_metric_type(&mut self, v: ::std::string::String) {
        self.metric_type = v;
    }
    pub fn set_metric_object(&mut self, v: MetricObject) {
        self.object = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MetricObject {
    target: String,
    target_value: u64,
}

impl MetricObject {
    pub fn new() -> MetricObject {
        ::std::default::Default::default()
    }
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = v;
    }
    pub fn set_target_value(&mut self, v: u64) {
        self.target_value = v;
    }
}
