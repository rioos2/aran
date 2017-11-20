// Copyright (c) 2017 RioCorp Inc.

use plansrv;
use servicesrv;
use std::collections::BTreeMap;
use DEFAULT_API_VERSION;
use std::path::PathBuf;
const ASSEMBLYLIST: &'static str = "AssemblyList";
const ASSEMBLYFACTORYLIST: &'static str = "AssemblyFactoryList";
pub const INITIAL_CONDITIONS: &'static [&'static str] = &["AssemblyStorageReady", "AssemblyNetworkReady"];
pub const SERVICE_LB_INITIAL_CONDITIONS: &'static [&'static str] = &["ServiceFrontendReady", "ServiceBackendReady"];
pub const SERVICE_DNS_INITIAL_CONDITIONS: &'static [&'static str] = &["ServiceFrontendReady", "ServiceBackendReady"];

pub const NEW_REPLICA_INITALIZING: &'static str = "Initializing replica ";
pub const ASSEMBLYS_URI: &'static str = "v1/assembly";
pub const INITIALIZING: &'static str = "Initializing";
const SERVICE: &'static str = "Service";
pub const LOADBALANCER: &'static str = "LoadBalancer";
pub const EXTERNALNAME: &'static str = "ExternalName";



#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Assembly {
    id: String,
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    selector: Vec<String>,
    name: String,
    uri: String,
    description: String,
    parent_id: String,
    origin: String,
    tags: Vec<String>,
    node: String,
    urls: BTreeMap<String, String>,
    status: Status,
    volumes: Vec<Volume>,
    instance_id: String,
    spec: Option<AssemblyFactory>,
    endpoints: Option<servicesrv::EndPoints>,
    created_at: String,
}

impl Assembly {
    pub fn new() -> Assembly {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn get_parent_id(&self) -> ::std::string::String {
        self.parent_id.clone()
    }

    pub fn set_parent_id(&mut self, v: ::std::string::String) {
        self.parent_id = v;
    }
    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }

    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    pub fn get_uri(&self) -> ::std::string::String {
        self.uri.clone()
    }

    pub fn set_urls(&mut self, v: BTreeMap<String, String>) {
        self.urls = v;
    }

    pub fn get_urls(&self) -> &BTreeMap<String, String> {
        &self.urls
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }
    pub fn set_instance_id(&mut self, v: ::std::string::String) {
        self.instance_id = v;
    }

