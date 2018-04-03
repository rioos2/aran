use std::sync::Arc;
use iron::prelude::*;

use protocol::api::base::IdGet;
use db::data_store::DataStoreConn;
use super::roles::Roles;

// role type to get the permission from database
#[derive(Clone)]
pub struct RoleType {
    pub name: IdGet,
    pub stored_procedure_name: String,
}

impl RoleType {
    pub fn new(name: String, stored_procedure_name: &str) -> Self {
        RoleType {
            name: IdGet::with_id(name),
            stored_procedure_name: stored_procedure_name.to_string(),
        }
    }
}

//Authorization is called from middleware.rs to verify the access of user or serviceaccount
pub struct Authorization {
    role_type: RoleType,
    ds: Arc<DataStoreConn>,
}

impl Authorization {
    pub fn new(ds: Arc<DataStoreConn>, role_type: RoleType) -> Self {
        Authorization {
            role_type: role_type,
            ds: ds,
        }
    }
    pub fn verify(&self) -> IronResult<()> {
        Roles::per_type(self.role_type.clone(), &self.ds);
        Ok(())
    }
}
