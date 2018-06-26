// Copyright 2018 The Rio Advancement Inc

use api::base::{MetaFields, ObjectMeta, Status, TypeMeta};
use api::blueprint::PlanProperties;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MarketPlace {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    #[serde(default)]
    meta_data: BTreeMap<String, String>,
    plans: Vec<PlanProperties>,
    #[serde(default)]
    created_at: String,
    category: String,
    version: String,
    icon: String,
    description: String,
    status: Status,
}

impl MetaFields for MarketPlace {
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

impl MarketPlace {
    pub fn new() -> MarketPlace {
        ::std::default::Default::default()
    }

    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> MarketPlace {
        MarketPlace {
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

    pub fn set_createdat(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }
    pub fn get_createdat(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_plan(&mut self, v: Vec<PlanProperties>) {
        self.plans = v;
    }

    pub fn get_plan(&self) -> Vec<PlanProperties> {
        self.plans.clone()
    }

    pub fn get_category(&self) -> ::std::string::String {
        self.category.clone()
    }

    pub fn set_icon(&mut self, v: ::std::string::String) {
        self.icon = v;
    }

    pub fn get_icon(&self) -> ::std::string::String {
        self.icon.clone()
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    pub fn get_version(&self) -> ::std::string::String {
        self.version.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_category(&mut self, v: ::std::string::String) {
        self.category = v;
    }

    pub fn set_meta_data(&mut self, v: BTreeMap<String, String>) {
        self.meta_data = v;
    }

    pub fn get_meta_data(&self) -> &BTreeMap<String, String> {
        &self.meta_data
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_marketplace() {
        let val = r#"{"object_meta":{
             "name":"ubuntu",
             "account":""
             },
             "plans":[{
                 "object_meta":{
                     "name":"ubuntu",
                     "account":"",
                     "owner_references":[{
                         "kind":"Package",
                         "api_version":"v1",
                         "name":"ubuntu",
                         "uid":"109876543212345678",
                         "block_owner_deletion":false
                         }]
                    },
                 "category": "machine",
                 "version": "16.04",
                 "characteristics" :{
                     "rioos_sh_image_extension": "img",
                     "rioos_sh_market_image_extension":  "tar.gz"
                     },
                 "icon" : "ubuntu.png",
                 "description": " Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
                 "status":{
                     "phase":"SyncPending"
                     },
                 "metadata": {"origin": "rioos_system"},
                 "lifecycle":{
                     "probe": {
                         "env": {},
                         "exec": [],
                         "http_get": {
                             "host": "",
                             "path": "",
                             "port": "",
                             "scheme": ""
                             },
                         "tcp_socket": {"host": "", "port": ""},
                         "http_headers": {}
                         },
                     "pre_stop": {
                         "command": []},
                     "post_start": {"command": []}
                     }
                }],
             "category": "machine",
             "version": "16.04",
             "icon": "ubuntu.png",
             "description": "Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
             "status":{"phase":"SyncPending"}
         }"#;
        let market: MarketPlace = json_decode(val).unwrap();
        assert_eq!(market.category, "machine");
        assert_eq!(market.version, "16.04");
    }

}
