// Copyright 2018 The Rio Advancement Inc
use super::session::Session;
use api::base::ObjectReference;
use api::base::{MetaFields, ObjectMeta, TypeMeta};
use std::collections::BTreeMap;
use cache::inject::ServiceAccountFeeder;
use api::base::IdGet;

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
    roles: Vec<String>,
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

    pub fn set_roles(&mut self, v: ::std::vec::Vec<String>) {
        self.roles = v;
    }

    pub fn get_roles(&self) -> ::std::vec::Vec<String> {
        self.roles.clone()
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ServiceAccountRoles {
    name: String,
    roles: Vec<String>,
}

impl ServiceAccountRoles {
    pub fn new() -> ServiceAccountRoles {
        ::std::default::Default::default()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }    

    pub fn set_roles(&mut self, v: ::std::vec::Vec<String>) {
        self.roles = v;
    }

    pub fn get_roles(&self) -> ::std::vec::Vec<String> {
        self.roles.clone()
    }
}

impl ServiceAccountFeeder for ServiceAccountRoles {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_name(), "_service_account".to_string())
    }

    fn ifeed(&mut self, m: Option<Vec<String>>) {
        match m {
            Some(roles) => self.set_roles(roles),
            None => {}
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;
    #[test]
    fn decode_sevice_account() {
        let val = r#"{
            "object_meta":{
                "name":"assemblyfactory-controller",
                "labels":{"group":"development"},
                "annotations":{"rioos.io/serviceaccount":"job"}},
            "metadata":{
                "origin":"rioos_system"},
            "secrets":[
            {
                "name":"controller-shared-informers-token-6b6qh"
            }]
        }"#;
        let serv_acc: ServiceAccount = json_decode(val).unwrap();
        assert_eq!(serv_acc.secrets.len(), 1);
        assert!(serv_acc.metadata.contains_key("origin"));
    }
}
