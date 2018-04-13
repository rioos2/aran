// Copyright 2018 The Rio Advancement Inc
use api::base::{TypeMeta, ObjectMeta, MetaFields, Status};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Volumes {
    #[serde(default)]
    id: String, //id an unique identifier in systems of record. Generated during creation of the Volumes
    pub object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Volumes
    mount_path: String,          //The mount path/pool name of the block device
    allocated: String,           //The size of the storage allocated.
    //Most recently observed status of the service. Populated by the system. Read-only.  Initially during submission, the status is "pending"
    status: Status,
    #[serde(default)]
    created_at: String,
}
impl Volumes {
    pub fn new() -> Volumes {
        ::std::default::Default::default()
    }

    //Create a new team with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Volumes {
        Volumes {
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

    pub fn set_mount_path(&mut self, v: ::std::string::String) {
        self.mount_path = v;
    }
    pub fn get_mount_path(&self) -> ::std::string::String {
        self.mount_path.clone()
    }

    pub fn set_allocated(&mut self, v: ::std::string::String) {
        self.allocated = v;
    }
    pub fn get_allocated(&self) -> ::std::string::String {
        self.allocated.clone()
    }
    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}
impl MetaFields for Volumes {
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
    use serde_json::{from_str as json_decode, Value};
    use serde_json::ser::to_string;

    use super::*;

    #[test]
    fn decode_volume() {
        let val = r#"{
            "object_meta": {
                               "name":"",
                               "account":"876234567",
                              "labels":{},
                               "annotations":{},
                               "owner_references":[
                               {
                                   "kind":"Assembly",
                                   "api_version":"v1",
                                  "name":"lev.megam.io",
                                   "uid":"876543212345678",
                                  "block_owner_deletion":false
                              },
                              {
                                  "kind":"StoragePool",
                                  "api_version":"v1",
                                 "name":"private",
                                  "uid":"87654567876544567",
                                 "block_owner_deletion":false
                             }
                               ],
                               "created_at":"",
                               "deleted_at":"",
                               "deletion_grace_period_seconds":0,

                               "finalizers":[],
                               "cluster_name":""
                          },
                "status": {
                              "phase": "pending",
                              "message": "",
                              "reason": "",
                              "conditions": [
                              {
                             "message": "nodelet has sufficient disk space available",
                             "reason": "NodeletHasSufficientDisk",
                             "status": "False",
                             "last_transition_time": "2017-09-21T06:35:16Z",
                             "last_probe_time": "2017-09-21T06:35:16Z",
                             "condition_type": "OutOfDisk",
                             "last_update_time": "2017-09-21T06:35:16Z"
                           }
                         ]
                                  },
            "mount_path": "/var/lib/path",
            "allocated": "50 GiB"
                }"#;
        let vol: Volumes = json_decode(val).unwrap();
        assert_eq!(vol.mount_path, "/var/lib/path");
        assert_eq!(vol.allocated, "50 GiB");
    }
}
