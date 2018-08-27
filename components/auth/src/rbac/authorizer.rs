use super::super::error::{Error, Result};
use protocol::api::base::IdGet;
use rbac::permissions;
use rbac::account;
use protocol::api::session;
use rbac::teams::{Teams, TrustAccess};


#[derive(Clone, Debug)]
pub enum AccountNames {
    //-----future purpose for team based authentication
    USERACCOUNT,
    SERVICEACCOUNT,
    NONE,
}

// team type to get the permission from database
#[derive(Clone, Debug)]
pub struct AccountType {
    pub name: String,
    pub account: AccountNames,
}

impl AccountType {
    pub fn new(name: String, account: AccountNames) -> Self {
        AccountType {
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
    //first it gets account/serviceaccount teams from cache.
    //If team type does't match then it returns NONE response.
    //And get permissions by team name and verify it.
    //Now we assume account/service_account has only one team.
    //In future we could extend it.
    pub fn verify(self, account_type: AccountType, incoming_to_trust: String) -> Result<bool> {
        match account_type.account {
             AccountNames::USERACCOUNT => {
                 let mut account_get = session::AccountGet::new();
                 account_get.set_email(account_type.name);
                 let mut account = self.accounts.get_by_email(account_get);
                 //account.pop()                 
                 if (account.get_is_admin()) {
                    return Ok(true);
                 }
                 return Err(Error::PermissionError(format!(
                    "User doesn't have permission for this operation."
                )))
             },
            AccountNames::SERVICEACCOUNT => {
                let mut account = self.service_accounts.get_by_name(IdGet::with_id(account_type.name));
                //account.pop()
                Ok(true)
            },
            AccountNames::NONE => {
                info!("« Authorizer verify {:?}", account_type.account);
                Ok(false)
            }
        }

     /*   let account = match account_box {
            Some(r) => r,
            None => {
                info!("« Authorizer Team none : {:?}", account_box);
                return Err(Error::PermissionError(format!(
                "User doesn't have permission for this operation."
            )))
            },
        };
        let perms_for_account = self.permissions.list_by_team(IdGet::with_id(account.to_string()));
        match Teams::per_type(perms_for_account.get_permissions()) {
            Ok(perm_for_account) => {
                let access = TrustAccess::new(incoming_to_trust);
                access.is_allowed(perm_for_account)
            }
            Err(err) => {
                info!("« Authorizer get none permissions : {:?}", perms_for_account.get_permissions());
                info!("« Authorizer team : {}", team.to_string());
                Err(Error::PermissionError(format!("{}", err)))
            },
        }*/
    }
}
