// Copyright 2018 The Rio Advancement Inc

use api::base::{TypeMeta, ObjectMeta, Status, MetaFields};

use std::collections::BTreeMap;

use api::blueprint::{LifeCycle, Envs, Port};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MarketPlace {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    category: String, //`Category` represents a plans  relationship to Rio/OS. Valid relationships are  machine, container, application and blockchain.
    version: String, //`version` represents the version of this marketplace software. example : Ubuntu 14.04
    #[serde(default)]
    characteristics: BTreeMap<String, String>, //`Characteristics` The  additional metadata of the marketplace.  example `extension: iso`, This says this marketplace is available as an iso.
    icon: String, //icon: An identifier to represent this marketplace pictorially. example ubuntu.png.
    description: String, //marketplace description
    #[serde(default)]
    ports: Vec<Port>, //`ports` The default port numbers available for this marketplace. example, wordpress app is available in port 80
    //`envs` The required, editable environment variables for a marketplace. Example: In this example, "RUBY_HOME": {"required":"true","value":"/usr/lib/ruby/2.4.9","editable":"false"},"RAILS_APP_HOME":{"required":"true","value":"/home/rails/app","editable":"true"}.
    //RAILS_APP_HOME default home is /var/lib/rioos/railsapp which is a required and editable field.
    #[serde(default)]
    envs: BTreeMap<String, Envs>,
    /*`Lifecycle` describes actions that the management system should take in response to lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the machine/container blocks until the action is complete, unless the machine/container process fails, in which case the handler is aborted.
    There are two hooks that are exposed to Machines/Containers:
    PostStart
    This hook executes immediately after a machine/container is created. In case of containers, however, there is no guarantee that the hook will execute before the container ENTRYPOINT. No parameters are passed to the handler.
    PreStop
    This hook is called immediately before a machine/container is terminated. It is blocking, meaning it is synchronous, so it must complete before the call to delete the machine/container can be sent. No parameters are passed to the handler.*/
    #[serde(default)]
    lifecycle: LifeCycle,
    #[serde(default)]
    status: Status, //`status` : <<old status definition>> Indicates if the marketplace can be used are not. Default no status is available. Will be turned on when the rio.marketplace syncer gets active.
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    created_at: String,
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

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_icon(&mut self, v: ::std::string::String) {
        self.icon = v;
    }
    pub fn get_icon(&self) -> ::std::string::String {
        self.icon.clone()
    }
    pub fn set_characteristics(&mut self, v: BTreeMap<String, String>) {
        self.characteristics = v;
    }

    pub fn get_characteristics(&self) -> &BTreeMap<String, String> {
        &self.characteristics
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

    pub fn get_category(&self) -> ::std::string::String {
        self.category.clone()
    }

    pub fn set_ports(&mut self, v: Vec<Port>) {
        self.ports = v;
    }

    pub fn get_ports(&self) -> &Vec<Port> {
        &self.ports
    }

    pub fn set_envs(&mut self, v: BTreeMap<String, Envs>) {
        self.envs = v;
    }

    pub fn get_envs(&self) -> &BTreeMap<String, Envs> {
        &self.envs
    }

    pub fn set_lifecycle(&mut self, v: LifeCycle) {
        self.lifecycle = v;
    }

    pub fn get_lifecycle(&self) -> &LifeCycle {
        &self.lifecycle
    }

    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = v;
    }

    pub fn get_version(&self) -> ::std::string::String {
        self.version.clone()
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
