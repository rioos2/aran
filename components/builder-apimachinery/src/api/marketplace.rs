// Copyright 2018 The Rio Advancement Inc

use api::base::{TypeMeta, ObjectMeta, Status, MetaFields};
use std::collections::BTreeMap;
use api::blueprint::{PlanProperties};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MarketPlace {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    plans: Vec<PlanProperties>,
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

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
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
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_marketplace() {
        let val = r#"{
            "object_meta":{
                "name":"ubuntu",
                "account":"",
                "created_at":"",
                "deleted_at":"",
                "deletion_grace_period_seconds":30,
                "labels":{},
                "annotations":{},
                "owner_references":[{
                    "kind":"Package",
                    "api_version":"v1",
                    "name":"ubuntu",
                    "uid":"956913916145836032",
                    "block_owner_deletion":false}],
                "initializers":{
                    "pending":[],
                    "result":{
                        "type_meta":{"kind":"","api_version":""},
                        "status":"","message":"","reason":"","details":{"name":"","group":"","kind":"","causes":[],"uid":"","retry_after_seconds":0},"code":0}},"finalizers":["orphan"],"cluster_name":""},
            "category":"machine",
            "version":"16.04",
            "characteristics":{
                "rioos_sh_image_extension": "raw",
                "rioos_sh_market_image_extension": "tar.gz"
                },
            "icon":"ubuntu.png",
            "description":" Ubuntu is an open source software operating system that runs from the desktop, to the cloud, to all your internet connected things ",
            "ports":[],
            "envs":{},
            "lifecycle":{
                "probe": {
                    "env": {},
                    "exec": [],
                    "http_get": {
                        "host": "",
                        "path": "",
                        "port": "",
                        "scheme": ""},
                    "tcp_socket": {
                        "host": "",
                        "port": ""},
                    "http_headers": {}},
                "pre_stop": {"command": []},
                "post_start": {"command": []}},
            "status":{"phase":"SyncPending","message":"","reason":"","conditions":[]}
            }"#;
        let market: MarketPlace = json_decode(val).unwrap();
        assert_eq!(market.category, "machine");
        assert_eq!(market.version, "16.04");
    }

}
