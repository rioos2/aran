// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::PolicyMembers;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_INVALDATED};
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

   
     pub fn list_by_account(&self, get_policies: &IdGet, account: &IdGet) -> PolicyMembersOutputList {
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
                response.push(self.merge_permissions(&row, PULL_INVALDATED)?)
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
    policies.set_created_at(created_at.to_rfc3339());
    policies.set_metadata(serde_json::from_value(row.get("meta_data")).unwrap());
    Ok(policies)
}
