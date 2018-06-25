// Copyright 2018 The Rio Advancement Inc
use api::base::{TypeMeta, ObjectMeta, MetaFields, Status};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Volumes {
    #[serde(default)]
    id: String, //id an unique identifier in systems of record. Generated during creation of the
    //Volumes
    pub object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Volumes
    mount_path: String,          //The mount path/pool name of the block device
    allocated: String,           //The size of the storage allocated.
    //Most recently observed status of the service. Populated by the system. Read-only.
    //Initially during submission, the status is "pending"
    status: Status,
    #[serde(default)]
    source: VolumeSource, // The contents of the target SettingMap's Data field will be presented in a
// volume as files using the keys in the Data field as the file names, unless
// the items element is populated with specific mappings of keys to paths.
// SettingMap volumes support ownership management and SELinux relabeling.

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

    pub fn set_source(&mut self, v: VolumeSource) {
        self.source = v;
    }

    pub fn get_source(&self) -> &VolumeSource {
        &self.source
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct VolumeSource {
    #[serde(default)]
    setting_map: SettingMap, // The name of the secret in the assembly's namespace to select from.
    #[serde(default)]
    nfs: Nfs, // NFS represents an NFS mount on the host that shares a assembly's lifetime
    #[serde(default)]
    openio:Openio,
    #[serde(default)]
    iscsi:Iscsi, // ISCSI represents an ISCSI Disk resource that is attached to a nodelet's host machine and then exposed to the assembly.
    #[serde(default)]
    rbd:Rbd // RBD represents a Rados Block Device mount on the host that shares a assembly's lifetime.
    #[serde(default)]
    host_path: HostPath,// HostPath represents a pre-existing file or directory on the host
	// machine that is directly exposed to the container. This is generally
	// used for system agents or other privileged things that are allowed
	// to see the host machine. Most containers will NOT need this.
    }

    // The contents of the target SettingMap's Data field will be presented in a
    // volume as files using the keys in the Data field as the file names, unless
    // the items element is populated with specific mappings of keys to paths.
    // SettingMap volumes support ownership management and SELinux relabeling.

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SettingMap {
    object_ref: ObjectReference, // The name of the secret in the assembly's namespace to select from.
    #[serde(default)]
    items: Vec<Items>, //If unspecified, each key-value pair in the Data field of the referenced
    // SettingMap will be projected into the volume as a file whose name is the
    // key and content is the value.
    #[serde(default)]
    default_mode: i32, //mode bits to use on created files by default. Must be a value between 0 and 0777. Defaults to 0644
    #[serde(default)]
    optional: bool, //Specify whether the SettingMap or it's keys must be defined

}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Nfs {
    #[serde(default)]
    server: String, // Server is the hostname or IP address of the NFS server.
    #[serde(default)]
    path: String, // Path that is exported by the NFS server.
    #[serde(default)]
    readonly: bool // ReadOnly here will force
	// the NFS export to be mounted with read-only permissions.
	// Defaults to false.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Openio {
    #[serde(default)]
    server: String, // // Server is the hostname or IP address of the NFS server.
    #[serde(default)]
    namespace: String,
    #[serde(default)]
    key: String,
    #[serde(default)]
    user: String
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Iscsi {
    #[serde(default)]
    target_portal: String, // iSCSI target portal. The portal is either an IP or ip_addr:port if the port  is other than default (typically TCP ports 860 and 3260).
    #[serde(default)]
    iqn: String, // Target iSCSI Qualified Name.
    #[serde(default)]
    lun: i32, // iSCSI target lun number.
    #[serde(default)]
    iscsi_interface: String, // Optional: Defaults to 'default' (tcp). iSCSI interface name that uses an iSCSI transport.
    #[serde(default)]
    fstype: String, // Filesystem type of the volume that you want to mount.  Tip: Ensure that the filesystem type is supported by the host operating system.  Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(default)]
    readonly: bool, // ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    #[serde(default)]
    portals: Vec<String>, // iSCSI target portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
    #[serde(default)]
    chap_auth_discovery: bool, // whether support iSCSI Discovery CHAP authentication
    #[serde(default)]
    chap_auth_session: bool, // whether support iSCSI Session CHAP authentication
    object_ref: ObjectReference, // CHAP secret for iSCSI target and initiator authentication
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Rbd {
    #[serde(default)]
    monitor: Vec<String>, // A collection of Ceph monitors.
    #[serde(default)]
    image: String, // The rados image name.
    #[serde(default)]
    fstype: String, // Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
    #[serde(default)]
    pool: String, // The rados pool name. Default is rbd.
    #[serde(default)]
    user: String, // The rados user name. Default is admin.
    #[serde(default)]
    keyring: String, // Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring.
    #[serde(default)]
    readonly: bool, // ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
    object_ref: ObjectReference, // SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil.

}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectReference {
    // Name of the referent.
	// More info: https://rioos.sh/docs/concepts/overview/working-with-objects/names/#names
	// TODO: Add other useful fields. api_version, kind, uid?
    name: String,
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HostPath {
    path: String, // Path of the directory on the host.
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Items {
#[serde(default)]
    key: String, // The key to project.
    // The relative path of the file to map the key to.
	// May not be an absolute path.
	// May not contain the path element '..'.
	// May not start with the string '..'.
#[serde(default)]
    path: String,
    //mode bits to use on this file, must be a value between 0
	// and 0777. If not specified, the volume defaultMode will be used.
	// This might be in conflict with other options that affect the file
	// mode, like fsGroup, and the result can be other mode bits set.
#[serde(default)]
    mode: i32,
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
