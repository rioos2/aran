// Copyright 2018 The Rio Advancement Inc

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use PolicyMembersOutputList;
use models::policy_members;
use super::{InvitationsOutputList, TeamMembersOutput};
use protocol::api::authorize::PolicyMembersList;
use protocol::api::schema::type_meta_url;
use protocol::api::base::{MetaFields, ObjectMeta};
use std::collections::BTreeMap;
use protocol::api::base::ChildTypeMeta;
use protocol::api::base::IdGet;
use protocol::api::authorize::{PolicyMembers, PolicyMemberInputs};

/// This struct used for insert new entry into policy_members table
pub struct PolicyFactory<'a> {
    conn: &'a DataStoreConn,
}

impl<'a> PolicyFactory<'a> { 

	pub fn new(conn: &'a DataStoreConn) -> PolicyFactory<'a> {
        PolicyFactory {
            conn: conn,
        }
    }
   
    /// Make policy member from policymember inputs and 
    /// store policy members details into postgres
    pub fn mk_policy_members(&self, list: &PolicyMemberInputs) -> PolicyMembersOutputList {
    	let member_list = self.mk_members(&list.clone());
    	let mk_policy_members = member_list.policies
                        .into_iter()
                        .map(|member| {
                            policy_members::DataStore::new(&self.conn).create(&member)
                        })
                        .fold(vec![], |mut acc, x| match x {
                            Ok(one_policy) => {                            	
                            	match one_policy {
                            		Some(res) => {
                            			acc.push(res);
                               			acc
                            		}
                            		None => acc,
                            	}
                                
                            }
                            Err(_) => acc,
                        });
        Ok(Some(mk_policy_members))
    }

    /// To update policy members into postgres
    pub fn update_policy_members(&self, list: &PolicyMembersList) -> PolicyMembersOutputList {
        let mk_policy_members = list.policies.clone()
                        .into_iter()
                        .map(|member| {
                            policy_members::DataStore::new(&self.conn).update(&member)
                        })
                        .fold(vec![], |mut acc, x| match x {
                            Ok(one_policy) => {                             
                                match one_policy {
                                    Some(res) => {
                                        acc.push(res);
                                        acc
                                    }
                                    None => acc,
                                }
                                
                            }
                            Err(_) => acc,
                        });
        Ok(Some(mk_policy_members))
    }

    /// make policymember from PolicyMemberInputs data
    /// set is_allow = true for allowed policies only
    /// otherwise set is_allow = false 
    fn mk_members(&self, inputs: &PolicyMemberInputs) -> PolicyMembersList {
        let allowed_policies = inputs.get_allowed_policies();
        let mut allowed: Vec<PolicyMembers> = allowed_policies.into_iter().map(|policy_name| {
            self.build_member("true".to_string(), &inputs.clone(), policy_name)
        }).collect();

        let denied_policies = inputs.get_denied_policies();
        let denied: Vec<PolicyMembers> = denied_policies.into_iter().map(|policy_name| {
            self.build_member("false".to_string(), &inputs.clone(), policy_name)
        }).collect();

        //merge allowed and denied policy_members vecs
        allowed.extend(denied);
        
        PolicyMembersList{
            policies: allowed
        }
    }

    fn build_member(&self, is_allow: String, inputs: &PolicyMemberInputs, policy_name: String) -> PolicyMembers {  
        let mut policy_members = PolicyMembers::new();
        let m = policy_members.mut_meta(
            ObjectMeta::new(),
            "POLICYMEMBERS".to_string(),
            inputs.get_account_id(),
        );
        let jackie = inputs.children();
        policy_members.set_meta(type_meta_url(jackie), m);

        let mut b = BTreeMap::new();
        b.insert("origin".to_string(), inputs.get_origin_id());
        b.insert("team".to_string(), inputs.get_team_id());

        policy_members.set_is_allow(is_allow);
        policy_members.set_metadata(b);
        policy_members.set_policy_name(policy_name);
        policy_members       
    }
       

}