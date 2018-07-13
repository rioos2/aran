use super::super::error::{Error, Result};
use protocol::api::base::IdGet;
use rbac::permissions;
use rbac::account;
use protocol::api::session;
use rbac::roles::{Roles, TrustAccess};

#[derive(Clone, Debug)]
pub enum RoleNames {
    USERACCOUNT,
    SERVICEACCOUNT,
    NONE,
}

// role type to get the permission from database
#[derive(Clone, Debug)]
pub struct RoleType {
    pub name: String,
    pub account: RoleNames,
}

impl RoleType {
    pub fn new(name: String, account: RoleNames) -> Self {
        RoleType {
            name: name,
            account: account,
        }
    }
}

//Authorization is called from middleware.rs to verify the access of user or serviceaccount
#[derive(Clone)]
pub struct Authorization {
    permissions: permissions::Permissions,
    accounts: account::AccountsFascade,
    service_accounts: account::ServiceAccountsFascade,
}

impl Authorization {
    pub fn new(permissions: permissions::Permissions, accounts: account::AccountsFascade, service_accounts: account::ServiceAccountsFascade) -> Self {
        Authorization {
            permissions: permissions,
            accounts: accounts,
            service_accounts: service_accounts,
        }
    }

    //verify method verifies account/service_account have accesibility of requested operation
    //first it gets account/serviceaccount roles from cache.
    //If role type does't match then it returns NONE response.
    //And get permissions by role name and verify it.
    //Now we assume account/service_account has only one role.
    //In future we could extend it.
    pub fn verify(self, role_type: RoleType, incoming_to_trust: String) -> Result<bool> {   
        let role_box: Option<String> = match role_type.account {
            RoleNames::USERACCOUNT => {
                let mut account_get = session::AccountGet::new();
                account_get.set_email(role_type.name);
                let mut account = self.accounts.get_by_email(account_get).get_roles();       
                account.pop()
            },
            RoleNames::SERVICEACCOUNT => {
                let mut account = self.service_accounts.get_by_name(IdGet::with_id(role_type.name)).get_roles();
                account.pop()
            },
            RoleNames::NONE => {
                None
            }
        };                

        let role = match role_box {
            Some(r) => r,
            None => return Err(Error::PermissionError(format!(
                "User doesn't have permission for this operation."
            ))),
        };
        let perms_for_account = self.permissions.list_by_role(IdGet::with_id(role.to_string()));
        match Roles::per_type(perms_for_account.get_permissions()) {
            Ok(perm_for_account) => {
                let access = TrustAccess::new(incoming_to_trust);
                access.is_allowed(perm_for_account)
            }
            Err(err) => Err(Error::PermissionError(format!("{}", err))),
        }
    }
}
