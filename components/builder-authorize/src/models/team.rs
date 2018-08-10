// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::Teams;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_INVALDATED};
use super::super::{TeamsOutput, TeamsOutputList};
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

    pub fn create(&self,teams: &Teams) -> TeamsOutput {
        let conn = self.db.pool.get_shard(0)?;
        let origin: String = match teams.get_metadata().get("origin") {
                        Some(org) => org.to_string(),
                        None => "".to_string()
                    };
         let rows = &conn.query(
            "SELECT * FROM insert_team_v1 ($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(teams.get_name() as String),
                &(teams.get_description() as String),
                &(teams.get_account() as String),
                &(origin as String),
                &(serde_json::to_value(teams.object_meta()).unwrap()),
                &(serde_json::to_value(teams.type_meta()).unwrap()),
                &(serde_json::to_value(teams.get_metadata()).unwrap()),                
            ],
        ).map_err(Error::TeamsCreate)?;

       if rows.len() > 0 {
            for row in rows {
                let team = self.collect_members(&row, PULL_INVALDATED)?;
                return Ok(Some(team));
            }
        }
        Ok(None)
    }

    //show team with team members
    pub fn show(&self,get_teams: &IdGet) -> TeamsOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_team_v1($1)",
            &[&(get_teams.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::TeamGet)?;

        if rows.len() > 0 {
            for row in rows {
                 let team = self.collect_members(&row, PULL_INVALDATED)?;
                return Ok(Some(team));
            }
        }
        Ok(None)
    }


    pub fn show_by_name(&self,get_teams: &IdGet) -> TeamsOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_team_by_name_v1($1)",
            &[&(get_teams.get_id() as String)],
        ).map_err(Error::TeamGet)?;

        if rows.len() > 0 {
            let team = row_to_teams(&rows.get(0))?;
            return Ok(Some(team));
        }
        Ok(None)
    }

    pub fn list(&self) -> TeamsOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_teams_v1()", &[])
            .map_err(Error::TeamsGet)?;
        if rows.len() > 0 {
            let mut response = Vec::new();
            for row in rows {
                response.push(row_to_teams(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_by_origins(&self,get_teams: &IdGet) -> TeamsOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_teams_by_origins_v1($1)",
            &[&(get_teams.get_id() as String)],
        ).map_err(Error::TeamGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_teams(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    /// Expands the teams by sticking in Members    
    fn collect_members(&self, row: &postgres::rows::Row, how_to: PullFromCache) -> Result<Teams> {
        let mut team = row_to_teams(&row)?;
        self.expander.with_members(&mut team, how_to);        
        Ok(team)
    }

}

fn row_to_teams(row: &postgres::rows::Row) -> Result<Teams> {
    let mut teams = Teams::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    teams.set_id(id.to_string() as String);
    teams.set_name(row.get("name"));
    teams.set_description(row.get("description"));
    teams.set_created_at(created_at.to_rfc3339());

    Ok(teams)
}
