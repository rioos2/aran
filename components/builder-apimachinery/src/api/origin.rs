// Copyright 2018 The Rio Advancement Inc
use api::base::{MetaFields, ObjectMeta, TypeMeta};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Origin {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the origin
    #[serde(default)]
    type_meta: TypeMeta, //Standard type metadata: kind: Origin
    pub object_meta: ObjectMeta, //Standard object metadata
    name: String,                //Name of the organization. This fields is same as ObjectMeta.name
    #[serde(default)]
    created_at: String, //when origin created
}
impl Origin {
    pub fn new() -> Origin {
        ::std::default::Default::default()
    }
    //Create a new origin with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Origin {
        Origin {
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

    pub fn set_org_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}
impl MetaFields for Origin {
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OriginMembers {
    #[serde(default)]
    id: String,   
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Team
    object_meta: ObjectMeta, ////Standard object metadata
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    updated_at: String,   
}

impl OriginMembers {
    pub fn new() -> OriginMembers {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> OriginMembers {
        OriginMembers {
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

    pub fn set_updated_at(&mut self, v: ::std::string::String) {
        self.updated_at = v;
    }

    pub fn get_updated_at(&self) -> ::std::string::String {
        self.updated_at.clone()
    }
  
   
}

impl MetaFields for OriginMembers {
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


#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_origin() {
        let val = r#"{
            "type_meta":{"kind": "Origin", "api_version": "V1"},
            "name": "rioos",
            "object_meta": {
                               "name":"",
                               "account":"87654321",
                              "labels":{},
                               "annotations":{},
                               "owner_references":[
                               {
                                   "kind":"",
                                   "api_version":"",
                                  "name":"",
                                   "uid":"",
                                  "block_owner_deletion":false
                              }
                               ],
                               "created_at":"",
                               "deleted_at":"",
                               "deletion_grace_period_seconds":0,

                               "finalizers":[],
                               "cluster_name":""
                          }
                }"#;
        let origin: Origin = json_decode(val).unwrap();
        assert_eq!(origin.name, "rioos");
    }
}
