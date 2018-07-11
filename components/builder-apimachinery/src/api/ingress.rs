// Copyright 2018 The Rio Advancement Inc

use api::base::{MetaFields, ObjectMeta, TypeMeta};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Ingress {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the job
    #[serde(default)]
    type_meta: TypeMeta,
    // Standard object's metadata.
    object_meta: ObjectMeta,
    // Spec is the desired state of the Ingress.
    spec: IngressSpec,
    // Status is the current state of the Ingress.
    #[serde(default)]
    status: LoadBalancerStatus,
    #[serde(default)]
    created_at: String,
}

impl Ingress {
    pub fn new() -> Ingress {
        ::std::default::Default::default()
    }

    //Create a new Ingress with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Ingress {
        Ingress {
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

    pub fn set_spec(&mut self, v: IngressSpec) {
        self.spec = v;
    }
    pub fn get_spec(&self) -> &IngressSpec {
        &self.spec
    }
    pub fn set_status(&mut self, v: LoadBalancerStatus) {
        self.status = v;
    }
    pub fn get_status(&self) -> &LoadBalancerStatus {
        &self.status
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for Ingress {
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
pub struct IngressSpec {
    // A default backend capable of servicing requests that don't match any
	// rule. At least one of 'backend' or 'rules' must be specified. This field
	// is optional to allow the loadbalancer controller or defaulting logic to
	// specify a global default.
	#[serde(default)]
    backend: IngressBackend,
    // TLS configuration. Currently the Ingress only supports a single TLS
	// port, 443. If multiple members of this list specify different hosts, they
	// will be multiplexed on the same port according to the hostname specified
	// through the SNI TLS extension, if the ingress controller fulfilling the
	// ingress supports SNI.
    #[serde(default)]
    tls: Vec<IngressTLS>,
    // A list of host rules used to configure the Ingress. If unspecified, or
	// no rule matches, all traffic is sent to the default backend.
	#[serde(default)]
    rules: Vec<IngressRule>,
}
// IngressBackend describes all endpoints for a given service and port.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct IngressBackend {
    service_port: i32, // Specifies the port of the referenced service.
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct IngressTLS {
    // Hosts are a list of hosts included in the TLS certificate. The values in
	// this list must match the name/s used in the tlsSecret. Defaults to the
	// wildcard host setting for the loadbalancer controller fulfilling this
	// Ingress, if left unspecified.
	#[serde(default)]
    hosts: Vec<String>,
    // SecretName is the name of the secret used to terminate SSL traffic on 443.
	// Field is left optional to allow SSL routing based on SNI hostname alone.
	// If the SNI host in a listener conflicts with the "Host" header field used
	// by an IngressRule, the SNI host is used for termination and value of the
	// Host header is used for routing.
    #[serde(default)]
    secret_id: String,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct IngressRule {
    // Host is the fully qualified domain name of a network host, as defined
	// by RFC 3986. Note the following deviations from the "host" part of the
	// URI as defined in the RFC:
	// 1. IPs are not allowed. Currently an IngressRuleValue can only apply to the
	//	  IP in the Spec of the parent Ingress.
	// 2. The `:` delimiter is not respected because ports are not allowed.
	//	  Currently the port of an Ingress is implicitly :80 for http and
	//	  :443 for https.
	// Both these may change in the future.
	// Incoming requests are matched against the host before the IngressRuleValue.
	// If the host is unspecified, the Ingress routes all traffic based on the
	// specified IngressRuleValue.
	#[serde(default)]
    host: String,
    // IngressRuleValue represents a rule to route requests for this IngressRule.
	// If unspecified, the rule defaults to a http catch-all. Whether that sends
	// just traffic matching the host to the default backend or all traffic to the
	// default backend, is left to the controller fulfilling the Ingress. Http is
	// currently the only supported IngressRuleValue.
    #[serde(default)]
    ingress_rule_value: IngressRuleValue,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct IngressRuleValue {
    // 1. Consider renaming this resource and the associated rules so they
	// aren't tied to Ingress. They can be used to route intra-cluster traffic.
	// 2. Consider adding fields for ingress-type specific global options
	// usable by a loadbalancer, like http keep-alive.

	#[serde(default)]
    http: HTTPIngressRuleValue,
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HTTPIngressRuleValue {
    // A collection of paths that map requests to backends.
    // TODO: Consider adding fields for ingress-type specific global
	// options usable by a loadbalancer, like http keep-alive.
	paths: Vec<HTTPIngressPath>
}

// HTTPIngressPath associates a path regex with a backend. Incoming urls matching
// the path are forwarded to the backend.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HTTPIngressPath {
	// Path is an extended POSIX regex as defined by IEEE Std 1003.1,
	// (i.e this follows the egrep/unix syntax, not the perl syntax)
	// matched against the path of an incoming request. Currently it can
	// contain characters disallowed from the conventional "path"
	// part of a URL as defined by RFC 3986. Paths must begin with
	// a '/'. If unspecified, the path defaults to a catch all sending
	// traffic to the backend.
	#[serde(default)]
	path: String,
	// Backend defines the referenced service endpoint to which the traffic
	// will be forwarded to.
	backend: IngressBackend,
}

// LoadBalancerStatus represents the status of a load-balancer.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct LoadBalancerStatus {
    // Ingress is a list containing ingress points for the load-balancer.
    // Traffic intended for the service should be sent to these ingress points.
	#[serde(default)]
	ingress: Vec<LoadBalancerIngress>,
}

// LoadBalancerIngress represents the status of a load-balancer ingress point:
// traffic intended for the service should be sent to an ingress point.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct LoadBalancerIngress {
    // IP is set for load-balancer ingress points that are IP based
    // (typically GCE or OpenStack load-balancers)
    #[serde(default)]
    ip: String,
    // Hostname is set for load-balancer ingress points that are DNS based
    // (typically AWS load-balancers)
    #[serde(default)]
    hostname: String,
}

///The status that is used to parse request in /status update of any api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusUpdate {
    pub status: LoadBalancerStatus,
    #[serde(default)]
    id: String,
}

impl StatusUpdate {
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_status(&mut self, v: LoadBalancerStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> &LoadBalancerStatus {
        &self.status
    }
}
