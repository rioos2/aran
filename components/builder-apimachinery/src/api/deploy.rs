// Copyright 2018 The Rio Advancement Inc
use std::collections::BTreeMap;

use api::base::{TypeMeta, ObjectMeta, IdGet, Status, MetaFields, ChildTypeMeta};
use api::blueprint::Plan;
use api::volume::Volumes;
use api::endpoints::EndPoints;
use api::linker::Services;


use cache::inject::{PlanFeeder, FactoryFeeder, EndPointsFeeder, VolumeFeeder, MetricFeeder, ServicesFeeder};

pub const PHASE_PENDING: &'static str = "Pending";
pub const PHASE_STAND_STILL: &'static str = "StandStill";

pub const NEW_REPLICA_INITALIZING_MSG: &'static str = "Initializing replicas...Brew some coffee!!!";
pub const NEW_STAND_STILL_MSG: &'static str = "I'm in sleep state. ...Wake me up!!!";


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactory {
    #[serde(default)]
    id: String, // Id an unique identifier in systems of record. Generated during creation of the AssemblyFactory
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: AssemblyFactory
    object_meta: ObjectMeta, ////Standard object metadata
    replicas: u32, //Replicas is the number of desired replicas of the plan.
    resources: BTreeMap<String, String>, //cpu, ram, disk, compute: cpu/gpu, storage: hdd/ssd
    secret: Secret, //Secret references to the secret for user and other sensitive information. If this is not provided, Login operation will fail.
    plan: String, // A Plan is meta-data that provides a description of the artifacts that make up an application, the services that are required to execute or utilize those artifacts, and the relationship of the artifacts to those services. Plans are expressed as json under a /plans resource.    Here we provide the identifier as pointed to /plans
    #[serde(default)]
    status: Status, //Most recently observed status of the service. Populated by the system. Read-only.  Initially during submission, the status is "pending"
    #[serde(default)]
    spec: AssemblyFactorySpec,
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    created_at: String,
}

impl AssemblyFactory {
    pub fn new() -> AssemblyFactory {
        ::std::default::Default::default()
    }

