// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::{Policies, PoliciesForLevel};
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::cache::{InMemoryExpander, PULL_DIRECTLY};
use super::super::{PolicyOutputList};
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

    pub fn list_blank(&self) -> PolicyOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_policies_v1()", &[])
            .map_err(Error::PoliciesGet)?;
        if rows.len() > 0 {
            let mut response = Vec::new();
            for row in rows {
                response.push(row_to_policies(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_by_level(&self, get_level: &IdGet) -> PolicyOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_policies_by_level_v1($1)",
            &[&(get_level.get_id() as String)],
        ).map_err(Error::PoliciesGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_policies(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    //This is a fascade method to get policies by level from cache.
    pub fn list_by_level_fascade(&self, policy: IdGet) -> PoliciesForLevel {
        let mut policies_for_level = PoliciesForLevel::new();
        policies_for_level.set_level(policy.get_id());
        self.expander
            .with_policies(&mut policies_for_level, PULL_DIRECTLY);
        policies_for_level
    }

}

fn row_to_policies(row: &postgres::rows::Row) -> Result<Policies> {
    let mut policy = Policies::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    policy.set_id(id.to_string() as String);
    policy.set_created_at(created_at.to_rfc3339());
    policy.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());

    Ok(policy)
}
