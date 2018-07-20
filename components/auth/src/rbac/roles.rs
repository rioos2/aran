use super::super::error::{Error, Result};
use protocol::api::authorize::Permissions;
use std::fmt;
type TrustedAccessList = Vec<TrustAccess>;

const ALL: &'static str = "*";
const ASSEMBLY: &'static str = "ASSEMBLYS";
const ASSEMBLYFACTORY: &'static str = "ASSEMBLYFACTORYS";
const STACKSFACTORY: &'static str = "STACKSFACTORYS";
const HORIZONTALSCALING: &'static str = "HORIZONTALSCALING";
const VERTICALSCALING: &'static str = "VERTICALSCALING";
const SECRET: &'static str = "SECRETS";
const ENDPOINT: &'static str = "ENDPOINTS";
const JOB: &'static str = "JOBS";
const SERVICE: &'static str = "SERVICES";
const VOLUME: &'static str = "VOLUMES";
const NODE: &'static str = "NODES";
const STORAGECONNECTOR: &'static str = "STORAGECONNECTORS";
const STORAGEPOOL: &'static str = "STORAGESPOOL";
const IMAGEREFERENCE: &'static str = "IMAGEREFERENCES";
const IMAGEMARK: &'static str = "IMAGEMARKS";
const BUILD: &'static str = "BUILDS";
const BUILDCONFIG: &'static str = "BUILDCONFIGS";
const PLAN: &'static str = "PLANS";
const ACCOUNT: &'static str = "ACCOUNTS";
const DATACENTER: &'static str = "DATACENTERS";
const NETWORK: &'static str = "NETWORKS";
const AUDIT: &'static str = "AUDITS";
const LOG: &'static str = "LOGS";
const BUCKET: &'static str = "BUCKETS";
const HEALTHZ: &'static str = "HEALTHZ";
const INGRESSES: &'static str = "INGRESSES";
const SETTINGSMAP: &'static str = "SETTINGSMAP";
const SERVICEACCOUNT: &'static str = "SERVICEACCOUNTS";
const PING: &'static str = "PING";
const SENSEI: &'static str = "SENSEIS";

const RESOURCE_GET: &'static str = "GET";
const RESOURCE_POST: &'static str = "POST";
const RESOURCE_PUT: &'static str = "PUT";
const RESOURCE_DELETE: &'static str = "DELETE";

pub struct Roles {}

