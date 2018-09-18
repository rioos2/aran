// Copyright 2018 The Rio Advancement Inc

use super::super::error::{Error, Result};
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::api::session;
use rbac::{account, teams, policies};
use rbac::permissions;
use rbac::trust_access::{TrustedAccess, TrustAccess};
use rbac::trust_access::TrustedAccessList;
use std;


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
    policies: policies::PolicyFascade,
}

impl Authorization {
    pub fn new(permissions: permissions::Permissions, accounts: account::AccountsFascade, service_accounts: account::ServiceAccountsFascade, teams: teams::TeamsFascade, policies: policies::PolicyFascade) -> Self {
        Authorization {
            permissions: permissions,
            accounts: accounts,
            service_accounts: service_accounts,
            teams: teams,
            policies: policies,
        }
    }

    //verify method verifies account/service_account have accesibility of requested operation
    //first it gets account/serviceaccount teams from cache.
    //If team type does't match then it returns NONE response.
    //And get permissions by team name and verify it.
    //Now we assume account/service_account has only one team.
    //In future we could extend it.
    pub fn verify(self, account_type: AccountType, incoming_to_trust: String) -> Result<bool> {
        let permissions = match account_type.account {
            AccountNames::USERACCOUNT => {
                let mut account_get = session::AccountGet::new();
                account_get.set_email(account_type.get_name());
                let mut account = self.accounts.get_by_email(account_get);
                debug!("« Account is Admin {:?}", account.get_is_admin());
                // check account is admin or not
                // account is admin then return true, because admin has full control
                // otherwise we check other permissions
                if account.get_is_admin() {
                    return Ok(true);
                }
                debug!(
                    "« Account is Normal User Authorization Start {:?}",
                    account.get_is_admin()
                );
                self.collect_permissions(self.collect_policies(
                    Some("default".to_string()),
                    account_type.get_team_id(),
                )?)
            }
            AccountNames::SERVICEACCOUNT => {
                self.collect_permissions(self.collect_policies(
                    Some("serviceaccount".to_string()),
                    "".to_string(),
                )?)
            }
            AccountNames::NONE => {
                info!("« Authorizer verify {:?}", account_type.account);
                Err(Error::PermissionError(format!(
                    "Authorizer doesn't match : {:?}",
                    account_type.account
                )))
            }
        };

        match permissions {
            Ok(perm_for_account) => {
                let access = TrustAccess::new(incoming_to_trust);
                access.is_allowed(perm_for_account)
            }
            Err(err) => {
                info!("« Authorizer get none permissions");
                info!("« Authorizer team : {}", account_type.get_team_id());
                Err(Error::PermissionError(format!("{}", err)))
            }
        }
    }

    /// this collect all policies using team and default level
    /// first collect level based policies from cache
    /// and collect policies by requested team id
    /// finally merge all collected policies and return it
    fn collect_policies(&self, level: Option<String>, team_id: String) -> Result<Vec<String>> {

        let mut policy_by_level = match level {
            Some(lev) => {
                let pols = match self.policies
                    .list_by_level(IdGet::with_id(lev))
                    .get_policies() {
                    Some(pol) => pol.iter().map(|x| x.object_meta().name).collect::<Vec<_>>(),
                    None => std::vec::Vec::new(),
                };
                pols
            }
            None => std::vec::Vec::new(),
        };


        let mut policy_by_team = std::vec::Vec::new();
        if !team_id.is_empty() {
            let team = self.teams.get_by_id(IdGet::with_id(team_id));
            match team.get_policies() {
                Some(pol) => {
                    for p in &pol {
                        if p.get_is_allow() == "true" {
                            policy_by_team.push(p.get_policy_name());
                        }
                    }
                }
                None => {}
            }
        }

        policy_by_level.extend(policy_by_team);

        Ok(policy_by_level)

    }

    /// this collect permissions by policy name
    /// and convert to trustedaccess structure for each permissions
    /// this for using auth verification
    /// ex . TrustResource::Machine, TrustResource::Container
    fn collect_permissions(&self, policies: Vec<String>) -> Result<TrustedAccessList> {
        if policies.is_empty() {
            return Err(Error::PermissionError(
                format!("{}", "Authorizer get none permissions".to_string()),
            ));
        }
        let perms_list = policies
            .into_iter()
            .map(|policy| {
                self.permissions.list_by_policy(IdGet::with_id(policy))
            })
            .fold(vec![], |mut acc, x| match TrustedAccess::per_type(
                x.get_permissions(),
            ) {
                Ok(perm_for_policy) => {
                    acc.extend(perm_for_policy);
                    acc
                }
                Err(_) => acc,
            });

        Ok(perms_list)
    }
}
