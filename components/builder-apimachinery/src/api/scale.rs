// Copyright 2018 The Rio Advancement Inc
use api::base::{MetaFields, ObjectMeta, TypeMeta};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HorizontalScaling {
    #[serde(default)]
    id: String, //id an unique identifier in systems of record. Generated during creation of the AssemblyFactory
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Horizontalscaling
    /*There are two types of  horizontal scaler.
    “MANUALHS” - Manual horizontal scaler directly scales the virtual machine/containers with no  metrics. It acts as a passthru.
    This is useful when during a deploy we say “launch this app + loadbalance with 2 instances under the LB”.
    “AUTOHS” - Auto horizontal scaler is automatically scale vm/container using scaling rules. It starts with the minimum and moves upto max as per the rule.*/
    scale_type: String,
    /*The state represents when controller apply the scaling rule on assemblies. It have the following labels:
    “ABLETOSCALE” - Ready to scale the assembly
    “LIMITREACHED” - it represents target limit was reached then the controller doesn’t deploy the another assembly*/
    state: String,
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    spec: Spec,
    status: Status, //Most recently observed status of the service.last_scale_time,current_replicas,desired_replicas details
    #[serde(default)]
    created_at: String,
}
impl HorizontalScaling {
    pub fn new() -> HorizontalScaling {
        ::std::default::Default::default()
    }

    //Create a new horizontalscaling with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> HorizontalScaling {
        HorizontalScaling {
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

    pub fn set_scale_type(&mut self, v: ::std::string::String) {
        self.scale_type = v;
    }

    pub fn get_scale_type(&self) -> ::std::string::String {
        self.scale_type.clone()
    }

    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = v;
    }

