use super::super::error::{Error, Result};
use protocol::api::base::IdGet;
use rbac::permissions;
use rbac::{account, teams};
use protocol::api::session;
use rbac::trust_access::{TrustedAccess, TrustAccess};
use auth::models::policy_members;


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
    pub team_id: String,
    pub org_id: String,
    pub account_id: String,
}

impl AccountType {
    pub fn new(name: String, account: AccountNames, team_id: String, org_id: String, account_id: String) -> Self {
        AccountType {
            name: name,
            account: account,
            team_id: team_id,
            org_id: org_id,
            account_id: account_id,
        }
    }

    pub fn get_team_id(&self) -> String {
        self.team_id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_org_id(&self) -> String {
        self.org_id.clone()
    }

    pub fn get_account_id(&self) -> String {
        self.account_id.clone()
    }
}

//Authorization is called from middleware.rs to verify the access of user or serviceaccount
#[derive(Clone)]
pub struct Authorization {
    permissions: permissions::Permissions,
    accounts: account::AccountsFascade,
    service_accounts: account::ServiceAccountsFascade,
    teams: teams::TeamsFascade,
}

impl Authorization {
    pub fn new(permissions: permissions::Permissions, accounts: account::AccountsFascade, service_accounts: account::ServiceAccountsFascade, teams: teams::TeamsFascade) -> Self {
        Authorization {
            permissions: permissions,
            accounts: accounts,
            service_accounts: service_accounts,
            teams: teams,
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
                 account_get.set_email(account_type.get_name());
                 let mut account = self.accounts.get_by_email(account_get);
                 //account.pop()                            
                 /*if (account.get_is_admin()) {
                    return Ok(true);
                 }
                 return Err(Error::PermissionError(format!(
                    "User doesn't have permission for this operation."
                )))*/
            

                //let team = self.teams.get_by_id(IdGet::with_id(account_type.get_team_id()));

                return Ok(true);
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
        

       /* let perms_for_account = self.permissions.list_by_team(IdGet::with_id(account.to_string()));
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
