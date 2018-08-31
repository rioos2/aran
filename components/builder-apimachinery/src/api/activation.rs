// Copyright 2018 The Rio Advancement Inc
use api::base::{MetaFields, ObjectMeta, TypeMeta,WhoAmITypeMeta};

pub const BUILTIN_TEAM_RIOOS_SUPERUSER: &'static str = "RIOOS:SUPERUSER";
pub const ACTIVE: &'static str = "active";


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Wizard {
    object_meta: ObjectMeta, //Standard object metadata
    type_meta: TypeMeta, //Standard type metadata: kind: SesssionCreate
    registered: bool,
    licensed: bool,
}

impl MetaFields for Wizard {
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
impl WhoAmITypeMeta for Wizard {
    const MY_KIND: &'static str = "GET:wizards";
}

impl Wizard {
    pub fn new() -> Wizard {
        ::std::default::Default::default()
    }

    pub fn set_license(&mut self, v: bool) {
        self.licensed = v;
    }

    pub fn set_registered(&mut self, v: bool) {
        self.registered = v;
    }
}
