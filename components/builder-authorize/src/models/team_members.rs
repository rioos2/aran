// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::TeamMembers;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_INVALDATED};
use super::super::{TeamMembersOutput, TeamMembersOutputList};
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

    pub fn create(&self,teams: &TeamMembers) -> TeamMembersOutput {
        let conn = self.db.pool.get_shard(0)?;        
         let rows = &conn.query(
            "SELECT * FROM insert_team_member_v1 ($1,$2,$3)",
            &[               
                &(serde_json::to_value(teams.type_meta()).unwrap()),
                &(serde_json::to_value(teams.object_meta()).unwrap()),                
                &(serde_json::to_value(teams.get_metadata()).unwrap()),                
            ],
        ).map_err(Error::TeamMembersCreate)?;      
        
        if rows.len() > 0 {
            for row in rows {
                let team = self.merge_team(&row, PULL_INVALDATED)?;
                return Ok(Some(team));
            }
        }
        Ok(None)
    }

     pub fn list_by_origins(&self, get_teams: &IdGet, account: &IdGet) -> TeamMembersOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_team_members_by_origins_v1($1, $2)",
            &[
                &(get_teams.get_id() as String),
                &(account.get_name() as String)
            ],
        ).map_err(Error::TeamGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(self.merge_team(&row, PULL_INVALDATED)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    /// Expands the teams by sticking in Members    
    fn merge_team(&self, row: &postgres::rows::Row, how_to: PullFromCache) -> Result<TeamMembers> {
        let mut teammember = row_to_team_members(&row)?;
        self.expander.with_teams(&mut teammember, how_to);        
        Ok(teammember)
    }

}

fn row_to_team_members(row: &postgres::rows::Row) -> Result<TeamMembers> {
    let mut teams = TeamMembers::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    
    teams.set_id(id.to_string() as String);        
    teams.set_created_at(created_at.to_rfc3339());
    teams.set_metadata(serde_json::from_value(row.get("meta_data")).unwrap());
    Ok(teams)
}
