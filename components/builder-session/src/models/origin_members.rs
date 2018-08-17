// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::origin::OriginMembers;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_INVALDATED};
use super::super::{OriginMembersOutput, OriginMembersOutputList};
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

    pub fn create(&self,teams: &OriginMembers) -> OriginMembersOutput {
        let conn = self.db.pool.get_shard(0)?;        
         let rows = &conn.query(
            "SELECT * FROM insert_origin_member_v1 ($1,$2,$3)",
            &[               
                &(serde_json::to_value(teams.type_meta()).unwrap()),
                &(serde_json::to_value(teams.object_meta()).unwrap()),                
                &(serde_json::to_value(teams.get_metadata()).unwrap()),                
            ],
        ).map_err(Error::OriginMembersCreate)?;     
       
        if rows.len() > 0 {
            let origin = row_to_origin_members(&rows.get(0))?;
            return Ok(Some(origin));
        }
        Ok(None)
    }   


    pub fn list(&self, get_account: &IdGet) -> OriginMembersOutputList {
        let conn = self.db.pool.get_shard(0)?; 

        let rows = &conn.query(
            "SELECT * FROM get_origin_members_by_account_v1($1)",
            &[&(get_account.get_name() as String)]
            ).map_err(Error::OriginMembersGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_origin_members(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

}

fn row_to_origin_members(row: &postgres::rows::Row) -> Result<OriginMembers> {
    let mut origin = OriginMembers::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    origin.set_id(id.to_string() as String);        
    origin.set_created_at(created_at.to_rfc3339());
    origin.set_metadata(serde_json::from_value(row.get("meta_data")).unwrap());
    Ok(origin)
}
