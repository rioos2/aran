// Copyright 2018 The Rio Advancement Inc
use api::base::{MetaFields, ObjectMeta, Status, TypeMeta, WhoAmITypeMeta};
use std::collections::BTreeMap;

pub const RIOOS_ASSEMBLY_FACTORY_ID: &'static str = "rioos_assembly_factory_id";

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Services {
    #[serde(default)]
    id: String,
    spec: Spec,
    status: Status,
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    #[serde(default)]
    created_at: String,
}

impl Services {
    pub fn new() -> Services {
        ::std::default::Default::default()
    }

    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Services {
        Services {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }

    pub fn with_type(&mut self, service_type: String) {
        self.get_mut_spec().service_type = service_type;
    }

    pub fn with_type_names(&mut self, service_type: String, names: BTreeMap<String, String>) {
        self.get_mut_spec().service_type = service_type;
        self.get_mut_spec().names = names;
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }
    pub fn get_mut_spec(&mut self) -> &mut Spec {
        &mut self.spec
    }
    pub fn get_spec(&self) -> &Spec {
        &self.spec
    }
    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }
    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
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
impl MetaFields for Services {
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

impl WhoAmITypeMeta for Services {
    const MY_KIND: &'static str = "POST:services";
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Spec {
    /*Only applies to Service Type: LoadBalancer LoadBalancer will get created with the IP specified in this field.type determines how the Service is exposed.
     Valid options are ExternalName and LoadBalancer. \"ExternalName\" maps to the specified externalName. \"LoadBalancer\" creates an external load-balancer (if supported in Rio/OS) which routes to the ExternalName
     Only applies to Service Type: LoadBalancer LoadBalancer will get created with the IP specified in this field.*/
    service_type: String,
    #[serde(default)]
    loadbalancer_ip: String, //The IP address created and attached to access the loadbalancer.
    #[serde(default)]
    names: BTreeMap<String, String>, //name(internal) is the reference that internal powerdns will return as a CNAME record for this service. No proxying will be involved. Must be a valid RFC-1123 hostname (https://tools.ietf.org/html/rfc1123) and requires Type to be ExternalName
    #[serde(default)]
    external_names: BTreeMap<String, String>, //ExternalName maps to a public dns . Rio/OS automatically figures out the ip address is public or private.
}

impl Spec {
    pub fn new(
        service_type: &str,
        loadbalancer_ip: &str,
        names: BTreeMap<String, String>,
        external_names: BTreeMap<String, String>,
    ) -> Spec {
        Spec {
            service_type: service_type.to_string(),
            loadbalancer_ip: loadbalancer_ip.to_string(),
            names: names,
            external_names: external_names,
        }
    }

    pub fn get_service_type(&self) -> ::std::string::String {
        self.service_type.clone()
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_service_spec() {
        let val = r#"{
            "service_type" :"LoadBalancer",
            "loadbalancer_ip": "192.168.1.11",
            "names":{"private_name":"levis-01.megam.io"},
            "external_names": {"public_name":"levis-01.megam.io"}
    }"#;
        let spec: Spec = json_decode(val).unwrap();
        assert_eq!(spec.service_type, "LoadBalancer");
        assert_eq!(spec.names.len(), 1);
        assert!(spec.names.contains_key("private_name"));
        assert_eq!(spec.external_names.len(), 1);
        assert!(spec.external_names.contains_key("public_name"));
    }

    #[test]
    fn decode_service() {
        let val = r#"{
            "type_meta":{"kind": "Service", "api_version": "V1"},
            "object_meta":{"name":"service1","account":"","labels":{},"annotations":{},"owner_references":[{"kind":"","api_version":"","name":"","uid":"","block_owner_deletion":false}],"created_at":"","deleted_at":"","deletion_grace_period_seconds":30,"finalizers":[],"cluster_name":""},
            "status":{"phase":"","message":"","reason":"","conditions":[{"message":"", "reason":"","status":"ready","last_transition_time":"","last_probe_time":"","condition_type":"","last_update_time": ""}]},
            "spec": {
                "service_type" :"LoadBalancer",
                "loadbalancer_ip": "192.168.1.11",
                "names":{"private_name":"levis-01.megam.io"},
                "external_names": {"public_name":"levis-01.megam.io"}
            }
    }"#;
        let service: Services = json_decode(val).unwrap();
        assert_eq!(service.id, "");
        assert_eq!(service.created_at, "");
    }

}
