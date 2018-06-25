// Copyright 2018 The Rio Advancement Inc
use api::base::{TypeMeta, ObjectMeta, MetaFields};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Team {
    #[serde(default)]
    id: String,
    name: String,
    pub object_meta: ObjectMeta,
    metadata: BTreeMap<String, String>,
    #[serde(default)]
    type_meta: TypeMeta,
    #[serde(default)]
    created_at: String,
}
impl Team {
    pub fn new() -> Team {
        ::std::default::Default::default()
    }

    //Create a new team with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Team {
        Team {
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

    pub fn set_team_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
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
impl MetaFields for Team {
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
    use serde_json::{from_str as json_decode};

    use super::*;

    #[test]
    fn decode_team() {
        let val = r#"{
            "type_meta":{"kind": "Team", "api_version": "V1"},
            "name": "development",
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
                               "deletion_grace_period_seconds":30,

                               "finalizers":[],
                               "cluster_name":""
                          },
            "metadata": {
            "origin":"rioos",
             "team":"development"
                }
                }"#;
        let team: Team = json_decode(val).unwrap();
        assert_eq!(team.name, "development");
        assert_eq!(team.metadata.len(), 2);
        assert!(team.metadata.contains_key("origin"));
        assert!(team.metadata.contains_key("team"));
    }
}
