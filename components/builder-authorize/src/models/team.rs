// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::Teams;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use super::super::{TeamsOutput, TeamsOutputList};
use db::data_store::DataStoreConn;
use postgres;
use serde_json;
pub struct DataStore;

impl DataStore {
    pub fn teams_create(datastore: &DataStoreConn, teams: &Teams) -> TeamsOutput {
        let conn = datastore.pool.get_shard(0)?;
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
            let teams_create = row_to_teams(&rows.get(0))?;
            return Ok(Some(teams_create));
        }
        Ok(None)
    }

    pub fn teams_show(datastore: &DataStoreConn, get_teams: &IdGet) -> TeamsOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_team_v1($1)",
            &[&(get_teams.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::TeamGet)?;

        if rows.len() > 0 {
            let team = row_to_teams(&rows.get(0))?;
            return Ok(Some(team));
        }
        Ok(None)
    }


    pub fn team_show_by_name(datastore: &DataStoreConn, get_teams: &IdGet) -> TeamsOutput {
        let conn = datastore.pool.get_shard(0)?;

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

    pub fn teams_list(datastore: &DataStoreConn) -> TeamsOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_teams_v1()", &[])
            .map_err(Error::TeamsGet)?;
        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_teams(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn team_list_by_origins(datastore: &DataStoreConn, get_teams: &IdGet) -> TeamsOutputList {
        let conn = datastore.pool.get_shard(0)?;

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
