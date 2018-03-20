// Copyright 2018 The Rio Advancement Inc
use std::collections::BTreeMap;
use api::base::{TypeMeta, ObjectMeta, Status, MetaFields};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Network {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the network
    network_type: String, //Different types of networks are distinguished based on their data transfer speed, and their reach. The supported values are `private_ipv4`, `private_ipv6`, `public_ipv4`, `public_ipv6`
    /*A local subdivision of the network in Classless Inter-Domain Routing (CIDR) notation written as the first address of a network, followed by a slash character (/), and ending with the bit-length of the prefix.
    For example, 192.168.1.0/24 is the prefix of the Internet Protocol version 4 network starting at the given address, having 24 bits allocated for the network prefix, and the remaining 8 bits reserved for host addressing. The IPv6 address specification 2001:db8::/32 is a large address block with 296 addresses.*/
    subnet_ip: String,
    netmask: String, //Indicates the netmask  a range of IP addresses that can be used by this subnet. Example: 255.255.255.0 for IPV4
    gateway: String, //Indicates the ipaddress of the gateways which connect networks so the devices on them can communicate
    bridge_hosts: BTreeMap<String, String>, // host_ip which applicable bridge for this network {"192.168.2.0":"riopri"}
    status: Status, //Most recently observed status of the service. Populated by the system. Read-only.  Initially during submission, the status is "pending"
    #[serde(default)]
    used_bits: Vec<i16>, // list of allocated range ip index
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Network
    #[serde(default)]
    created_at: String,
}
impl Network {
    pub fn new() -> Network {
        ::std::default::Default::default()
    }
    //Create a new network with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Network {
        Network {
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

    pub fn set_subnet_ip(&mut self, v: ::std::string::String) {
        self.subnet_ip = v;
    }
    pub fn get_subnet_ip(&self) -> ::std::string::String {
        self.subnet_ip.clone()
    }

    pub fn set_network_type(&mut self, v: ::std::string::String) {
        self.network_type = v;
    }
    pub fn get_network_type(&self) -> ::std::string::String {
        self.network_type.clone()
    }

    pub fn set_netmask(&mut self, v: ::std::string::String) {
        self.netmask = v;
    }
    pub fn get_netmask(&self) -> ::std::string::String {
        self.netmask.clone()
    }

    pub fn set_gateway(&mut self, v: ::std::string::String) {
        self.gateway = v;
    }
    pub fn get_gateway(&self) -> ::std::string::String {
        self.gateway.clone()
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

    pub fn set_bridge_hosts(&mut self, v: BTreeMap<String, String>) {
        self.bridge_hosts = v;
    }

    pub fn get_bridge_hosts(&self) -> &BTreeMap<String, String> {
        &self.bridge_hosts
    }

    pub fn set_used_bits(&mut self, v: ::std::vec::Vec<i16>) {
        self.used_bits = v;
    }

    pub fn get_used_bits(&self) -> ::std::vec::Vec<i16> {
        self.used_bits.clone()
    }
}
impl MetaFields for Network {
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
    use serde_json::from_str as json_decode;

    use super::*;
    #[test]
    fn decode_network() {
        let val = r#"
    {
        "network_type": "private_ipv4",
        "subnet_ip": "192.168.1.0/24",
        "netmask": "255.255.255.0",
        "gateway": "192.168.1.1",
        "status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},
        "bridge_hosts": {"192.168.1.47":"riopub4", "192.168.1.48":"riopriv4"},
        "object_meta":{"name":"private","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""}
    }"#;
        let net: Network = json_decode(val).unwrap();
        assert_eq!(net.network_type, "private_ipv4");
        assert_eq!(net.subnet_ip, "192.168.1.0/24");
        assert_eq!(net.netmask, "255.255.255.0");
        assert_eq!(net.gateway, "192.168.1.1");
        assert_eq!(net.bridge_hosts.len(), 2);
        assert!(net.bridge_hosts.contains_key("192.168.1.47"));
        assert!(net.bridge_hosts.contains_key("192.168.1.48"));
    }
}
