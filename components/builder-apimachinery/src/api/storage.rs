// Copyright 2018 The Rio Advancement Inc
use api::base::{TypeMeta, ObjectMeta, Status, MetaFields};
use api::node::NodeInfo;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Storage {
    #[serde(default)]
    id: String, // Id an unique identifier in systems of record. Generated during creation of the AssemblyFactory
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Storage
    host_ip: String, //ip of the server
    storage_type: String, //type of the storage server
    storage_info: Disks, //disk detail for the storage
    parameters: BTreeMap<String, String>,
    status: Status, //Most recently observed status of the service. Populated by the system. Read-only.  Initially during submission, the status is "pending"
    node_info: NodeInfo, //Set of ids/uuids to uniquely identify the node.
    #[serde(default)]
    created_at: String,
}
impl Storage {
    pub fn new() -> Storage {
        ::std::default::Default::default()
    }
    //Create a new storage with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Storage {
        Storage {
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

    pub fn set_host_ip(&mut self, v: ::std::string::String) {
        self.host_ip = v;
    }
    pub fn get_host_ip(&self) -> ::std::string::String {
        self.host_ip.clone()
    }

    pub fn set_storage_type(&mut self, v: ::std::string::String) {
        self.storage_type = v;
    }
    pub fn get_storage_type(&self) -> ::std::string::String {
        self.storage_type.clone()
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_storage_info(&mut self, v: Disks) {
        self.storage_info = v;
    }

    pub fn get_storage_info(&self) -> &Disks {
        &self.storage_info
    }

    //Convert it using Into or From<String>
    pub fn get_disks_str(&self) -> String {
        self.get_storage_info().disks.iter().fold(
            "".to_string(),
            |acc, ref d| {
                format!("{}{} → {}{}", acc,d.disk,d.size,"\n")
            },
        )
    }

    pub fn set_paramaters(&mut self, v: BTreeMap<String, String>) {
        self.parameters = v;
    }

    pub fn get_parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }
    pub fn set_node_info(&mut self, v: NodeInfo) {
        self.node_info = v;
    }

    pub fn get_node_info(&self) -> &NodeInfo {
        &self.node_info
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}
impl MetaFields for Storage {
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
pub struct Disks {
    disks: Vec<Disk>,
}

impl Disks {
    pub fn new(disks: Vec<Disk>) -> Disks {
        Disks { disks: disks }
    }
    pub fn get_disks(&self) -> Vec<Disk> {
        self.disks.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Disk {
    disk: String, //name of the disk
    disk_type: String, //type of the disk
    point: String, //mount path of the disk
    size: String, //total size of the disk
    used_size: String, // used size of the disk
}

impl Disk {
    pub fn new(disk: &str, disk_type: &str, point: &str, size: &str, used_size: &str) -> Disk {
        Disk {
            disk: disk.to_string(),
            disk_type: disk_type.to_string(),
            point: point.to_string(),
            size: size.to_string(),
            used_size: used_size.to_string(),
        }
    }
    pub fn get_size(&self) -> ::std::string::String {
        self.size.clone()
    }
    // pub fn get_used_size(&self) -> ::std::string::String {
    //     self.used_size.clone()
    // }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DataCenter {
    #[serde(default)]
    id: String,
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Datacenter
    nodes: Vec<String>, //list of nodes to the region
    networks: Vec<String>, //list of networks ,which network to support the datacenter
    enabled: bool, //used to disable the datacenter when time of may be node ar network failure
    storage: String, //which storage type to support the datacenter creation
    advanced_settings: BTreeMap<String, String>, //add some additional features for the datacenter
    flag: String, //describe the which place that datacenter located, provide that country flag Example:india.png
    currency: String, //type of the currency that country support
    status: Status, // //Most recently observed status of the service. Populated by the system. Read-only.  Initially during submission, the status is "pending"
    #[serde(default)]
    created_at: String,
}
impl DataCenter {
    pub fn new() -> DataCenter {
        ::std::default::Default::default()
    }
    //Create a new datacenter with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> DataCenter {
        DataCenter {
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

    pub fn set_flag(&mut self, v: ::std::string::String) {
        self.flag = v;
    }
    pub fn get_flag(&self) -> ::std::string::String {
        self.flag.clone()
    }

    pub fn set_currency(&mut self, v: ::std::string::String) {
        self.currency = v;
    }
    pub fn get_currency(&self) -> ::std::string::String {
        self.currency.clone()
    }

    pub fn set_networks(&mut self, v: ::std::vec::Vec<String>) {
        self.networks = v;
    }
    pub fn get_networks(&self) -> ::std::vec::Vec<String> {
        self.networks.clone()
    }

    pub fn set_nodes(&mut self, v: ::std::vec::Vec<String>) {
        self.nodes = v;
    }
    pub fn get_nodes(&self) -> ::std::vec::Vec<String> {
        self.nodes.clone()
    }

    pub fn set_storage(&mut self, v: ::std::string::String) {
        self.storage = v;
    }
    pub fn get_storage(&self) -> ::std::string::String {
        self.storage.clone()
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_advanced_settings(&mut self, v: BTreeMap<String, String>) {
        self.advanced_settings = v;
    }

    pub fn get_advanced_settings(&self) -> &BTreeMap<String, String> {
        &self.advanced_settings
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = v;
    }
    pub fn get_enabled(&self) -> bool {
        self.enabled.clone()
    }
}

impl MetaFields for DataCenter {
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
pub struct StoragePool {
    #[serde(default)]
    id: String, // Id an unique identifier in systems of record. Generated during creation of the StoargePool
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind:StoragePool
    connector_id: String, //id that refer the where is that storage pool locat
    storage_info: Disks,
    #[serde(default)]
    parameters: BTreeMap<String, String>, //Parameters holds the parameters for the provisioner that should,create volumes of this storage class.
    status: Status,
    #[serde(default)]
    remote_storage_disks: BTreeMap<String, Vec<String>>, //collection of storage connector disks
    #[serde(default)]
    created_at: String,
}
impl StoragePool {
    pub fn new() -> StoragePool {
        ::std::default::Default::default()
    }
    //Create a new storagepool with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> StoragePool {
        StoragePool {
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

    pub fn set_connector_id(&mut self, v: ::std::string::String) {
        self.connector_id = v;
    }
    pub fn get_connector_id(&self) -> ::std::string::String {
        self.connector_id.clone()
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_storage_info(&mut self, v: Disks) {
        self.storage_info = v;
    }

    pub fn get_storage_info(&self) -> &Disks {
        &self.storage_info
    }
    //Convert it using Into or From<String>
    pub fn get_disks_str(&self) -> String {
        self.get_storage_info().disks.iter().fold(
            "".to_string(),
            |acc, ref d| {
                format!("{}{} → {}{}", acc,d.disk,d.size,"\n")
            },
        )
    }

    pub fn set_paramaters(&mut self, v: BTreeMap<String, String>) {
        self.parameters = v;
    }

    pub fn get_parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }

    pub fn set_remote_storage_disks(&mut self, v: BTreeMap<String, Vec<String>>) {
        self.remote_storage_disks = v;
    }

    pub fn get_remote_storage_disks(&self) -> &BTreeMap<String, Vec<String>> {
        &self.remote_storage_disks
    }


    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for StoragePool {
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
    use serde_json::{from_str as json_decode};

    use super::*;
    #[test]
    fn decode_storage() {
        let val = r#"
        {
        "host_ip": "172.168.1.1",
        "storage_type":"iscsi",
        "status":{
            "message":"",
            "reason":"",
            "phase": "pending",
            "conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]
        },
        "storage_info": {
            "disks": [
            {
                "disk": "/dev/sdb",
                "disk_type": "/dev/sdb1",
                "point": "/home",
                "size": "50GB",
                "used_size": "10GB"
            },
            {
                "disk": "/dev/sdb1",
                "disk_type": "/dev/sdb2",
                "point": "/home/ranji",
                "size": "500GB",
                "used_size": "100GB"
            }]},
            "node_info":{
                "machine_id": "589f17c8cc084c078c5d364241433afc",
                "system_uuid": "85EE9345-A1AF-11E3-BE7C-28E347559DE7",
                "kernel_version": "4.4.0-93-generic",
                "os_image": "Ubuntu 16.04.3 LTS",
                "architecture": "amd64"
            },
        "parameters":{"pool_name": "iscsi-pool","user_id": "iscsi-user","password": "iscsi-password"},
        "object_meta":{"name":"iscsi","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}
    }"#;
        let storage: Storage = json_decode(val).unwrap();
        assert_eq!(storage.host_ip, "172.168.1.1");
        assert_eq!(storage.storage_type, "iscsi");
        assert_eq!(storage.parameters.len(), 3);
        assert!(storage.parameters.contains_key("pool_name"));
        assert!(storage.parameters.contains_key("user_id"));
        assert!(storage.parameters.contains_key("password"));
    }

    #[test]
    fn decode_datcenter() {
        let val = r#"
        {
        "nodes": ["844747261714907136"],
        "networks":["844751056645668864"],
        "enabled": true,
        "storage": "87654345678",
        "advanced_settings":{},
        "flag":"ch.png",
        "currency": "rs",
        "status":{
            "message":"",
            "reason":"",
            "phase": "pending",
            "conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]
        },
        "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],"cluster_name":""}
    }"#;
        let datacenter: DataCenter = json_decode(val).unwrap();
        assert_eq!(datacenter.nodes.len(), 1);
        assert_eq!(datacenter.networks.len(), 1);
        assert_eq!(datacenter.enabled, true);
        assert_eq!(datacenter.storage, "87654345678");
        assert_eq!(datacenter.flag, "ch.png");
        assert_eq!(datacenter.currency, "rs");
    }

    #[test]
    fn decode_storage_pool() {
        let val = r#"
        {
        "connector_id": "87654345678",
        "parameters":{},
        "storage_info": {
            "disks": [
            {
                "disk": "/dev/sdb",
                "disk_type": "/dev/sdb1",
                "point": "/home",
                "size": "50GB",
                "used_size": "10GB"
            },
            {
                "disk": "/dev/sdb1",
                "disk_type": "/dev/sdb2",
                "point": "/home/ranji",
                "size": "500GB",
                "used_size":"100GB"
            }]},
        "status":{
            "message":"",
            "reason":"",
            "phase": "pending",
            "conditions": [{"message":"nodelet has sufficient disk space available","reason":"NodeletHasSufficientDisk","status":"False","last_transition_time":"2017-09-21T06:35:16Z","last_probe_time":"2017-09-21T06:35:16Z","condition_type":"OutOfDisk","last_update_time": ""}]
        },
        "object_meta":{"name":"chennai","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30, "finalizers":[],"cluster_name":""}
    }"#;
        let pool: StoragePool = json_decode(val).unwrap();
        assert_eq!(pool.connector_id, "87654345678");
    }
}
