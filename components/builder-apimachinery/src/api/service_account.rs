// Copyright 2018 The Rio Advancement Inc
use api::base::{TypeMeta, ObjectMeta, MetaFields};
use super::session::Session;
use std::collections::BTreeMap;

// ObjectReference contains enough information to let you inspect or modify the referred object.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectReference {   
    kind: String,  
    origin: String, 
    name: String, 
    uid: String, 
    api_version: String, 
    resource_version: String,
    field_path: String 
}

impl ObjectReference {
    pub fn new() -> ObjectReference {
        ::std::default::Default::default()
    }
    
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }

    pub fn get_kind(&self) -> ::std::string::String {
        self.kind.clone()
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }

    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
    }

    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }

    pub fn get_api_version(&self) -> ::std::string::String {
        self.api_version.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
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

    pub fn set_resource_version(&mut self, v: ::std::string::String) {
        self.resource_version = v;
    }

    pub fn get_resource_version(&self) -> ::std::string::String {
        self.resource_version.clone()
    }

    pub fn set_field_path(&mut self, v: ::std::string::String) {
        self.field_path = v;
    }

    pub fn get_field_path(&self) -> ::std::string::String {
        self.field_path.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ServiceAccount {
    #[serde(default)]
    id: String,
    object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    metadata: BTreeMap<String, String>,
    secrets: Vec<ObjectReference>,
    #[serde(default)]
    created_at: String,
}
impl ServiceAccount {
    pub fn new() -> ServiceAccount {
        ::std::default::Default::default()
    }

    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> ServiceAccount {
        ServiceAccount {
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

    pub fn get_secrets(&self) -> Vec<ObjectReference> {
        self.secrets.clone()
    }

    pub fn set_secrets(&mut self, v: Vec<ObjectReference>) {
        self.secrets = v;
    }
   
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}
impl MetaFields for ServiceAccount {
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

impl Into<Session> for ServiceAccount {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());        
        session.set_meta(self.type_meta(), self.object_meta());
        session
    }
}