    pub fn get_instance_id(&self) -> ::std::string::String {
        self.instance_id.clone()
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

    pub fn set_selector(&mut self, v: ::std::vec::Vec<String>) {
        self.selector = v;
    }

    pub fn get_selector(&self) -> ::std::vec::Vec<String> {
        self.selector.clone()
    }
    pub fn set_type_meta(&mut self, v: TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMeta {
        &self.object_meta
    }

    pub fn set_node(&mut self, v: ::std::string::String) {
        self.node = v;
    }

    pub fn get_node(&self) -> ::std::string::String {
        self.node.clone()
    }

    pub fn set_volumes(&mut self, v: Vec<Volume>) {
        self.volumes = v;
    }

    pub fn get_volumes(&self) -> &Vec<Volume> {
        &self.volumes
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

    pub fn set_spec(&mut self, v: Option<AssemblyFactory>) {
        self.spec = v;
    }

    pub fn get_spec(&self) -> Option<AssemblyFactory> {
        self.spec.clone()
    }
    pub fn set_endpoints(&mut self, v: Option<servicesrv::EndPoints>) {
        self.endpoints = v;
    }

    pub fn get_endpoints(&self) -> Option<servicesrv::EndPoints> {
        self.endpoints.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    phase: String,
    message: String,
    reason: String,
    conditions: Vec<Condition>,
}

impl Status {
    pub fn get_phase(&self) -> ::std::string::String {
        self.phase.clone()
    }
    pub fn with_conditions(phase: &str, message: &str, reason: &str, conditions: Vec<Condition>) -> Status {
        Status {
            phase: phase.to_string(),
            message: message.to_string(),
            conditions: conditions,
            reason: reason.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Volume {
    id: u32,
    target: String,
    volume_type: String,
    size: String,
}

impl Volume {
    pub fn with_volumes(id: u32, target: &str, volume_type: &str, size: &str) -> Volume {
        Volume {
            id: id,
            target: target.to_string(),
            volume_type: volume_type.to_string(),
            size: size.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Condition {
    message: String,
    reason: String,
    status: String,
    last_transition_time: String,
    last_probe_time: String,
    condition_type: String,
}

impl Condition {
    pub fn with_type(message: &str, reason: &str, status: &str, last_transition_time: &str, last_probe_time: &str, condition_type: &str) -> Condition {
        Condition {
            condition_type: condition_type.to_string(),
            status: status.to_string(),
            reason: reason.to_string(),
            last_transition_time: last_transition_time.to_string(),
            message: message.to_string(),
            last_probe_time: last_probe_time.to_string(),
        }
    }

    pub fn get_condition_type(&self) -> ::std::string::String {
        self.condition_type.clone()
    }
    pub fn get_status(&self) -> ::std::string::String {
        self.status.clone()
    }
    pub fn get_reason(&self) -> ::std::string::String {
        self.reason.clone()
    }
    pub fn get_last_transition_time(&self) -> ::std::string::String {
        self.last_transition_time.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblysGetResponse {
    kind: String,
    api_version: String,
    items: Vec<Assembly>,
}


impl AssemblysGetResponse {
    pub fn new() -> AssemblysGetResponse {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
    pub fn set_assemblys(&mut self, v: Vec<Assembly>) {
        self.items = v;
        self.kind = ASSEMBLYLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }

    pub fn get_items(&self) -> Vec<Assembly> {
        self.items.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct IdGet {
    id: String,
    name: String,
}

impl IdGet {
    pub fn new() -> IdGet {
        ::std::default::Default::default()
    }
    // Param is passed by value, moved
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
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactory {
    id: String,
    type_meta: TypeMeta,
    object_meta: ObjectMeta,
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    origin: String,
    replicas: u32,
    properties: Properties,
    plan: String,
    plan_data: plansrv::Plan,
    external_management_resource: Vec<String>,
    component_collection: BTreeMap<String, String>,
    status: Status,
    opssettings: OpsSettings,
    created_at: String,
}

impl AssemblyFactory {
    pub fn new() -> AssemblyFactory {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
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

    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    pub fn get_uri(&self) -> ::std::string::String {
        self.uri.clone()
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

    pub fn set_external_management_resource(&mut self, v: ::std::vec::Vec<String>) {
        self.external_management_resource = v;
    }

    pub fn get_external_management_resource(&self) -> ::std::vec::Vec<String> {
        self.external_management_resource.clone()
    }

    pub fn set_plan(&mut self, v: ::std::string::String) {
        self.plan = v;
    }

    pub fn get_plan(&self) -> ::std::string::String {
        self.plan.clone()
    }

    pub fn set_properties(&mut self, v: Properties) {
        self.properties = v;
    }

    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn set_type_meta(&mut self, v: TypeMeta) {
        self.type_meta = v;
    }

    pub fn get_type_meta(&self) -> &TypeMeta {
        &self.type_meta
    }

    pub fn set_object_meta(&mut self, v: ObjectMeta) {
        self.object_meta = v;
    }

    pub fn get_object_meta(&self) -> &ObjectMeta {
        &self.object_meta
    }


    pub fn set_component_collection(&mut self, v: BTreeMap<String, String>) {
        self.component_collection = v;
    }

    pub fn get_component_collection(&self) -> &BTreeMap<String, String> {
        &self.component_collection
    }

    pub fn set_status(&mut self, v: Status) {
        self.status = v;
    }

    pub fn get_status(&self) -> &Status {
        &self.status
    }

    pub fn set_opssettings(&mut self, v: OpsSettings) {
        self.opssettings = v;
    }

    pub fn get_opssettings(&self) -> &OpsSettings {
        &self.opssettings
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_plan_data(&mut self, v: plansrv::Plan) {
        self.plan_data = v;
    }
    pub fn get_plan_data(&self) -> &plansrv::Plan {
        &self.plan_data
    }
}

pub fn generate_service(assembly_fac: &AssemblyFactory, service_type: &str) -> servicesrv::Services {
    let mut service = servicesrv::Services::new();
    let mut selector = BTreeMap::new();
    selector.insert(
        servicesrv::RIO_ASM_FAC_ID.to_string(),
        assembly_fac.get_id(),
    );
    if service_type == LOADBALANCER {
        selector.insert("Loadbalancer_provider".to_string(), "vulcand".to_string());
        selector.insert(
            "Loadbalancer_provider_image".to_string(),
            "/v1/plan/vulcand".to_string(),
        );
        service.set_spec(servicesrv::Spec::new(
            selector,
            service_type,
            "",
            BTreeMap::new(),
            BTreeMap::new(),
        ));
        service.set_status(Status::with_conditions(
            INITIALIZING,
            NEW_REPLICA_INITALIZING,
            "",
            SERVICE_LB_INITIAL_CONDITIONS
                .iter()
                .map(|x| Condition::with_type("", "", "False", "", "", x))
                .collect::<Vec<_>>(),
        ));
    } else {
        selector.insert("Dns_provider".to_string(), "powerdns".to_string());
        selector.insert(
            "Dns_provider_image".to_string(),
            "/v1/plan/powerdns".to_string(),
        );
        let mut name = BTreeMap::new();
        name.insert(assembly_fac.get_id(), assembly_fac.get_name());
        service.set_spec(servicesrv::Spec::new(
            selector,
            service_type,
            "",
            name,
            BTreeMap::new(),
        ));
        service.set_status(Status::with_conditions(
            INITIALIZING,
            NEW_REPLICA_INITALIZING,
            "",
            SERVICE_DNS_INITIAL_CONDITIONS
                .iter()
                .map(|x| Condition::with_type("", "", "False", "", "", x))
                .collect::<Vec<_>>(),
        ));
    }
    service.set_type_meta(TypeMeta::new(SERVICE));
    service

}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ObjectMeta {
    name: String,
    origin: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
    owner_references: Vec<OwnerReferences>,
}

impl ObjectMeta {
    pub fn new() -> ObjectMeta {
        ::std::default::Default::default()
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn set_origin(&mut self, v: ::std::string::String) {
        self.origin = v;
    }
    pub fn get_origin(&self) -> ::std::string::String {
        self.origin.clone()
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn set_cluster_name(&mut self, v: ::std::string::String) {
        self.cluster_name = v;
    }
    pub fn set_labels(&mut self, v: BTreeMap<String, String>) {
        self.labels = v;
    }
    pub fn get_labels(&self) -> &BTreeMap<String, String> {
        &self.labels
    }
    pub fn set_annotations(&mut self, v: BTreeMap<String, String>) {
        self.annotations = v;
    }

    pub fn set_owner_references(&mut self, v: Vec<OwnerReferences>) {
        self.owner_references = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OwnerReferences {
    kind: String,
    api_version: String,
    name: String,
    uid: String,
    block_owner_deletion: bool,
}

impl OwnerReferences {
    pub fn new() -> OwnerReferences {
        ::std::default::Default::default()
    }
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = v;
    }
    pub fn set_api_version(&mut self, v: ::std::string::String) {
        self.api_version = v;
    }
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = v;
    }
    pub fn set_block_owner_deletion(&mut self, v: bool) {
        self.block_owner_deletion = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Properties {
    domain: String,
    cloudsetting: String,
    region: String,
    storage_type: String,
}

impl Properties {
    pub fn get_region(&self) -> ::std::string::String {
        self.region.clone()
    }

    pub fn new(domain: &str, cloudsetting: &str, region: &str, storage_type: &str) -> Properties {
        Properties {
            domain: domain.to_string(),
            cloudsetting: cloudsetting.to_string(),
            region: region.to_string(),
            storage_type: storage_type.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TypeMeta {
    kind: String,
    api_version: String,
}

impl TypeMeta {
    pub fn new(kind: &str) -> TypeMeta {
        TypeMeta {
            kind: kind.to_string(),
            api_version: DEFAULT_API_VERSION.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct OpsSettings {
    nodeselector: String,
    priority: String,
    nodename: String,
    restartpolicy: String,
}

impl OpsSettings {
    pub fn new(nodeselector: &str, priority: &str, nodename: &str, restartpolicy: &str) -> OpsSettings {
        OpsSettings {
            nodeselector: nodeselector.to_string(),
            priority: priority.to_string(),
            nodename: nodename.to_string(),
            restartpolicy: restartpolicy.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AssemblyFactoryGetResponse {
    // message fields
    kind: String,
    api_version: String,
    items: Vec<AssemblyFactory>,
}


impl AssemblyFactoryGetResponse {
    pub fn new() -> AssemblyFactoryGetResponse {
        ::std::default::Default::default()
    }

    // Param is passed by value, moved
    pub fn set_assemblys_factory(&mut self, v: Vec<AssemblyFactory>) {
        self.items = v;
        self.kind = ASSEMBLYFACTORYLIST.to_string();
        self.api_version = DEFAULT_API_VERSION.to_string();
    }

    pub fn get_items(&self) -> Vec<AssemblyFactory> {
        self.items.clone()
    }
}
