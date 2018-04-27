use std::sync::Arc;
use serde_json;

use protocol::api::base::IdGet;
use protocol::api::authorize::Permissions;
use protocol::cache::{PULL_DIRECTLY, NewCacheServiceFn, CACHE_PREFIX_PERMISSION};
use protocol::cache::inject::PermissionFeeder;

use db::data_store::DataStoreConn;
use super::roles::Roles;
use rbac::roles::TrustAccess;
use rbac::ExpanderSender;
use super::super::error::{Result, Error};

use auth::models::permission::DataStore;

// role type to get the permission from database
#[derive(Clone)]
pub struct RoleType {
    pub name: IdGet,
}

impl RoleType {
    pub fn new(name: String) -> Self {
        RoleType { name: IdGet::with_id(name) }
    }
}

//Authorization is called from middleware.rs to verify the access of user or serviceaccount
pub struct Authorization {
    role_type: RoleType,
    ds: Box<DataStoreConn>,
    permissions: Option<Vec<Permissions>>,
}

impl Authorization {
    pub fn new(ds: Arc<DataStoreConn>, role_type: RoleType) -> Self {
        Authorization {
            role_type: role_type,
            ds: Box::new((*ds).clone()),
            permissions: None,
        }
    }

    pub fn set_permissions(&mut self, v: Option<Vec<Permissions>>) {
        self.permissions = v;
    }

    pub fn verify(mut self, trusted: String) -> Result<bool> {
        self.with_cache();
        let conn = self.ds.clone();
        conn.expander.with_permission(&mut self, PULL_DIRECTLY);
        match Roles::per_type(self.permissions) {
            Ok(data) => {
                let access = TrustAccess::new(trusted);
                access.is_allowed(data)
            }
            Err(err) => Err(Error::PermissionError(format!("{}", err))),
        }
    }
}

impl PermissionFeeder for Authorization {
    fn p_get_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.role_type.name.get_id(), "".to_string())
    }

    fn p_feed(&mut self, m: Option<Vec<Permissions>>) {
        self.set_permissions(m);
    }
}

impl ExpanderSender for Authorization {
    fn with_cache(&mut self) {
        let _conn = self.ds.clone();
        let permission_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_PERMISSION.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                DataStore::list_by_name(&_conn, &id).ok().and_then(|p| {
                    serde_json::to_string(&p).ok()
                })
            }),
        ));
        &self.ds.expander.with(permission_service);
    }
}
