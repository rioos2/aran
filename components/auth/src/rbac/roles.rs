use super::super::error::{Result, Error};
use db::data_store::DataStoreConn;
use rbac::authorizer::RoleType;
use iron::prelude::IronResult;
use auth::models::permission::DataStore;
use protocol::api::authorize::Permissions;

type TrustedAccessList = Vec<TrustAccess>;

const ALL: &'static str = "*";
const ASSEMBLY: &'static str = "assembly";
const ASSEMBLYFACTORY: &'static str = "assemblyfactory";
//const SERVICEACCOUNT: &'static str = "serviceaccount";
const HORIZONTALSCALING: &'static str = "horizontalscaling";
const VERTICALSCALING: &'static str = "verticalscaling";
const SECRET: &'static str = "secret";
const ENDPOINT: &'static str = "endpoint";
const JOB: &'static str = "job";
const SERVICE: &'static str = "service";
const VOLUME: &'static str = "volume";
const NODE: &'static str = "node";
const STORAGECONNECTOR: &'static str = "storageconnector";
const STORAGEPOOL: &'static str = "storagepool";
//const SETTINGSMAP: &'static str = "settingsmap";
const IMAGEREFERENCE: &'static str = "imagereference";
const IMAGEMARK: &'static str = "imagemark";
const BUILD: &'static str = "build";
const BUILDCONFIG: &'static str = "buildconfig";
const PLAN: &'static str = "plan";
const ACCOUNT: &'static str = "account";
const DATACENTER: &'static str = "datacenter";
const NETWORK: &'static str = "network";
const AUDIT: &'static str = "audit";
const LOG: &'static str = "log";
const HEALTHZ: &'static str = "healthz";

const RESOURCE_ALL: &'static str = "*";
const RESOURCE_GET: &'static str = "get";
const RESOURCE_POST: &'static str = "post";
const RESOURCE_PUT: &'static str = "put";
const RESOURCE_DELETE: &'static str = "delete";

pub struct Roles {}

impl Roles {
    pub fn per_type(role_type: RoleType, ds: &DataStoreConn) -> Result<TrustedAccessList> {
        match DataStore::list_by_name(ds, &role_type.name, &role_type.stored_procedure_name) {
            Ok(Some(perm)) => Ok(perm.iter().map(|x| x.clone().into()).collect::<Vec<_>>()),
            Ok(None) => Err(Error::PermissionError(format!("No Recored Found"))),
            Err(err) => Err(Error::PermissionError(format!("{}", err))),
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
        for p in perms.iter() {
            if p == self {                
                flag = true;
            }
        }
        match flag {
            true => Ok(flag),
            false => Err(Error::PermissionError(format!("User doesn't have permission for this operation."))),
        }
    }

}

//PartialEq for TrustAccess enumeration
//when requested account resource is wild(*) permission then return true
//otherwise it will check resource and level permissions
impl PartialEq for TrustAccess {
    fn eq(&self, other: &TrustAccess) -> bool {        
        match self.1 {
            TrustLevel::ResourceWild => self.0 == other.0,
            _ => self.0 == other.0 && self.1 == other.1
        }
    }
}

//Resource access for the user
#[derive(Debug, PartialEq, Eq, Clone)]
enum TrustResource {
    Assembly,
    AssemblyFactory,
   // ServiceAccount
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
    //Settingsmap
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
           // SERVICEACCOUNT => TrustResource::ServiceAccount
            HORIZONTALSCALING => TrustResource::HorizontalScaling,
            VERTICALSCALING => TrustResource::VerticalScaling,
            SECRET => TrustResource::Secret,
            ENDPOINT => TrustResource::Endpoint,
            JOB => TrustResource::Job,
            SERVICE => TrustResource::Service,
            VOLUME => TrustResource::Volume,
            NODE =>  TrustResource::Node,
            STORAGECONNECTOR => TrustResource::StorageConnector,
            STORAGEPOOL => TrustResource::Storagepool,
           // SETTINGSMAP => TrustResource::Settingsmap
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
            HEALTHZ => TrustResource::Healthz,
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
