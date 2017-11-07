// Copyright (c) 2017 RioCorp Inc.
use {asmsrv, nodesrv};
use constants::*;
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HorizontalScaling {
    id: String,
    object_meta: asmsrv::ObjectMeta,
    type_meta: asmsrv::TypeMeta,
    name: String,
    description: String,
    tags: Vec<String>,
    origin: String,
    scale_type: String,
    representation_skew: String,
    state: String,
    metadata: Vec<String>,
    spec: Spec,
    status: Status,
    created_at: String,
}
impl HorizontalScaling {
    pub fn new() -> HorizontalScaling {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }

    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_tags(&mut self, v: ::std::vec::Vec<String>) {
        self.tags = v;
    }

    pub fn get_tags(&self) -> ::std::vec::Vec<String> {
        self.tags.clone()
    }

    pub fn set_scale_type(&mut self, v: ::std::string::String) {
        self.scale_type = v;
    }

    pub fn get_scale_type(&self) -> ::std::string::String {
        self.scale_type.clone()
    }

    pub fn set_representation_skew(&mut self, v: ::std::string::String) {
        self.representation_skew = v;
    }

    pub fn get_representation_skew(&self) -> ::std::string::String {
        self.representation_skew.clone()
    }

    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = v;
    }

    pub fn get_state(&self) -> ::std::string::String {
        self.state.clone()
    }

    pub fn set_metadata(&mut self, v: ::std::vec::Vec<String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> ::std::vec::Vec<String> {
        self.metadata.clone()
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
    pub fn set_type_meta(&mut self, v: asmsrv::TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &asmsrv::TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: asmsrv::ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &asmsrv::ObjectMeta {
        &self.object_meta
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Spec {
    scale_target_ref: String,
    min_replicas: u64,
    max_replicas: u64,
    metrics: Vec<Metrics>,
}

impl Spec {
    pub fn new() -> Spec {
        ::std::default::Default::default()
    }
    pub fn set_scale_target_ref(&mut self, v: ::std::string::String) {
        self.scale_target_ref = v;
    }

    pub fn set_min_replicas(&mut self, v: u64) {
        self.min_replicas = v;
    }

    pub fn set_max_replicas(&mut self, v: u64) {
        self.max_replicas = v;
    }

    pub fn set_metrics(&mut self, v: Vec<Metrics>) {
        self.metrics = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Metrics {
    metric_type: String,
    object: MetricObject,
    resource: MetricResource,
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
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MetricObject {
    target: String,
    target_value: u64,
    metric_time_spec: TimeSpec,
}

impl MetricObject {
    pub fn new() -> MetricObject {
        ::std::default::Default::default()
    }
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = v;
    }
    pub fn set_target_value(&mut self, v: u64) {
        self.target_value = v;
    }
    pub fn set_metric_time_spec(&mut self, v: TimeSpec) {
        self.metric_time_spec = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct MetricResource {
    name: String,
    min_target_value: String,
    max_target_value: String,
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
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TimeSpec {
    scale_up_by: String,
    scale_up_wait_time: String,
    scale_down_by: String,
    scale_down_wait_time: String,
}

impl TimeSpec {
    pub fn new() -> TimeSpec {
        ::std::default::Default::default()
    }
    pub fn set_scale_up_by(&mut self, v: ::std::string::String) {
        self.scale_up_by = v;
    }

    pub fn set_scale_up_wait_time(&mut self, v: ::std::string::String) {
        self.scale_up_wait_time = v;
    }
    pub fn set_scale_down_by(&mut self, v: ::std::string::String) {
        self.scale_down_by = v;
    }
    pub fn set_scale_down_wait_time(&mut self, v: ::std::string::String) {
        self.scale_down_wait_time = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    last_scale_time: String,
    current_replicas: u64,
    desired_replicas: u64,
}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }
    pub fn set_last_scale_time(&mut self, v: ::std::string::String) {
        self.last_scale_time = v;
    }

    pub fn set_current_replicas(&mut self, v: u64) {
        self.current_replicas = v;
    }
    pub fn set_desired_replicas(&mut self, v: u64) {
        self.desired_replicas = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct HorizontalScalingGetResponse {
    kind: String,
    api_version: String,
    items: Vec<HorizontalScaling>,
}


impl HorizontalScalingGetResponse {
    pub fn new() -> HorizontalScalingGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_hs_collection(&mut self, v: Vec<HorizontalScaling>) {
        self.items = v;
        self.kind = HORIZONTALSCALINGLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }
}
/////// REFACTOR later using trait ResultResult which has a type kind.

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ScalingGet {
    title: String,
    from_date: String,
    to_date: String,
    metrics: nodesrv::Osusages,
}

impl ScalingGet {
    pub fn new() -> ScalingGet {
        ::std::default::Default::default()
    }
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = v;
    }

    pub fn set_metrics(&mut self, v: nodesrv::Osusages) {
        self.metrics = v;
    }
    pub fn set_from_date(&mut self, v: ::std::string::String) {
        self.from_date = v;
    }
    pub fn set_to_date(&mut self, v: ::std::string::String) {
        self.to_date = v;
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ScalingGetResponse {
    kind: String,
    api_version: String,
    id: String,
    results: ScalingGet,
}

impl ScalingGetResponse {
    pub fn new() -> ScalingGetResponse {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
    pub fn set_results(&mut self, v: ScalingGet) {
        self.results = v;
    }
}


impl Into<ScalingGetResponse> for ScalingGet {
    fn into(self) -> ScalingGetResponse {
        let mut scaling_response = ScalingGetResponse::new();
        scaling_response.set_results(self);
        scaling_response.set_kind(ASSEMBLYMETRICLIST.to_string());
        scaling_response.set_api_version(DEFAULT_API_VERSION.to_string());
        scaling_response.set_id(ASSEMBLYMETRICLIST.to_string());
        scaling_response
    }
}