impl Roles {
    pub fn per_type(permission: Option<Vec<Permissions>>) -> Result<TrustedAccessList> {
        match permission {
            Some(perm) => Ok(perm.iter().map(|x| x.clone().into()).collect::<Vec<_>>()),
            None => Err(Error::PermissionError(format!("Record Not Found"))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrustAccess(TrustResource, TrustLevel);

impl TrustAccess {
    //create new object of TrustAccess
    //it is split the argument to TrustResource and TrustLevel enums
    //and generate TrustAccess struct
    pub fn new(name: String) -> TrustAccess {
        if name.contains(".") {
            let x = name.split(".").collect::<Vec<_>>();
            return TrustAccess(TrustResource::from_str(x[1]), TrustLevel::from_str(x[2]));
        }
        TrustAccess(TrustResource::Wild, TrustLevel::ResourceWild)
    }

    //this function return true only for super user
    //when requested account resource and level is wild(*) then it is return true
    //otherwise return false(it means requested account is not a super user)
    pub fn is_all_wild(&self, perms: TrustedAccessList) -> bool {
        perms[0].0 == TrustResource::Wild && perms[0].1 == TrustLevel::ResourceWild
    }

    pub fn is_allowed(&self, perms: TrustedAccessList) -> Result<bool> {
        let mut flag = false;
        if perms.len() < 2 && self.is_all_wild(perms.clone()) {
            return Ok(true);
        }

        debug!("Not ALL WILD.");

        for p in perms.iter() {
            if p == self {
                flag = true;
            }
        }
        match flag {
            true => Ok(flag),
            false => {
                Err(Error::PermissionError(format!(
                "User doesn't have permission for this operation."
            )))
            },
        }
    }
}

//PartialEq for TrustAccess enumeration
//when requested account resource is wild(*) permission then return true
//otherwise it will check resource and level permissions
impl PartialEq for TrustAccess {
    fn eq(&self, other: &TrustAccess) -> bool {
        debug!(
            "Comparing [{:?}={:?}] [{:?}={:?}]",
            self.0.clone(),
            other.0.clone(),
            self.1.clone(),
            other.1.clone()
        );
        match self.1 {
            TrustLevel::ResourceWild => self.0 == other.0,
            _ => self.0 == other.0 && self.1 == other.1,
        }
    }
}

//Resource access for the user
#[derive(Debug, PartialEq, Eq, Clone)]
enum TrustResource {
    Assembly,
    AssemblyFactory,
    StacksFactory,
    ServiceAccount,
    HorizontalScaling,
    VerticalScaling,
    Secret,
    Endpoint,
    Job,
    Service,
    Volume,
    Node,
    StorageConnector,
    Storagepool,
    Settingsmap,
    Imagereference,
    Imagemark,
    Build,
    Buildconfig,
    Plan,
    Account,
    Datacenter,
    Network,
    Log,
    Audit,
    Healthz,
    Wild,
    Bucket,
    Ingresses,
    Ping,
    Sensei,
    None,
}

//TrustLevel access for the user
#[derive(Debug, PartialEq, Eq, Clone)]
enum TrustLevel {
    ResourceGet,
    ResourcePost,
    ResourcePut,
    ResourceDelete,
    ResourceWild,
    ResourceNone,
}

//convert the Permissions into TrustAccess
impl Into<TrustAccess> for Permissions {
    fn into(self) -> TrustAccess {
        let name = self.get_name();
        if name.contains(".") {
            let x = name.split(".").collect::<Vec<_>>();
            return TrustAccess(TrustResource::from_str(x[1]), TrustLevel::from_str(x[2]));
        }
        TrustAccess(TrustResource::Wild, TrustLevel::ResourceWild)
    }
}

//convert resource string to TrustResource enum value
impl TrustResource {
    pub fn from_str(value: &str) -> TrustResource {
        match &value[..] {
            ALL => TrustResource::Wild,
            ASSEMBLY => TrustResource::Assembly,
            ASSEMBLYFACTORY => TrustResource::AssemblyFactory,
            STACKSFACTORY=> TrustResource::StacksFactory,
            SERVICEACCOUNT => TrustResource::ServiceAccount,
            HORIZONTALSCALING => TrustResource::HorizontalScaling,
            VERTICALSCALING => TrustResource::VerticalScaling,
            SECRET => TrustResource::Secret,
            ENDPOINT => TrustResource::Endpoint,
            JOB => TrustResource::Job,
            SERVICE => TrustResource::Service,
            VOLUME => TrustResource::Volume,
            NODE => TrustResource::Node,
            STORAGECONNECTOR => TrustResource::StorageConnector,
            STORAGEPOOL => TrustResource::Storagepool,
            SETTINGSMAP => TrustResource::Settingsmap,
            IMAGEREFERENCE => TrustResource::Imagereference,
            IMAGEMARK => TrustResource::Imagemark,
            BUILD => TrustResource::Build,
            BUILDCONFIG => TrustResource::Buildconfig,
            PLAN => TrustResource::Plan,
            ACCOUNT => TrustResource::Account,
            DATACENTER => TrustResource::Datacenter,
            NETWORK => TrustResource::Network,
            AUDIT => TrustResource::Audit,
            LOG => TrustResource::Log,
            BUCKET => TrustResource::Bucket,
            HEALTHZ => TrustResource::Healthz,
            INGRESSES => TrustResource::Ingresses,
            PING => TrustResource::Ping,
            SENSEI =>TrustResource::Sensei,

            _ => TrustResource::None,
        }
    }
}

//convert level string to TrustLevel enum value
impl TrustLevel {
    pub fn from_str(value: &str) -> TrustLevel {
        match &value[..] {
            ALL => TrustLevel::ResourceWild,
            RESOURCE_GET => TrustLevel::ResourceGet,
            RESOURCE_POST => TrustLevel::ResourcePost,
            RESOURCE_PUT => TrustLevel::ResourcePut,
            RESOURCE_DELETE => TrustLevel::ResourceDelete,
            _ => TrustLevel::ResourceNone,
        }
    }
}
