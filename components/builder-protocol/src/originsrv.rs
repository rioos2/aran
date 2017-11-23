// Copyright (c) 2017 RioCorp Inc.
use {asmsrv, servicesrv};
use DEFAULT_API_VERSION;
const ORIGINSLIST: &'static str = "OriginList";

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Origin {
    id: String,
    object_meta: servicesrv::ObjectMetaData,
    type_meta: asmsrv::TypeMeta,
    created_at: String,
}
impl Origin {
    pub fn new() -> Origin {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: servicesrv::ObjectMetaData) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &servicesrv::ObjectMetaData {
        &self.object_meta
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OriginGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Origin>,
}

impl OriginGetResponse {
    pub fn new() -> OriginGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_org_collection(&mut self, v: Vec<Origin>) {
        self.items = v;
        self.kind = ORIGINSLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
    pub fn get_items(&self) -> Vec<Origin> {
        self.items.clone()
    }
}
