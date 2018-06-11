// Copyright 2018 The Rio Advancement Inc
use api::base::{TypeMeta, ObjectMeta, Status, MetaFields};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(default)]
    id: String, // Id an unique identifier in systems of record. Generated during creation of the AssemblyFactory
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta //standard type metadata: kind: Bucket   
}
impl Bucket {
    pub fn new() -> Bucket {
        ::std::default::Default::default()
    }
    //Create a new storage with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Bucket {
        Bucket {
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
   
}
impl MetaFields for Bucket {
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