    //Create a new assemblyfactory with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> AssemblyFactory {
        AssemblyFactory {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_replicas(&mut self, v: u32) {
        self.replicas = v;
    }

    pub fn get_replicas(&self) -> u32 {
        self.replicas
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
    pub fn set_plan(&mut self, v: ::std::string::String) {
        self.plan = v;
    }

    pub fn get_plan(&self) -> ::std::string::String {
        self.plan.clone()
    }

    pub fn set_secret(&mut self, v: Secret) {
        self.secret = v;
    }

    pub fn get_secret(&self) -> &Secret {
        &self.secret
    }

    pub fn set_resources(&mut self, v: BTreeMap<String, String>) {
        self.resources = v;
    }

    pub fn get_resources(&self) -> &BTreeMap<String, String> {
        &self.resources
    }

    pub fn set_spec(&mut self, v: AssemblyFactorySpec) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> &AssemblyFactorySpec {
        &self.spec
    }

    // Mutable pointer to the field spec.
    pub fn mut_spec(&mut self) -> &mut AssemblyFactorySpec {
        &mut self.spec
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

// Cache based feeders for the base AssemblyFactory
//           AssemblyFactory
//              |
//             Plan
//
// The plan feeder, which get a callback from an expander cache.
// The expander cache is ttl and loads the plan the first time.
impl PlanFeeder for AssemblyFactory {
    fn pget_id(&mut self) -> IdGet {
        IdGet::with_id(self.get_plan().clone())
    }

    fn pfeed(&mut self, p: Option<Plan>) {
        self.mut_spec().set_plan(p);
    }
}


// The service feeder, which gets called from an expander cache.
// The expander cache is ttl and loads the service the first time.
impl ServicesFeeder for AssemblyFactory {
    fn sget_id(&mut self) -> IdGet {
        IdGet::with_id(self.get_id().clone())
    }

    fn sfeed(&mut self, s: Option<Vec<Services>>) {
        self.mut_spec().set_services(s);
    }
}

impl MetaFields for AssemblyFactory {
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

impl ChildTypeMeta for AssemblyFactory {
    const CHILD_KIND: &'static str = "POST:assemblys";
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Assembly {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the Assembly
    #[serde(default)]
    type_meta: TypeMeta, //Standard type metadata: kind: Assembly
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    selector: Vec<String>, // selector to restrict the list of returned objects by their labels. Defaults to everything
    status: Status, //Most recently observed status of the service. Populated by the system. Read-only.  Initially during submission, the status is "pending"
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    spec: Spec,
    #[serde(default)]
    created_at: String,
}

impl Assembly {
    pub fn new() -> Assembly {
        ::std::default::Default::default()
    }

    //Create a new assembly with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Assembly {
        Assembly {
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

    pub fn set_selector(&mut self, v: ::std::vec::Vec<String>) {
        self.selector = v;
    }

    pub fn get_selector(&self) -> ::std::vec::Vec<String> {
        self.selector.clone()
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

    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> Spec {
        self.spec.clone()
    }

    // Mutable pointer to the field spec.
    pub fn mut_spec(&mut self) -> &mut Spec {
        &mut self.spec
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

impl MetaFields for Assembly {
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

// Cache based feeders for the base Assembly
//           Assembly
//              |
//  AssemblyFactory (Parent), Endpoints, Volume, Metrics
//
// The assemblyfactory feeder, which gets called from an expander cache.
// The expander cache is ttl and loads the factory the first time.
impl FactoryFeeder for Assembly {
    fn fget_id(&mut self) -> IdGet {
        IdGet::with_id_name(
            self.get_owner_references()
                .iter()
                .map(|x| x.get_uid().to_string())
                .collect::<String>(),
            "_factory".to_string(),
        )
    }

    fn ffeed(&mut self, f: Option<AssemblyFactory>) {
        self.mut_spec().set_parent(f);
    }
}

// The endpoints feeder, which gets called from an expander cache.
// The expander cache is ttl and loads the endpoints the first time.
impl EndPointsFeeder for Assembly {
    fn eget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_id(), "_endpoint".to_string())
    }

    fn efeed(&mut self, e: Option<EndPoints>) {
        self.mut_spec().set_endpoints(e);
    }
}



/// The volume feeder, which gets called from an expander cache.
/// The expander cache is ttl and loads the volume for the assembly the first time.
impl VolumeFeeder for Assembly {
    fn vget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_id(), "_volume".to_string())
    }

    fn vfeed(&mut self, v: Option<Vec<Volumes>>) {
        self.mut_spec().set_mut_volumes(v);
    }
}

impl MetricFeeder for Assembly {
    fn mget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_id(), "_metricsingle".to_string())
    }

    fn mfeed(&mut self, m: Option<BTreeMap<String, String>>) {
        self.mut_spec().set_metrics(m);
    }
}

// The metrics live feeder, which gets called from an expander cache.
// The metrics is live, for now, we don't store it in the expander cache
// Convienient way of getting it metrics via the expander
/*impl MetricsLiveFeeder for Assembly {
    fn get_id() -> IdGet {
        &IdGet::with_id(self.get_id())
    }

    fn feed(&self, m: BTreeMap<String, String>) {
        self.set_metrics(e)
    }
}*/

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Secret {
    pub id: String,
}
impl Secret {
    pub fn with_secrets(id: &str) -> Secret {
        Secret { id: id.to_string() }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Spec {
    assembly_factory: Option<AssemblyFactory>,

    endpoints: Option<EndPoints>,

    volumes: Option<Vec<Volumes>>,

    metrics: Option<BTreeMap<String, String>>,
}

impl Spec {
    pub fn new() -> Spec {
        ::std::default::Default::default()
    }
    pub fn set_parent(&mut self, factory: Option<AssemblyFactory>) {
        self.assembly_factory = factory;
    }

    pub fn get_parent(&self) -> Option<AssemblyFactory> {
        self.assembly_factory.clone()
    }

    pub fn set_endpoints(&mut self, endpoints: Option<EndPoints>) {
        self.endpoints = endpoints;
    }

    pub fn get_endpoints(&self) -> Option<EndPoints> {
        self.endpoints.clone()
    }

    /*pub fn set_volumes(&mut self, volumes: Option<Volumes>) {
        self.volumes = volumes;
    }*/

    pub fn set_mut_volumes(&mut self, volumes: Option<Vec<Volumes>>) {
        self.volumes = volumes;
    }

    pub fn get_volumes(&self) -> Option<Vec<Volumes>> {
        self.volumes.clone()
    }

    pub fn set_metrics(&mut self, v: Option<BTreeMap<String, String>>) {
        self.metrics = v;
    }

    pub fn get_metrics(&self) -> Option<BTreeMap<String, String>> {
        self.metrics.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactorySpec {
    /* Tolerations:
    A toleration “matches” a taint if the keys are the same and the effects are the same, and:
    the operator is Exists (in which case no value should be specified), or
    the operator is Equal and the values are equal
    An empty key with operator Exists matches all keys, values and effects which means this will tolerate everything.
    {
      "tolerations": [
        {
          "operator": "Exists"
        }
      ]
    }

    //An empty effect matches all effects with key key.
    {
      "tolerations": [
        {
          "key": "key",
          "operator": "Exists"
        }
      ]
    }
    /// Places a taint on node node1. The taint has key key, value value, and taint effect NoSchedule.
    /// This means that no pod will be able to schedule onto node1 unless it has a matching toleration.
    {
      "tolerations": [
        {
          "key": "key",
          "operator": "Equal",
          "value": "value",
          "effect": "NoSchedule"
        }
      ]
    }
*/
    #[serde(default)]
    tolerations: Vec<Tolerations>,

    /*"restartPolicy": String
    Always: Restart Container; Pod phase stays Running.
    OnFailure: Restart Container; Pod phase stays Running.
    Never: Pod phase becomes Failed.*/
    restart_policy: String,
    /*The affinity/anti-affinity feature, expands the types of constraints you can express.
    You can indicate that the rule is “soft”/”preference” rather than a hard requirement, so if the scheduler can’t satisfy it, the pod will still be scheduled
    you can constrain against labels on other pods running on the node (or other topological domain),
    rather than against labels on the node itself, which allows rules about which pods can and cannot be co-located

    “node affinity” and
    “inter-pod affinity/anti-affinity.

    Node affinity is conceptually similar to nodeSelector:  it allows you to constrain which nodes your
    pod is eligible to schedule on, based on labels on the node.

    - You can think of them as “hard” and “soft” respectively, in the sense that the former specifies rules
    that must be met for a pod to schedule onto a node (just like nodeSelector but using a more expressive
    syntax), while the latter specifies preferences that the scheduler will try to enforce but will not
    guarantee.
    The “IgnoredDuringExecution” part of the names means that, similar to how nodeSelector works,
     if labels on a node change at runtime such that the affinity rules on a pod are no longer met,
     the pod will still continue to run on the node. In the future we plan to offer
     requiredDuringSchedulingRequiredDuringExecution which will be just like
     requiredDuringSchedulingIgnoredDuringExecution except that it will evict pods from nodes that cease to
     satisfy the pods’ node affinity requirements.

    - requiredDuringSchedulingIgnoredDuringExecution would be “only run the pod on nodes with Intel CPUs”
    - preferredDuringSchedulingIgnoredDuringExecution would be “try to run this set of pods in availability zone XYZ, but if it’s not possible, then allow some to run elsewhere”.


    {
      "affinity": {
        "nodeAffinity": {
          "requiredDuringSchedulingIgnoredDuringExecution": {
            "nodeSelectorTerms": [
              {
                "matchExpressions": [
                  {
                    "key": "kubernetes.io/e2e-az-name",
                    "operator": "In",
                    "values": [
                      "e2e-az1",
                      "e2e-az2"
                    ]
                  }
                ]
              }
            ]
          },
          "preferredDuringSchedulingIgnoredDuringExecution": [
            {
              "weight": 1,
              "preference": {
                "matchExpressions": [
                  {
                    "key": "another-node-label-key",
                    "operator": "In",
                    "values": [
                      "another-node-label-value"
                    ]
                  }
                ]
              }
            }
          ]
        }
      }
    }

    An example of requiredDuringSchedulingIgnoredDuringExecution affinity would be “co-locate the pods
    of service A and service B in the same zone, since they communicate a lot with each other”
    and an example preferredDuringSchedulingIgnoredDuringExecution anti-affinity would be “spread the pods
    from this service across zones”
    (a hard requirement wouldn’t make sense, since you probably have more pods than zones).

    {
      "affinity": {
        "assemblyfactoryAffinity": {
          "requiredDuringSchedulingIgnoredDuringExecution": [
            {
              "labelSelector": {
                "matchExpressions": [
                  {
                    "key": "security",
                    "operator": "In",
                    "values": [
                      "S1"
                    ]
                  }
                ]
              },
              "topologyKey": "failure-domain.riocorp.io/zone"
            }
          ]
        },
        "assemblyfactoryAntiAffinity": {
          "preferredDuringSchedulingIgnoredDuringExecution": [
            {
              "weight": 100,
              "assemblyfactoryAffinityTerm": {
                "labelSelector": {
                  "matchExpressions": [
                    {
                      "key": "security",
                      "operator": "In",
                      "values": [
                        "S2"
                      ]
                    }
                  ]
                },
                "topologyKey": "riocorp.io/hostname"
              }
            }
          ]
        }
      }
    }

    */
    #[serde(default)]
    affinity: Affinity,
    /*nodeSelector is the simplest form of constraint. It specifies a map of key-value pairs. For the assemblyfactory
   to be eligible to run on a node, the node must have each of the indicated key-value pairs as labels
   The most common usage is one key-value pair.
   eg: Label a nodel as `disktype: ssd`
   When the assembly factory is declared, set
   "nodeselector": [{"disktype": "ssd"}]*/
    #[serde(default)]
    node_selector: BTreeMap<String, String>,

    #[serde(default)]
    plan: Option<Plan>,

    #[serde(default)]
    services: Option<Vec<Services>>,
}

impl AssemblyFactorySpec {
    pub fn new() -> AssemblyFactorySpec {
        ::std::default::Default::default()
    }

    pub fn set_tolerations(&mut self, v: Vec<Tolerations>) {
        self.tolerations = v;
    }

    pub fn get_tolerations(&self) -> &Vec<Tolerations> {
        &self.tolerations
    }
    pub fn set_node_selector(&mut self, v: BTreeMap<String, String>) {
        self.node_selector = v;
    }

    pub fn get_node_selector(&self) -> &BTreeMap<String, String> {
        &self.node_selector
    }

    pub fn set_affinity(&mut self, v: Affinity) {
        self.affinity = v;
    }

    pub fn get_affinity(&self) -> &Affinity {
        &self.affinity
    }
    pub fn set_restart_policy(&mut self, v: ::std::string::String) {
        self.restart_policy = v;
    }

    pub fn get_restart_policy(&self) -> ::std::string::String {
        self.restart_policy.clone()
    }
    pub fn set_plan(&mut self, v: Option<Plan>) {
        self.plan = v;
    }

    pub fn get_plan(&self) -> Option<Plan> {
        self.plan.clone()
    }

    pub fn set_services(&mut self, v: Option<Vec<Services>>) {
        self.services = v;
    }

    pub fn get_services(&self) -> Option<Vec<Services>> {
        self.services.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Tolerations {
    key: String,
    operator: String,
    value: String,
    effect: String,
}
impl Tolerations {
    pub fn with_tolerations(key: &str, operator: &str, value: &str, effect: &str) -> Tolerations {
        Tolerations {
            key: key.to_string(),
            operator: operator.to_string(),
            value: value.to_string(),
            effect: effect.to_string(),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Affinity {
    assemblyfactory_affinity: String,
}
impl Affinity {
    pub fn with_affinity(assemblyfactory_affinity: &str) -> Affinity {
        Affinity { assemblyfactory_affinity: assemblyfactory_affinity.to_string() }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ExecURL {
    pub url: String,
}

///////////// To discuss
/*
//how to attach handlers to Container lifecycle events. rioos supports the postStart and preStop events.
"lifecycle": {
          "postStart": {
            "exec": {
              "command": [
                "/bin/sh",
                "-c",
                "echo Hello from the postStart handler > /usr/share/message"
              ]
            }
          },
          "preStop": {
            "exec": {
              "command": [
                "/usr/sbin/nginx",
                "-s",
                "quit"
              ]
            }
          }
        }
*/

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_assembly_factory() {
        let val = r#"{
            "id":"876123456523456",
            "type_meta":{"kind": "AssemblyFactory", "api_version": "V1"},
            "object_meta": {
                               "name":"levi.megam.io",
                               "account":"87654321",
                              "labels":{
                                  "rioos_environment":"development",
                                  "rioos_category":"machine"
                               },
                               "annotations":{
                                   "rioos/karthika.calvincare.org/apply":"OnHeadBald",
                                   "rioos/ruchi.calvincare.org/pickup":"OnHungry"
                               },
                               "owner_references":[
                               {
                                   "kind":"Assembly",
                                   "api_version":"v1",
                                  "name":"levi.megam.io",
                                   "uid":"0001010",
                                  "block_owner_deletion":true
                              }
                               ],
                               "created_at":"2017-11-20T06:49:06.907347+00:00",
                               "deleted_at":"2017-11-20T06:49:06.907347+00:00",
                               "deletion_grace_period_seconds":30,
                               "initializers":{"pending": [{
                                               "name": "loadbalancer"
                                           }],
                                            "result": {
                                                   "status":"success",
                                                   "message": "omitempty",
                                                   "type_meta":{
                                                       "kind":"",
                                                       "api_version":""
                                                   },
                                                   "reason":"",
                                                   "code": 400,
                                                   "details":{
                                                       "name":"name",
                                                       "group": "grp",
                                                       "kind": "",
                                                       "uid":"",
                                                       "retry_after_seconds": 30,
                                                       "causes": [{
                                                           "cause_type": "",
                                                           "message":"",
                                                           "field":""
                                                       }]
                                                   }

                                            }},
                               "finalizers":[
                                   "orphan"
                                   ],
                               "cluster_name":"dc1_torono"
                          },
            "replicas": 2,
            "resources": {"compute_type":"cpu","stotage_type":"hdd"},
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
            "created_at": "2017-11-20T06:49:06.907347+00:00",
            "secret": {"id": "98765345678654345"},
            "plan":"8765456787654334567",
            "metadata": {
            "io:rioos:orginin::name":"rioos",
             "io:rioos:team::name":"development"
            },
            "spec": {
                "tolerations": [{
                        "key": "key",
                        "operator": "Equal",
                        "value": "value",
                        "effect": "NoSchedule"
                    }],
                "node_selector" : {"node_name": "node1","node_id": "87654345678765"},
                "affinity" : {
                        "assemblyfactory_affinity": "requiredDuringSchedulingIgnoredDuringExecution"
                    },
                "restart_policy": "Always"
            }

    }"#;
        let assemblyfactory: AssemblyFactory = json_decode(val).unwrap();
        assert_eq!(assemblyfactory.replicas, 2);
        assert_eq!(assemblyfactory.resources.len(), 2);
        assert!(assemblyfactory.resources.contains_key("compute_type"));
        assert!(assemblyfactory.resources.contains_key("stotage_type"));
        assert_eq!(assemblyfactory.metadata.len(), 2);
        assert!(assemblyfactory.metadata.contains_key(
            "io:rioos:orginin::name",
        ));
        assert!(assemblyfactory.metadata.contains_key("io:rioos:team::name"));
    }

    #[test]
    fn decode_assembly_factory_without_id_and_created_at() {
        let val = r#"{
            "type_meta":{"kind": "AssemblyFactory", "api_version": "V1"},
            "object_meta": {
                               "name":"levi.megam.io",
                               "account":"87654321234",
                              "labels":{
                                  "rioos_environment":"development",
                                  "rioos_category":"machine"
                               },
                               "annotations":{
                                   "rioos/karthika.calvincare.org/apply":"OnHeadBald",
                                   "rioos/ruchi.calvincare.org/pickup":"OnHungry"
                               },
                               "owner_references":[
                               {
                                   "kind":"Assembly",
                                   "api_version":"v1",
                                  "name":"levi.megam.io",
                                   "uid":"0001010",
                                  "block_owner_deletion":true
                              }
                               ],
                               "created_at":"2017-11-20T06:49:06.907347+00:00",
                               "deleted_at":"2017-11-20T06:49:06.907347+00:00",
                               "deletion_grace_period_seconds":30,
                               "initializers":{"pending": [{
                                               "name": "loadbalancer"
                                           }],
                                            "result": {
                                                   "status":"success",
                                                   "message": "omitempty",
                                                   "type_meta":{
                                                       "kind":"",
                                                       "api_version":""
                                                   },
                                                   "reason":"",
                                                   "code": 400,
                                                   "details":{
                                                       "name":"name",
                                                       "group": "grp",
                                                       "kind": "",
                                                       "uid":"",
                                                       "retry_after_seconds": 30,
                                                       "causes": [{
                                                           "cause_type": "",
                                                           "message":"",
                                                           "field":""
                                                       }]
                                                   }

                                            }},
                               "finalizers":[
                                   "orphan"
                                   ],
                               "cluster_name":"dc1_torono"
                          },
            "replicas": 2,
            "resources": {"compute_type":"cpu","stotage_type":"hdd"},
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
            "secret": {"id": "98765345678654345"},
            "plan":"8765456787654334567",
            "metadata": {
            "io:rioos:orginin::name":"rioos",
             "io:rioos:team::name":"development"
            },
            "spec": {
                "tolerations": [{
                        "key": "key",
                        "operator": "Equal",
                        "value": "value",
                        "effect": "NoSchedule"
                    }],
                "node_selector" : {"node_name": "node1","node_id": "87654345678765"},
                "affinity" : {
                        "assemblyfactory_affinity": "requiredDuringSchedulingIgnoredDuringExecution"
                    },
                "restart_policy": "Always"
            }

    }"#;
        let assemblyfactory: AssemblyFactory = json_decode(val).unwrap();
        assert_eq!(assemblyfactory.replicas, 2);
        assert_eq!(assemblyfactory.id, "");
        assert_eq!(assemblyfactory.created_at, "");
        assert_eq!(assemblyfactory.resources.len(), 2);
        assert!(assemblyfactory.resources.contains_key("compute_type"));
        assert!(assemblyfactory.resources.contains_key("stotage_type"));
        assert_eq!(assemblyfactory.metadata.len(), 2);
        assert!(assemblyfactory.metadata.contains_key(
            "io:rioos:orginin::name",
        ));
        assert!(assemblyfactory.metadata.contains_key("io:rioos:team::name"));
    }

    #[test]
    fn decode_secrets() {
        let secret = r#"{"id": "98765345678654345"}"#;
        let secret_value: Secret = json_decode(secret).unwrap();
        assert_eq!(secret_value.id, "98765345678654345");
    }

    #[test]
    fn decode_tolerations() {
        let tolerations = r#"{
            "key": "key",
            "operator": "Equal",
            "value": "value",
            "effect": "NoSchedule"
        }"#;
        let tolrate: Tolerations = json_decode(tolerations).unwrap();
        assert_eq!(tolrate.key, "key");
        assert_eq!(tolrate.operator, "Equal");
        assert_eq!(tolrate.value, "value");
        assert_eq!(tolrate.effect, "NoSchedule");
    }

    #[test]
    fn decode_affinity() {
        let affinity = r#"{"assemblyfactory_affinity": "requiredDuringSchedulingIgnoredDuringExecution"}"#;
        let affinit: Affinity = json_decode(affinity).unwrap();
        assert_eq!(
            affinit.assemblyfactory_affinity,
            "requiredDuringSchedulingIgnoredDuringExecution"
        );
    }

    #[test]
    fn decode_assembly() {
        let val = r#"{"id":"876123456523456",
        "type_meta":{"kind": "Assembly", "api_version": "V1"},
        "object_meta": {
                                       "name":"levi.megam.io",
                                       "account":"87654323456",
                                      "labels":{
                                          "rioos_environment":"development",
                                          "rioos_category":"machine"
                                       },
                                       "annotations":{
                                           "rioos/karthika.calvincare.org/apply":"OnHeadBald",
                                           "rioos/ruchi.calvincare.org/pickup":"OnHungry"
                                       },
                                       "owner_references":[
                                       {
                                           "kind":"Assembly",
                                           "api_version":"v1",
                                          "name":"levi.megam.io",
                                           "uid":"0001010",
                                          "block_owner_deletion":true
                                      }
                                       ],
                                       "created_at":"2017-11-20T06:49:06.907347+00:00",
                                       "deleted_at":"2017-11-20T06:49:06.907347+00:00",
                                       "deletion_grace_period_seconds":30,
                                       "finalizers":[
                                           "orphan"
                                           ],
                                       "cluster_name":"dc1_torono"
                                  },
        "selector": ["876543456787654"],
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
                "metadata": {
                "io:rioos:scheduled::node":"765434567"
                },
        "created_at": "2017-11-20T06:49:06.907347+00:00"
            }"#;
        let assembly: Assembly = json_decode(val).unwrap();
        assert_eq!(assembly.id, "876123456523456");
        assert_eq!(assembly.created_at, "2017-11-20T06:49:06.907347+00:00");
        assert_eq!(assembly.metadata.len(), 1);
        assert!(assembly.metadata.contains_key("io:rioos:scheduled::node"));
    }

    #[test]
    fn decode_assembly_with_default_id_and_created_at() {
        let val = r#"{
        "type_meta":{"kind": "Assembly", "api_version": "V1"},
        "object_meta": {
                                       "name":"levi.megam.io",
                                       "account":"7654234567",
                                      "labels":{
                                          "rioos_environment":"development",
                                          "rioos_category":"machine"
                                       },
                                       "annotations":{
                                           "rioos/karthika.calvincare.org/apply":"OnHeadBald",
                                           "rioos/ruchi.calvincare.org/pickup":"OnHungry"
                                       },
                                       "owner_references":[
                                       {
                                           "kind":"Assembly",
                                           "api_version":"v1",
                                          "name":"levi.megam.io",
                                           "uid":"0001010",
                                          "block_owner_deletion":true
                                      }
                                       ],
                                       "created_at":"2017-11-20T06:49:06.907347+00:00",
                                       "deleted_at":"2017-11-20T06:49:06.907347+00:00",
                                       "deletion_grace_period_seconds":30,
                                       "finalizers":[
                                           "orphan"
                                           ],
                                       "cluster_name":"dc1_torono"
                                  },
        "selector": ["876543456787654"],
        "metadata": {
        "io:rioos:scheduled::node":"765434567"
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
                }
            }"#;
        let assembly: Assembly = json_decode(val).unwrap();
        assert_eq!(assembly.id, "");
        assert_eq!(assembly.created_at, "");
        assert_eq!(assembly.metadata.len(), 1);
        assert!(assembly.metadata.contains_key("io:rioos:scheduled::node"));
    }

}
