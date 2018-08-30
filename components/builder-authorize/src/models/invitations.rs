// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [teams, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::invitations::Invitations;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use super::super::{InvitationsOutput,InvitationsOutputList};
use db::data_store::DataStoreConn;
use postgres;
use serde_json;

pub const PENDING: &'static str = "pending";
pub const ACCEPT: &'static str = "accept";

pub struct DataStore;

impl DataStore {
    pub fn create(datastore: &DataStoreConn, ins: &Invitations) -> InvitationsOutput {
        let conn = datastore.pool.get_shard(0)?;
       
         let rows = &conn.query(
            "SELECT * FROM insert_invitations_v1 ($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(ins.get_invite_from() as String),
                &(ins.get_invite_to() as String),
                &(ins.get_origin_id() as String),
                &(ins.get_team_id() as String),
                &(serde_json::to_value(ins.object_meta()).unwrap()),
                &(serde_json::to_value(ins.type_meta()).unwrap()),
                &(PENDING.to_string()),
            ],
        ).map_err(Error::InvitationsCreate)?;

        if rows.len() > 0 {
            let invitations_create = row_to_invitations(&rows.get(0))?;
            return Ok(Some(invitations_create));
        }
        Ok(None)
    }   

    pub fn show(datastore: &DataStoreConn, net_get: &IdGet) -> InvitationsOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_invitations_v1($1)",
            &[&(net_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::InvitationsGet)?;

        if rows.len() > 0 {
            let net = row_to_invitations(&rows.get(0))?;            
            return Ok(Some(net));
        }       
        
        Ok(None)
    }

   pub fn list_by_teams(datastore: &DataStoreConn, get_teams: &IdGet) -> InvitationsOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_invitations_by_teams_v1($1)",
            &[&(get_teams.get_id() as String)],
        ).map_err(Error::InvitationsGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_invitations(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn update_status(db: &DataStoreConn, team: &IdGet) -> InvitationsOutput {
        let conn = db.pool.get_shard(0)?;
        
        let rows = &conn.query(
            "SELECT * FROM update_status_by_team_v1($1, $2)",
            &[
                &(team.get_id().parse::<i64>().unwrap()),
                &(ACCEPT.to_string()),
            ],
        ).map_err(Error::InvitationsUpdate)?;

        if rows.len() > 0 {
            let end = row_to_invitations(&rows.get(0))?;
            return Ok(Some(end));
        }
        Ok(None)
    }
}

fn row_to_invitations(row: &postgres::rows::Row) -> Result<Invitations> {
    let mut invitations = Invitations::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    invitations.set_id(id.to_string() as String);
    invitations.set_invite_from(row.get("invite_from"));
    invitations.set_invite_to(row.get("invite_to"));
    invitations.set_origin_id(row.get("origin_id"));
    invitations.set_team_id(row.get("team_id"));
    invitations.set_status(row.get("status"));
    invitations.set_created_at(created_at.to_rfc3339());

    Ok(invitations)
}
