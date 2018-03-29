// Copyright 2018 The Rio Advancement Inc
use std::collections::BTreeMap;
use api::base::{TypeMeta, ObjectMeta, MetaFields};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Secret {
    #[serde(default)]
    id: String,
    secret_type: String,
    object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    #[serde(default)]
    data: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
    #[serde(default)]
    created_at: String,
}
impl Secret {
    const SSH_KEY_PAIR_SIZE: &'static str = "ssh_keypair_size";
    const DEFAULT_SSH_KEY_PAIR_SIZE: &'static str = "2048";

    pub fn new() -> Secret {
        ::std::default::Default::default()
    }
    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Secret {
        Secret {
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

    pub fn set_secret_type(&mut self, v: ::std::string::String) {
        self.secret_type = v;
    }
    pub fn get_secret_type(&self) -> ::std::string::String {
        self.secret_type.clone()
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

    pub fn bit_size(&self) -> Option<u32> {
        *(&self.get_data()
              .get(Self::SSH_KEY_PAIR_SIZE)
              .unwrap_or(&Self::DEFAULT_SSH_KEY_PAIR_SIZE.to_string())
              .parse::<u32>()
              .ok())
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}
impl MetaFields for Secret {
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
    fn decode_secret() {
        let val = r#"{
        "secret_type": "opaque",
        "data": {},
        "metadata":{"origin":"rioos_system"},
        "object_meta": {"name":"ca","account":"875710505434488832","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],
        "created_at":"","deleted_at":"","deletion_grace_period_seconds":0,"finalizers":["orphan"],"cluster_name":"dc1_torono"}
}"#;
        let secret: Secret = json_decode(val).unwrap();
        assert_eq!(secret.secret_type, "opaque");
    }
}
