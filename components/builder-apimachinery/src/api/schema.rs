use iron::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use api::base::TypeMeta;

use error::{self, Result};

//
lazy_static! {
    static ref DISPATCH_TABLE: DispatchTable = {
        let mut map = DispatchTable::new();
        map.register("POST:accounts", "Account");

        map.register("POST:assemblyfactorys", "AssemblyFactory");
        map.register("GET:assemblyfactorys", "AssemblyFactoryList");

        map.register("POST:stacksfactorys", "StacksFactory");
        map.register("GET:stacksfactorys", "StacksFactoryList");

        map.register("POST:plans", "PlanFactory");
        map.register("GET:plans", "PlanFactoryList");

        map.register("POST:assemblys", "Assembly");
        map.register("GET:assemblys", "AssemblyList");
        map.register("GET:assemblyfactorysdescribe", "AssemblyList");
        map.register("GET:stacksfactorysdescribe", "AssemblyFactoryList");

        map.register("POST:nodes", "Node");
        map.register("GET:nodes", "NodeList");
        map.register("POST:nodesdiscover", "NodeList");

        map.register("POST:senseis", "Sensei");
        map.register("GET:senseis", "SenseiList");

        map.register("POST:licenseactivate", "License");
        map.register("GET:licenses", "LicenseList");

        map.register("POST:origins", "Origin");
        map.register("GET:origins", "OriginList");

        map.register("POST:storageconnectors", "Storage");
        map.register("GET:storageconnectors", "StorageList");

        map.register("POST:storagespool", "StoragePool");
        map.register("GET:storagespool", "StoragePoolList");

        map.register("POST:datacenters", "Datacenter");
        map.register("GET:datacenters", "DatacenterList");

        map.register("POST:jobs", "Job");
        map.register("GET:jobs", "JobList");
        map.register("GET:jobsnode", "JobList");

        map.register("POST:networks", "Network");
        map.register("GET:networks", "NetworkList");

        map.register("POST:services", "Service");
        map.register("GET:services", "ServiceList");

        map.register("POST:endpoints", "Endpoints");
        map.register("GET:endpoints", "EndPointList");
        map.register("GET:endpointsassembly", "EndPointList");

        map.register("POST:serviceaccounts", "ServiceAccount");
        map.register("PUT:serviceaccountsorigins", "ServiceAccount");

        map.register("GET:serviceaccountsorigins", "ServiceAccount");
        map.register("GET:serviceaccounts", "ServiceAccountList");

        map.register("GET:roles", "RoleList");
        map.register("GET:permissions", "PermissionList");
        map.register("GET:permissionsroles", "PermissionList");

        map.register("POST:volumes", "Volume");
        map.register("GET:volumesassemblys", "VolumeList");

        map.register("POST:horizontalscaling", "HorizontalScaling");
        map.register("GET:horizontalscaling", "HorizontalScalingList");
        map.register("GET:horizontalscalingmetrics", "ScalingMetricList");

        map.register("POST:teams", "Team");

        map.register("POST:verticalscaling", "VerticalScaling");
        map.register("GET:verticalscaling", "VerticalScalingList");
        map.register("GET:verticalscalingmetrics", "ScalingMetricList");

        map.register("POST:secrets", "Secret");
        map.register("POST:secretsorigins", "Secret");

        map.register("GET:secrets", "SecretList");
        map.register("GET:secretsorigins", "SecretList");
        map.register("GET:secretsall", "SecretList");

        map.register("POST:settingsmap", "SettingsMap");

        map.register("GET:marketplaces", "MarketplaceList");
        map.register("POST:marketplaces", "PlanFactory");

        map.register("GET:logs", "LogList");

        map.register("POST:accountsaudits", "Event");
        map.register("GET:accountsaudits", "EventList");

        map.register("GET:imagevulnerablity", "Image");

        map.register("POST:buildconfigs", "BuildConfig");
        map.register("GET:buildconfigs", "BuildConfigList");

        map.register("POST:builds", "Build");
        map.register("GET:builds", "BuildList");

        map.register("POST:imagereferences", "ImageReference");
        map.register("GET:imagereferences", "ImageReferenceList");

        map.register("POST:imagemarks", "ImageMark");
        map.register("GET:imagemarks", "ImageMarkList");
        map.register("GET:imagemarksbuilds", "ImageMarkList");

        map.register("POST:packages", "Package");
        map.register("GET:authoidcproviders", "OpenidList");
        map.register("GET:authsamlproviders", "SamlList");

        map.register("POST:accountsbuckets", "Bucket");
        map.register("GET:accountsbuckets", "BucketList");
        map.register("POST:buckets", "Bucket");
        map.register("GET:buckets", "BucketList");

        map.register("POST:accountsbucketsfilesupload", "BucketAccessor");
        map.register("GET:accountsbucketsfilesdownload", "BucketAccessor");
        map.register("POST:bucketsfilesupload", "BucketAccessor");
        map.register("GET:bucketsfilesdownload", "BucketAccessor");

        map.register("POST:ingresses", "Ingress");
        map.register("GET:ingresses", "IngressList");

        map.register("GET:wizards", "Wizard");

        map
    };
}

