// Copyright (c) 2017 RioCorp Inc.

pub const DEFAULT_API_VERSION: &'static str = "v1";
pub const RIO_ASM_FAC_ID: &'static str = "rioos_assembly_factory_id";


pub const ASSEMBLY: &'static str = "Assembly";
pub const ASSEMBLYFACTORY: &'static str = "AssemblyFactory";
pub const ASSEMBLYLIST: &'static str = "AssemblyList";
pub const ASSEMBLYFACTORYLIST: &'static str = "AssemblyFactoryList";

pub const ROLESLIST: &'static str = "RolesList";
pub const PERMISSIONSLIST: &'static str = "PermissionsList";

pub const ORIGIN: &'static str = "Origin";
pub const ORIGINSLIST: &'static str = "OriginsList";


pub const JOB: &'static str = "Job";
pub const JOBSLIST: &'static str = "JobsList";

pub const NETWORKS: &'static str = "Networks";
pub const NETWORKLIST: &'static str = "NetworkList";

pub const NODE: &'static str = "Node";
pub const NODELIST: &'static str = "NodeList";
pub const PLANLIST: &'static str = "PlanList";
pub const HORIZONTALSCALINGLIST: &'static str = "HorizontalPodAutoscalerList";
pub const ASSEMBLYMETRICLIST: &'static str = "AssemblyMetricList";

pub const SECRETLIST: &'static str = "SecretList";
pub const SECRET: &'static str = "Secret";

pub const SERVICE: &'static str = "Service";
pub const SERVICELIST: &'static str = "ServiceList";

pub const ENDPOINTSLIST: &'static str = "EndpointsList";
pub const ENDPOINT: &'static str = "Endpoint";
pub const SERVICE_ACCOUNT: &'static str = "ServiceAccount";
pub const SERVICEACCOUNTLIST: &'static str = "ServiceAccountsList";
pub const SAMLLIST: &'static str = "SamlProviderList";
pub const OPENIDLIST: &'static str = "OidcProviderList";

pub const STORAGE: &'static str = "Storage";
pub const DATACENTER: &'static str = "DataCenter";
pub const STORAGEPOOL: &'static str = "StoragePool";
pub const STORAGELIST: &'static str = "StorageList";
pub const STOARGEPOOLLIST: &'static str = "StoragePoolList";
pub const DATACENTERLIST: &'static str = "DatacenterList";

pub const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";


pub const INITIAL_CONDITIONS: &'static [&'static str] = &["AssemblyStorageReady", "AssemblyNetworkReady"];
pub const NEW_REPLICA_INITALIZING: &'static str = "Initializing replica ";
pub const ASSEMBLYS_URI: &'static str = "v1/assembly";
pub const INITIALIZING: &'static str = "Initializing";
