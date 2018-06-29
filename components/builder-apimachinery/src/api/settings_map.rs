// Copyright 2018 The Rio Advancement Inc
use api::base::{MetaFields, ObjectMeta, TypeMeta};
use api::base::WhoAmITypeMeta;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SettingsMap {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta, //TypeMeta describes an individual object in an API response or request with strings representing the type of the object and its API schema version.
    object_meta: ObjectMeta, //ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
    metadata: BTreeMap<String, String>, //describe the origin name of the settings map
    data: BTreeMap<String, String>, //describe the security information
    #[serde(default)]
    created_at: String,
}
impl SettingsMap {
    pub fn new() -> SettingsMap {
        ::std::default::Default::default()
    }
    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> SettingsMap {
        SettingsMap {
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

    pub fn set_data(&mut self, v: BTreeMap<String, String>) {
        self.data = v;
    }

    pub fn get_data(&self) -> &BTreeMap<String, String> {
        &self.data
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

impl WhoAmITypeMeta for SettingsMap {
    const MY_KIND: &'static str = "POST:settingsmap";
}


impl MetaFields for SettingsMap {
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
    fn decode_ssttingsmap() {
        let val = r#"{
        "metadata": {"origin":"rioos_system"},
        "data": {},
        "object_meta": {"name":"cluster_info","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],
        "created_at":"","deleted_at":"","deletion_grace_period_seconds":0,"finalizers":[""],"cluster_name":""}
}"#;
        let setmap: SettingsMap = json_decode(val).unwrap();
        assert_eq!(setmap.object_meta.name, "cluster_info");
    }
}
