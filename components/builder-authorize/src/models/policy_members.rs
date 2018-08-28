// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::PolicyMembers;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_INVALDATED, PULL_DIRECTLY};
use super::super::{PolicyMembersOutput, PolicyMembersOutputList};
use db::data_store::DataStoreConn;
use postgres;
use serde_json;

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
    expander: &'a InMemoryExpander,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore {
            db: db,
            expander: &db.expander,
        }
    }

    pub fn create(&self, members: &PolicyMembers) -> PolicyMembersOutput {
        let conn = self.db.pool.get_shard(0)?;
        
        let rows = &conn.query(
            "SELECT * FROM insert_policy_member_v1 ($1,$2,$3,$4,$5)",
            &[
                &(members.get_policy_name() as String),  
                &(members.get_is_allow() as String),             
                &(serde_json::to_value(members.object_meta()).unwrap()),
                &(serde_json::to_value(members.type_meta()).unwrap()),
                &(serde_json::to_value(members.get_metadata()).unwrap()),                
            ],
        ).map_err(Error::PolicyMembersCreate)?;

       if rows.len() > 0 {
            for row in rows {
                let member = self.merge_permissions(&row, PULL_DIRECTLY)?;
                return Ok(Some(member));
            }
        }
        Ok(None)
    }

    pub fn update(&self, members: &PolicyMembers) -> PolicyMembersOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_policy_member_v1($1,$2)",
            &[
                &(members.get_id().parse::<i64>().unwrap()),
                &(members.get_is_allow() as String), 
            ],
        ).map_err(Error::PolicyMembersUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let assembly = self.merge_permissions(&row, PULL_DIRECTLY)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn list_by_account(&self, account: &IdGet) -> PolicyMembersOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_policy_members_by_account_v1($1)",
            &[
                &(account.get_name() as String)
            ],
        ).map_err(Error::PolicyMembersGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(self.merge_permissions(&row, PULL_DIRECTLY)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_by_team(&self, get_teams: &IdGet) -> PolicyMembersOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_policy_members_by_team_v1($1)",
            &[
                &(get_teams.get_id() as String)
            ],
        ).map_err(Error::PolicyMembersGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(self.merge_permissions(&row, PULL_DIRECTLY)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    /// Expands the policies by sticking in Members    
    fn merge_permissions(&self, row: &postgres::rows::Row, how_to: PullFromCache) -> Result<PolicyMembers> {
        let mut policymember = row_to_policy_members(&row)?;
        self.expander.with_permissions(&mut policymember, how_to);        
        Ok(policymember)
    }

}

fn row_to_policy_members(row: &postgres::rows::Row) -> Result<PolicyMembers> {
    let mut policies = PolicyMembers::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    
    policies.set_id(id.to_string() as String);    
    policies.set_is_allow(row.get("is_allow"));        
    policies.set_policy_name(row.get("policy_name"));    
    policies.set_created_at(created_at.to_rfc3339());
    policies.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    Ok(policies)
}
