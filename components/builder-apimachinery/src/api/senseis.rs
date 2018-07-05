// Copyright 2018 The Rio Advancement Inc

use api::base::{MetaFields, ObjectMeta, TypeMeta};
use api::node::{Spec, NodeStatus};
use std::collections::BTreeMap;

//Rioos prometheus tool automatically allocated "rioos-nodes" job, so we use it
pub const SENSEI_JOBS: &'static str = "job=rioos-masters";

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Senseis {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the node
    node_ip: String, //ip address of the node
    object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    spec: Spec,         //
    status: NodeStatus, //Status is information about the current status of a senseis.
    #[serde(default)]
    metadata: BTreeMap<String, String>,
    #[serde(default)]
    created_at: String,
}

impl Senseis {
    pub fn new() -> Senseis {
        ::std::default::Default::default()
    }
    //Create a new senseis with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Senseis {
        Senseis {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
    pub fn set_node_ip(&mut self, v: ::std::string::String) {
        self.node_ip = v;
    }
    pub fn get_node_ip(&self) -> ::std::string::String {
        self.node_ip.clone()
    }
    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &Spec {
        &self.spec
    }
    pub fn set_status(&mut self, v: NodeStatus) {
        self.status = v;
    }
    pub fn get_status(&self) -> &NodeStatus {
        &self.status
    }
    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for Senseis {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}
