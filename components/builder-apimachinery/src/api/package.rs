// Copyright 2018 The Rio Advancement Inc
use api::base::{MetaFields, ObjectMeta, TypeMeta};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Package {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    version_number: String,
    extension: String,
    #[serde(default)]
    created_at: String,
}
impl Package {
    pub fn new() -> Package {
        ::std::default::Default::default()
    }
    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Package {
        Package {
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

    pub fn set_version_number(&mut self, v: ::std::string::String) {
        self.version_number = v;
    }
    pub fn get_version_number(&self) -> ::std::string::String {
        self.version_number.clone()
    }

    pub fn set_extension(&mut self, v: ::std::string::String) {
        self.extension = v;
    }
    pub fn get_extension(&self) -> ::std::string::String {
        self.extension.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}
impl MetaFields for Package {
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