// Information schema for `rioos`.
#[derive(Debug)]
pub struct ApiSchema {
    pub version: String,
    pub kind: String,
}

impl Default for ApiSchema {
    fn default() -> Self {
        ApiSchema {
            version: "".to_string(),
            kind: "None".to_string(),
        }
    }
}

struct SchemaRegistry;

/// The schema registry that keeps track of the supported api by the server.
/// This registry loades from the configuration from swagger.yaml when the server starts.
// / The current version of schema to use is available by calling current() method.
impl SchemaRegistry {
    fn _load() -> Result<()> {
        Ok(())
    }

    //The current version of the schema.
    fn current() -> Box<Dispatcher> {
        Box::new(V1Schema)
    }

    //The manually loaded DispatcherTable. Ideally this will be from the data got by calling
    //load() method.
    fn loaded() -> &'static DispatchTable {
        &DISPATCH_TABLE
    }
}

/// Dispatchers connect to Message Queue Servers
pub trait Dispatcher: Send + 'static {
    //const VERSION: &'static str;

    /// Returns a function dispatch table mapping which maps which protocol message is handled
    /// by which `Handler`.
    fn dispatch_table(&self) -> &'static DispatchTable;
}

struct V1Schema;

impl Dispatcher for V1Schema {
    //const VERSION: &'static str = "v1";

    fn dispatch_table(&self) -> &'static DispatchTable {
        SchemaRegistry::loaded()
    }
}

impl Default for V1Schema {
    fn default() -> Self {
        V1Schema {}
    }
}

pub struct DispatchTable(HashMap<&'static str, &'static str>);

impl DispatchTable {
    pub fn new() -> Self {
        DispatchTable(HashMap::new())
    }

    /// Returns a `Kind` for the given group.
    pub fn get(&self, group: &str) -> Option<&&str> {
        self.0.get(group)
    }

    /// Registers a group to a given `Kind`.
    pub fn register(&mut self, group: &'static str, kind: &'static str) {
        if self.0.insert(group, kind).is_some() {
            panic!(
                "Attempted to register a second kind {} for group, '{}'",
                kind, group,
            );
        }
    }
}

/// A private helper function for finding the requests url ApiSchema
pub fn dispatch_url(group: String) -> ApiSchema {
    match SchemaRegistry::current().dispatch_table().get(&group) {
        Some(kind) => ApiSchema {
            version: "v1".to_string(), //Todo  later
            kind: kind.to_string(),
        },
        None => {
            warn!("dispatch, unknown kind, {}", group);
            ApiSchema::default()
        }
    }
}

/// Helper function for finding the requests ApiSchema
pub fn dispatch(req: &mut Request) -> ApiSchema {
    dispatch_url(parse_schema_url(req))
}

//Helper function for parse url path and finding api schemas
pub fn parse_schema_url(req: &mut Request) -> String {
    let mut url: String = req.url.path().into_iter().collect();
    if req.url.path().len() > 2 {
        url = format!("{}{}", req.url.path()[0], req.url.path()[2]);
    }
    format!("{}:{}", req.method, url.replace(char::is_numeric, ""))
}

/// Helper function for finding the requests type_meta
/// Input Request
/// Returns type_meta
pub fn type_meta(req: &mut Request) -> TypeMeta {
    dispatch(req).into()
}

/// Helper function for finding the requests url type_meta
/// Input request url
/// Returns TypeMeta
pub fn type_meta_url(url: String) -> TypeMeta {
    dispatch_url(url).into()
}

impl Into<TypeMeta> for ApiSchema {
    fn into(self) -> TypeMeta {
        TypeMeta::with(self.kind, self.version)
    }
}

struct DispatchUrl {
    method: String,
    url: String,
}

impl FromStr for DispatchUrl {
    type Err = error::Error;

    fn from_str(value: &str) -> Result<Self> {
        let items: Vec<&str> = value.split(":").collect();
        let (method, url) = match items.len() {
            2 => (items[0], items[1]),
            _ => return Err(error::Error::RequiredConfigField(value.to_string().clone())),
        };

        Ok(DispatchUrl {
            method: method.to_string(),
            url: url.to_string(),
        })
    }
}

impl fmt::Display for DispatchUrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.method, self.url)
    }
}