    pub fn get_state(&self) -> ::std::string::String {
        self.state.clone()
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_spec(&mut self, v: Spec) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> &Spec {
        &self.spec
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
impl MetaFields for HorizontalScaling {
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
pub struct Spec {
    min_replicas: u32, //Min_replicas is the lower limit for the number of replicas to which the horizontalscaler can scale down
    max_replicas: u32, //Max Replicas is the upper limit for the number of replicas to which the horizontalscaler can scale up. It cannot be less that minReplicas
    scale_up_wait_time: u32,
    scale_down_wait_time: u32,
    metrics: Vec<Metrics>,
}

impl Spec {
    pub fn new() -> Spec {
        ::std::default::Default::default()
    }
    pub fn set_min_replicas(&mut self, v: u32) {
        self.min_replicas = v;
    }

    pub fn set_max_replicas(&mut self, v: u32) {
        self.max_replicas = v;
    }

    pub fn set_metrics(&mut self, v: Vec<Metrics>) {
        self.metrics = v;
    }
    pub fn get_min_replicas(&self) -> u32 {
        self.min_replicas
    }

    pub fn get_max_replicas(&self) -> u32 {
        self.max_replicas
    }

    pub fn get_metrics(&self) -> Vec<Metrics> {
        self.metrics.clone()
    }
    pub fn set_scale_down_wait_time(&mut self, v: u32) {
        self.scale_down_wait_time = v;
    }
    pub fn set_scale_up_wait_time(&mut self, v: u32) {
        self.scale_up_wait_time = v;
    }
    pub fn get_scale_up_wait_time(&self) -> u32 {
        self.scale_up_wait_time.clone()
    }
    pub fn get_scale_down_wait_time(&self) -> u32 {
        self.scale_down_wait_time.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Metrics {
    metric_type: String, //`Type` is the type of metric source.  It should be one of "Object",  "Resource", each mapping to a matching field in the object.
    #[serde(default)]
    object: MetricObject, //`Object` refers to a metric describing a single rioos object. (for example, hits-per-second on an Services:Loadbalancer object).
    resource: MetricResource, //`Resource` refers to a resource metric (such as those specified in requests and limits) known to rioos describing each assembly in the current scale target (e.g. CPU or memory).
                              //Such metrics are built in to Rioos, and have special scaling options on top of those available to normal per-assembly metrics using the "AssemblyFactory" source.
}

impl Metrics {
    pub fn new() -> Metrics {
        ::std::default::Default::default()
    }
    pub fn set_metric_type(&mut self, v: ::std::string::String) {
        self.metric_type = v;
    }
    pub fn set_metric_object(&mut self, v: MetricObject) {
        self.object = v;
    }

    pub fn set_metric_resource(&mut self, v: MetricResource) {
        self.resource = v;
    }
    pub fn get_metric_resource(&self) -> MetricResource {
        self.resource.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MetricObject {
    target: String, //`Target`:  Target is the described Rioos object. (example Services)
    target_value: u32, //`TargetValue` is the target value of the metric (as a quantity).
    metric_time_spec: TimeSpec,
}

impl MetricObject {
    pub fn new() -> MetricObject {
        ::std::default::Default::default()
    }
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = v;
    }
    pub fn set_target_value(&mut self, v: u32) {
        self.target_value = v;
    }
    pub fn set_metric_time_spec(&mut self, v: TimeSpec) {
        self.metric_time_spec = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MetricResource {
    name: String,             //`Name` is the name of the metric in question.
    min_target_value: String, //`MinTargetValue` is the Instance Range for a range of machines allowed to run. This is the minimum allowed.
    max_target_value: String, //`MaxTargetValue` is the Instance Range for a range of machines allowed to run. This is the maximum allowed. Example minimum 2 machines shall be running, scaled up to 4 machines.
    metric_time_spec: TimeSpec,
}

impl MetricResource {
    pub fn new() -> MetricResource {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn set_min_target_value(&mut self, v: ::std::string::String) {
        self.min_target_value = v;
    }
    pub fn set_max_target_value(&mut self, v: ::std::string::String) {
        self.max_target_value = v;
    }
    pub fn set_metric_time_spec(&mut self, v: TimeSpec) {
        self.metric_time_spec = v;
    }
    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn get_min_target_value(&self) -> ::std::string::String {
        self.min_target_value.clone()
    }
    pub fn get_max_target_value(&self) -> ::std::string::String {
        self.max_target_value.clone()
    }
    pub fn get_metric_time_spec(&self) -> TimeSpec {
        self.metric_time_spec.clone()
    }
}

//`scale_up_by` `scale_up_wait_time`, ..down..: Control the rate of scaling in two ways, both for scaling up and for scaling down. We can define how many instances are added or removed, and set a delay.
// By default, one instance is added/removed, and there is a 5-minute delay. You can tweak these settings to increase the rate of growth. For example, powering up more machines at once gives a big step up.
//However, powering up one machine more frequently gives a more gradual rate of growth, which allows time for the OS and guest services to start up.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TimeSpec {
    scale_up_by: String,
    scale_down_by: String,
}

impl TimeSpec {
    pub fn new() -> TimeSpec {
        ::std::default::Default::default()
    }
    pub fn set_scale_up_by(&mut self, v: ::std::string::String) {
        self.scale_up_by = v;
    }

    pub fn set_scale_down_by(&mut self, v: ::std::string::String) {
        self.scale_down_by = v;
    }

    pub fn get_scale_up_by(&self) -> ::std::string::String {
        self.scale_up_by.clone()
    }

    pub fn get_scale_down_by(&self) -> ::std::string::String {
        self.scale_down_by.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    last_scale_time: String, //`last_scale_time` is the last time the HorizontalScaler scaled the number of assemblys,  used by the horizontalscaler to control how often the number of assemblys is changed.
    current_replicas: u32, //`current_replicas` is current number of replicas of assemblys managed by this horizontalscaler, as last seen by the horizontalscaler.
    desired_replicas: u32, //`desired_replicas` is the desired number of replicas of assemblys managed by this horizontalscaler, as last calculated by the horizontalscaler.
}

impl Status {
    pub fn new(last_scale_time: &str, current_replicas: u32, desired_replicas: u32) -> Status {
        Status {
            last_scale_time: last_scale_time.to_string(),
            current_replicas: current_replicas,
            desired_replicas: desired_replicas,
        }
    }
    pub fn get_last_scale_time(&self) -> ::std::string::String {
        self.last_scale_time.clone()
    }
    pub fn get_current_replicas(&self) -> u32 {
        self.current_replicas.clone()
    }
    pub fn get_desired_replicas(&self) -> u32 {
        self.desired_replicas.clone()
    }
}
///The status that is used to parse request in /status update of any api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusUpdate {
    pub status: Status,
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

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ScalingGetResponse {
    metrics: BTreeMap<String, BTreeMap<String, String>>,
}

impl ScalingGetResponse {
    pub fn new() -> ScalingGetResponse {
        ::std::default::Default::default()
    }
    pub fn set_metrics(&mut self, v: BTreeMap<String, BTreeMap<String, String>>) {
        self.metrics = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct VerticalScaling {
    #[serde(default)]
    id: String, //id an unique identifier in systems of record. Generated during creation of the VerticalScaling
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Horizontalscaling
    /*There are two types of  horizontal scaler.
    “MANUALVS” - Manual vertical scaler directly scales the virtual machine/containers with  metrics.
    “AUTOVS” - Auto vertical scaler is automatically scale vm/container resources using scaling rules. It starts with the minimum and moves upto max as per the rule.*/
    scale_type: String,
    state: String,
    update_policy: UpdatePolicy, //The update policy controls how verticalscaling(VS) applies changes.
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    spec: VerticalScalingSpec,
    status: VerticalScalingStatus, //Most recently observed status of the service.last_scale_time,current_replicas,desired_replicas details
    #[serde(default)]
    created_at: String,
}

impl VerticalScaling {
    pub fn new() -> VerticalScaling {
        ::std::default::Default::default()
    }

    //Create a new horizontalscaling with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> VerticalScaling {
        VerticalScaling {
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

    pub fn set_scale_type(&mut self, v: ::std::string::String) {
        self.scale_type = v;
    }

    pub fn get_scale_type(&self) -> ::std::string::String {
        self.scale_type.clone()
    }

    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = v;
    }

    pub fn get_state(&self) -> ::std::string::String {
        self.state.clone()
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_spec(&mut self, v: VerticalScalingSpec) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> &VerticalScalingSpec {
        &self.spec
    }

    pub fn set_status(&mut self, v: VerticalScalingStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> &VerticalScalingStatus {
        &self.status
    }

    pub fn set_update_policy(&mut self, v: UpdatePolicy) {
        self.update_policy = v;
    }

    pub fn get_update_policy(&self) -> &UpdatePolicy {
        &self.update_policy
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for VerticalScaling {
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
/*The update policy controls how verticalscaling(VS) applies changes.
"updatePolicy" {
  "mode": "",
}
Mode can be set to one of the following:
"auto" (default): VS assigns resources on assembly creation and additionally can update them during lifetime of the assembly, including evicting / rescheduling the assembly.
"off": VS assigns resources on assembly creation and additionally can update them during lifetime of the assembly, but doesn’t allow  evicting / rescheduling the assembly from the node it runs.
 Instead it informs the user by posting an error in the events that gets shows to the user.
 */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePolicy {
    mode: String,
}

impl UpdatePolicy {
    pub fn new() -> UpdatePolicy {
        ::std::default::Default::default()
    }
    pub fn set_mod(&mut self, v: ::std::string::String) {
        self.mode = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct VerticalScalingSpec {
    scale_up_wait_time: u32,
    scale_down_wait_time: u32,
    min_resource: BTreeMap<String, String>, //Min_resource is the lower limit for the number of resource to which the vertical scaler can scale down
    max_resource: BTreeMap<String, String>, //Max resource is the upper limit for the number of resource to which the verticalscaler can scale up.
    /*eg:

An assembly factory with 3 assemblys can have min_resource of 30% of cpu, max_resouce of 60%.
Based on the strategy, the vertical scaling will occur.

The strategies can be

`CumulativeAverage `
- The min_resource average utilization  of (cpu) for the 3 assembly's must be 30% >
- The max_resource average utilization  of (cpu) for the 3 assembly's must be 60% <

`AtleastOne`
- The min_resource utilization of (cpu) for atleast one of the assembly  must be 30% >
- The max_resource utilization of (cpu) for atleast one of the assembly  must be 60% >

`CumulativeAll`
- The min_resource utilization of (cpu) for all of the assembly  must be 30% >
- The max_resource utilization of (cpu) for all of the assembly  must be 60% >

`BestFitHistorical`
- The min_resource utilization of (cpu) for best fit historical data for similar assembly's  must be 30% >
- The max_resource utilization of (cpu) for best fit historical data for similar assembly's  must be 60% >
*/
    metrics: Vec<Metrics>,
}

impl VerticalScalingSpec {
    pub fn new() -> VerticalScalingSpec {
        ::std::default::Default::default()
    }
    pub fn set_min_resource(&mut self, v: BTreeMap<String, String>) {
        self.min_resource = v;
    }

    pub fn set_max_resource(&mut self, v: BTreeMap<String, String>) {
        self.max_resource = v;
    }

    pub fn set_metrics(&mut self, v: Vec<Metrics>) {
        self.metrics = v;
    }
    pub fn get_min_resource(&self) -> BTreeMap<String, String> {
        self.min_resource.clone()
    }

    pub fn get_max_resource(&self) -> BTreeMap<String, String> {
        self.max_resource.clone()
    }

    pub fn get_metrics(&self) -> Vec<Metrics> {
        self.metrics.clone()
    }

    pub fn set_scale_down_wait_time(&mut self, v: u32) {
        self.scale_down_wait_time = v;
    }
    pub fn set_scale_up_wait_time(&mut self, v: u32) {
        self.scale_up_wait_time = v;
    }
    pub fn get_scale_up_wait_time(&self) -> u32 {
        self.scale_up_wait_time.clone()
    }
    pub fn get_scale_down_wait_time(&self) -> u32 {
        self.scale_down_wait_time.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct VerticalScalingStatus {
    #[serde(default)]
    last_scale_time: String,
    current_resource: BTreeMap<String, String>, //`current_resources` is current resource average of the metrics for the assemblyfactory managed by this verticalscaler, as last seen by the verticalscaler.
    desired_resource: BTreeMap<String, String>, //`desired_resources` is desired resource average of the metrics for the assemblyfactory managed by this verticalscaler, as last seen by the verticalscaler.
    /*eg: An assemblyfactory with 3 assemblyfactory with cpu metrics  in %

```    assembly01: 60
    assembly02: 30
    assembly03: 40
```



For the strategy:

`CumulativeAverage `
- The min_resource average utilization  of (cpu) for the 3 assembly's must be 30% >
- The max_resource average utilization  of (cpu) for the 3 assembly's must be 60% <
`current_resource: (60 + 30 + 40)/3`
`desired_resource: ?`


`AtleastOne`
- The min_resource utilization of (cpu) for atleast one of the assembly  must be 30% >
- The max_resource utilization of (cpu) for atleast one of the assembly  must be 60% >

`current_resource: 30 - stick the min_resource value`
`desired_resource: 30 - stick the min_resource value`
(Since none of them have gone beyond the clip level, we are saying everything in normal)
(If the average of all the assembly0x have gone beyond the clip level, this will scale)

```    assembly01: 40
    assembly02: 80
```






`current_resource: 70 - In abnormal condition, stick the value of the assembly0x which has the shooted above threshhold`
`desired_resource: 70 - stick the value of the assembly0x value`
(Here we found one of the assembly beyong the clip level.)


`CumulativeAll`
- The min_resource utilization of (cpu) for all of the assembly  must be 30% >
- The max_resource utilization of (cpu) for all of the assembly  must be 60% >

`current_resource: 70 - stick the resource value of the average`
`desired_resource: 70 - stick the resource value of the average`
(Only if all the assembly0x have gone beyond the clip level, this will scale)

The following will not scale for  `CumulativeAll` strategy

```    assembly01: 40
    assembly02: 80
```



The following will  scale for  `CumulativeAll` strategy

```    assembly01: 70
    assembly02: 70
```



`BestFitHistorical`
- The min_resource utilization of (cpu) for best fit historical data for similar assembly's  must be 30% >
- The max_resource utilization of (cpu) for best fit historical data for similar assembly's  must be 60% >
*/
}

impl VerticalScalingStatus {
    pub fn new(
        last_scale_time: &str,
        current_resource: BTreeMap<String, String>,
        desired_resource: BTreeMap<String, String>,
    ) -> VerticalScalingStatus {
        VerticalScalingStatus {
            last_scale_time: last_scale_time.to_string(),
            current_resource: current_resource,
            desired_resource: desired_resource,
        }
    }
    pub fn get_last_scale_time(&self) -> ::std::string::String {
        self.last_scale_time.clone()
    }
    pub fn get_current_resource(&self) -> BTreeMap<String, String> {
        self.current_resource.clone()
    }
    pub fn get_desired_resource(&self) -> BTreeMap<String, String> {
        self.desired_resource.clone()
    }
}

///The status that is used to parse request in /status update of any api.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerticalScalingStatusUpdate {
    pub status: VerticalScalingStatus,
    #[serde(default)]
    id: String,
}

impl VerticalScalingStatusUpdate {
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_status(&mut self, v: VerticalScalingStatus) {
        self.status = v;
    }

    pub fn get_status(&self) -> &VerticalScalingStatus {
        &self.status
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_horizontal_scaling() {
        let val = r#"{
            "object_meta":{"name":"hzscaling","account":"8765423456787655","labels":{},"annotations":{},"owner_references":[{"kind":"AssemblyFactory","api_version":"v1","name":"lev.megam.io","uid":"8765345678765434567","block_owner_deletion":true}],
            "created_at":"","deleted_at":"","deletion_grace_period_seconds":0, "finalizers":[],"cluster_name":""},
            "status":{"last_scale_time":"","current_replicas":1,"desired_replicas":1},
            "scale_type":"AUTOHS",
            "state":"data",
            "metadata":{},
            "spec":{"scale_up_wait_time":5,"scale_down_wait_time":5,"min_replicas":4,"max_replicas":5,"metrics":[{"metric_type": "Resource","object":
            {"target": "hits_as_per_second","target_value":1000,"metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}},
            "resource":{"name": "memory","min_target_value":"2","max_target_value":"4","metric_time_spec":{"scale_up_by":"5m","scale_down_by":"5m"}}}]}
    }"#;
        let scale: HorizontalScaling = json_decode(val).unwrap();
        assert_eq!(scale.scale_type, "AUTOHS");
        assert_eq!(scale.state, "data");
    }

    #[test]
    fn decode_scale_spec() {
        let val = r#"
        {
        "scale_up_wait_time":5,
        "scale_down_wait_time":5,
        "min_replicas": 4,
        "max_replicas": 5,
        "metrics": [{
            "metric_type": "Resource",
            "object":
                {
                "target": "hits_as_per_second",
                "target_value":1000,
                "metric_time_spec": {
                    "scale_up_by":"5m",
                    "scale_down_by":"5m"
                                }
                            },
                "resource":{
                    "name": "memory",
                    "min_target_value":"2",
                    "max_target_value":"4",
                    "metric_time_spec":{
                        "scale_up_by":"5m",
                        "scale_down_by":"5m"
                            }
                        }
                    }]
        }"#;
        let spec: Spec = json_decode(val).unwrap();
        assert_eq!(spec.scale_up_wait_time, 5);
        assert_eq!(spec.scale_down_wait_time, 5);
        assert_eq!(spec.min_replicas, 4);
        assert_eq!(spec.max_replicas, 5);
        assert_eq!(spec.metrics.len(), 1);
    }

    #[test]
    fn decode_scale_metrics() {
        let val = r#"
        {
        "metric_type": "Resource",
        "object": {
            "target": "hits_as_per_second",
            "target_value": 1000,
            "metric_time_spec": {
                "scale_up_by":"5m",
                "scale_down_by":"5m"
                            }
                        },
        "resource": {
            "name": "memory",
            "min_target_value":"2",
            "max_target_value":"4",
            "metric_time_spec":{
                "scale_up_by":"5m",
                "scale_down_by":"5m"
            }}
        }"#;
        let met: Metrics = json_decode(val).unwrap();
        assert_eq!(met.metric_type, "Resource");
    }
    #[test]
    fn decode_scale_metric_object() {
        let val = r#"
        {
        "target": "hits_as_per_second",
        "target_value": 1000,
        "metric_time_spec": {
            "scale_up_by":"5m",
            "scale_down_by":"5m"
        }
        }"#;
        let met: MetricObject = json_decode(val).unwrap();
        assert_eq!(met.target, "hits_as_per_second");
        assert_eq!(met.target_value, 1000);
    }

    #[test]
    fn decode_scale_metric_object_time_spec() {
        let val = r#"
        {
        "scale_up_by": "5m",
        "scale_down_by": "5m"
        }"#;
        let met: TimeSpec = json_decode(val).unwrap();
        assert_eq!(met.scale_up_by, "5m");
        assert_eq!(met.scale_down_by, "5m");
    }
    #[test]
    fn decode_scale_status() {
        let val = r#"
        {
        "last_scale_time": "",
        "current_replicas": 1,
        "desired_replicas": 1
        }"#;
        let status: Status = json_decode(val).unwrap();
        assert_eq!(status.last_scale_time, "");
        assert_eq!(status.current_replicas, 1);
        assert_eq!(status.desired_replicas, 1);
    }

    #[test]
    fn decode_vertica_scaling() {
        let val = r#"{
            "update_policy":
                    {
                        "mode":"auto"
                    },
            "object_meta":
                    {
                        "name": "hzscaling",
                        "account":"098765432",
                        "labels":{},
                        "annotations":{},
                        "owner_references":[{
                                "kind":"Assembly",
                                "api_version":"v1",
                                "name":"lev1.megam.io",
                                "uid":"876556786543",
                                "block_owner_deletion":false
                                            }],
                        "created_at":"",
                        "deleted_at":"",
                        "deletion_grace_period_seconds":0,
                        "finalizers":[],
                        "cluster_name":""
                        },
             "status":
                    {
                        "last_scale_time": "",
                        "current_resource":
                                {
                                    "cpu":"2",
                                    "ram":"1000 GiB"
                                },
                        "desired_resource":
                                {
                                    "cpu":"3",
                                    "ram":"2000 GiB"
                                }
                        },
            "scale_type":"AUTOVS",
            "state":"active",
            "metadata":{},
            "spec":
                    {
                        "scale_up_wait_time":5,
                        "scale_down_wait_time":5,
                        "min_resource":
                                {
                                    "cpu":"2",
                                    "Ram":"1000 MiB"
                                },
                        "max_resource":
                                {
                                    "cpu":"4",
                                    "Ram":"4000 MiB"
                                },
                        "metrics":[
                            {
                                "metric_type": "Resource",
                                "resource":
                                        {
                                            "name": "memory",
                                            "min_target_value":"2",
                                            "max_target_value":"4",
                                            "metric_time_spec":
                                                {
                                                    "scale_up_by":"5m",
                                                    "scale_down_by":"5m"
                                                }
                                            }
                                }
                                       ]
                        }
    }"#;
        let vertical: VerticalScaling = json_decode(val).unwrap();
        assert_eq!(vertical.update_policy.mode, "auto");
        assert_eq!(vertical.scale_type, "AUTOVS");
        assert_eq!(vertical.state, "active");
    }

}
