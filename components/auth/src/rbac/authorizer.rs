use super::super::error::{Error, Result};
use protocol::api::base::IdGet;
use rbac::permissions;
use rbac::roles::{Roles, TrustAccess};

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
#[derive(Clone)]
pub struct Authorization {
    permissions: permissions::Permissions,
}

impl Authorization {
    pub fn new(permissions: permissions::Permissions) -> Self {
        Authorization { permissions: permissions }
    }

    pub fn verify(self, role_type: RoleType, incoming_to_trust: String) -> Result<bool> {
        let perms_for_account = self.permissions.list_by_email(role_type.name);

        match Roles::per_type(perms_for_account.get_permissions()) {
            Ok(perm_for_account) => {
                let access = TrustAccess::new(incoming_to_trust);
                access.is_allowed(perm_for_account)
            }
            Err(err) => Err(Error::PermissionError(format!("{}", err))),
        }
    }
}
