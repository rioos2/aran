use super::super::error::{Result, Error};
use db::data_store::DataStoreConn;
use rbac::authorizer::RoleType;
use iron::prelude::IronResult;
use auth::models::permission::DataStore;
use protocol::api::authorize::Permissions;

type TrustedAccessList = Vec<TrustAccess>;

const ALL: &'static str = "*";
const ASSEMBLY: &'static str = "assembly";
const ASSEMBLY_FACTORY: &'static str = "assemblyfactory";
const PLAN: &'static str = "plan";

const RESOURCE_GET: &'static str = "get";
const RESOURCE_POST: &'static str = "post";
const RESOURCE_PUT: &'static str = "put";

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

#[derive(Debug)]
pub struct TrustAccess(TrustResource, TrustLevel);

impl TrustAccess {
    pub fn new(name: String) -> TrustAccess {
        if name.contains(".") {
            let x = name.split(".").collect::<Vec<_>>();
            return TrustAccess(TrustResource::from_str(x[1]), TrustLevel::from_str(x[2]));
        }
        TrustAccess(TrustResource::Wild, TrustLevel::ResourceWild)
    }

    pub fn is_allowed(&self, perms: TrustedAccessList) -> Result<bool> {      
        let mut flag = false;
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

impl PartialEq for TrustAccess {
    fn eq(&self, other: &TrustAccess) -> bool {
        self.0 == other.0
    }
}

//Resource access for the user
#[derive(Debug, PartialEq, Eq)]
enum TrustResource {
    Assembly,
    AssemblyFactory,
    Plan,
    Wild,
    None,
}

//TrustLevel access for the user
#[derive(Debug)]
enum TrustLevel {
    ResourceGet,
    ResourcePost,
    ResourcePut,
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

impl TrustResource {
    pub fn from_str(value: &str) -> TrustResource {
        match &value[..] {
            ALL => TrustResource::Wild,
            ASSEMBLY => TrustResource::Assembly,
            ASSEMBLY_FACTORY => TrustResource::AssemblyFactory,
            PLAN => TrustResource::Plan,
            _ => TrustResource::None,
        }
    }
}

impl TrustLevel {
    pub fn from_str(value: &str) -> TrustLevel {
        match &value[..] {
            ALL => TrustLevel::ResourceWild,
            RESOURCE_GET => TrustLevel::ResourceGet,
            RESOURCE_POST => TrustLevel::ResourcePost,
            RESOURCE_PUT => TrustLevel::ResourcePut,
            _ => TrustLevel::ResourceNone,
        }
    }
}
