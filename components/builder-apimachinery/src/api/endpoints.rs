use api::base::{TypeMeta, ObjectMeta, MetaFields};
/*"Endpoints is a collection of endpoints that implement the actual service. Example:\n  Name: \"mysvc\",\n  Subsets: [\n    {\n      Addresses: [{\"ip\": \"10.10.1.1\"}, {\"ip\": \"10.10.2.2\"}],\n
 Ports: [{\"name\": \"a\", \"port\": 8675}, {\"name\": \"b\", \"port\": 309}]\n    },\n    {\n
 Addresses: [{\"ip\": \"10.10.3.3\"}],\n      Ports: [{\"name\": \"a\", \"port\": 93}, {\"name\": \"b\", \"port\": 76}]\n    },\n ]"*/
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct EndPoints {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta, //Standard type metadata: kind: EndPoints
    pub object_meta: ObjectMeta, //Standard object metadata
    /*"The set of all endpoints is the union of all subsets. Addresses are placed into subsets according to the IPs they share.
     A single address with multiple ports, some of which are ready and some of which are not (because they come from different containers/assembly) will result in the address being displayed in different subsets for the different ports.
      No address will appear in both Addresses and NotReadyAddresses in the same subset. Sets of addresses and ports that comprise a service."*/
    pub subsets: Subsets,
    #[serde(default)]
    created_at: String,
}

impl EndPoints {
    pub fn new() -> EndPoints {
        ::std::default::Default::default()
    }

    //Create a new endpoints with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> EndPoints {
        EndPoints {
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

    pub fn set_subsets(&mut self, v: Subsets) {
        self.subsets = v;
    }

    pub fn get_subsets(&self) -> &Subsets {
        &self.subsets
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

/*"EndpointSubset is a group of addresses with a common set of ports.
 The expanded set of endpoints is the Cartesian product of Addresses x Ports.
 For example, given:\n  {\n    Addresses: [{\"ip\": \"10.10.1.1\"}, {\"ip\": \"10.10.2.2\"}],\n    Ports:     [{\"name\": \"a\", \"port\": 8675}, {\"name\": \"b\", \"port\": 309}]\n  }\nThe resulting set of endpoints can be viewed as:\n    a: [ 10.10.1.1:8675, 10.10.2.2:8675 ],\n    b: [ 10.10.1.1:309, 10.10.2.2:309 ]"*/
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Subsets {
    addresses: Vec<Addesses>, //IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.
    unready_addresses: Vec<Addesses>, //IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check.
    ports: Vec<Ports>, //Port numbers available on the related IP addresses.
}

impl Subsets {
    pub fn new(addresses: Vec<Addesses>, unready_addresses: Vec<Addesses>, ports: Vec<Ports>) -> Subsets {
        Subsets {
            addresses: addresses,
            unready_addresses: unready_addresses,
            ports: ports,
        }
    }
    pub fn is_empty(&self) -> bool {
        return self.addresses.is_empty() && self.unready_addresses.is_empty() && self.ports.is_empty();
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Addesses {
    name: String, //The must bea network service name (private_ipv4, public_ip4, private_ipv6, public_ipv6) for the addresses.
    protocol_version: String, //Protocol for port. Must be UDP or TCP or HTTP.
    ip: String, //What IP to bind the external port to.
    mac_address: String, //mac address of the endpoint
}

impl Addesses {
    pub fn new(name: &str, protocol_version: &str, ip: &str, mac_address: &str) -> Addesses {
        Addesses {
            name: name.to_string(),
            protocol_version: protocol_version.to_string(),
            ip: ip.to_string(),
            mac_address: mac_address.to_string(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Ports {
    name: String,
    port: String,
    protocol: String,
}

impl Ports {
    pub fn new(name: &str, port: &str, protocol: &str) -> Ports {
        Ports {
            name: name.to_string(),
            port: port.to_string(),
            protocol: protocol.to_string(),
        }
    }
}

impl MetaFields for EndPoints {
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
    fn decode_service_port() {
        let val = r#"{
        "name":"http",
        "port" :"8080",
        "protocol": "http"
}"#;
        let ports: Ports = json_decode(val).unwrap();
        assert_eq!(ports.name, "http");
        assert_eq!(ports.port, "8080");
        assert_eq!(ports.protocol, "http");
    }

    #[test]
    fn decode_service_address() {
        let val = r#"{
        "name":"private",
        "protocol_version" :"ipv4",
        "ip": "192.168.1.11",
        "mac_address": "00:0a:95:9d:68:16"
}"#;
        let addr: Addesses = json_decode(val).unwrap();
        assert_eq!(addr.name, "private");
        assert_eq!(addr.protocol_version, "ipv4");
        assert_eq!(addr.ip, "192.168.1.11");
        assert_eq!(addr.mac_address, "00:0a:95:9d:68:16");
    }
    #[test]
    fn decode_endpoints_subsets() {
        let val = r#"{
        "addresses":[{"name": "private","protocol_version": "ipv4","ip": "192.168.1.10","mac_address": "00:0a:95:9d:68:16"}],
        "unready_addresses" :[{"name": "private", "protocol_version": "ipv4", "ip": "192.168.1.11","mac_address": ""}],
        "ports": [{ "name": "", "port": "","protocol":"tcp"}]
}"#;
        let addr: Subsets = json_decode(val).unwrap();
        assert_eq!(addr.addresses.len(), 1);
        assert_eq!(addr.unready_addresses.len(), 1);
        assert_eq!(addr.ports.len(), 1);
    }
}
