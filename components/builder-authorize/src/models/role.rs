// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [roles, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::Roles;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use super::super::{RolesOutput, RolesOutputList};
use db::data_store::DataStoreConn;
use postgres;
use serde_json;
pub struct DataStore;

impl DataStore {
    pub fn roles_create(datastore: &DataStoreConn, roles: &Roles) -> RolesOutput {
        let conn = datastore.pool.get_shard(0)?;
        let origin: String = match roles.get_metadata().get("origin") {
                        Some(org) => org.to_string(),
                        None => "".to_string()
                    };
         let rows = &conn.query(
            "SELECT * FROM insert_role_v1 ($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(roles.get_name() as String),
                &(roles.get_description() as String),
                &(roles.get_account() as String),
                &(origin as String),
                &(serde_json::to_value(roles.object_meta()).unwrap()),
                &(serde_json::to_value(roles.type_meta()).unwrap()),
                &(serde_json::to_value(roles.get_metadata()).unwrap()),
            ],
        ).map_err(Error::RolesCreate)?;


        if rows.len() > 0 {
            let roles_create = row_to_roles(&rows.get(0))?;
            return Ok(Some(roles_create));
        }
        Ok(None)
    }

    pub fn roles_show(datastore: &DataStoreConn, get_roles: &IdGet) -> RolesOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_role_v1($1)",
            &[&(get_roles.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::RoleGet)?;

        if rows.len() > 0 {
            let role = row_to_roles(&rows.get(0))?;
            return Ok(Some(role));
        }
        Ok(None)
    }


    pub fn role_show_by_name(datastore: &DataStoreConn, get_roles: &IdGet) -> RolesOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_role_by_name_v1($1)",
            &[&(get_roles.get_id() as String)],
        ).map_err(Error::RoleGet)?;

        if rows.len() > 0 {
            let role = row_to_roles(&rows.get(0))?;
            return Ok(Some(role));
        }
        Ok(None)
    }

    pub fn roles_list(datastore: &DataStoreConn) -> RolesOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_roles_v1()", &[])
            .map_err(Error::RolesGet)?;
        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_roles(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn role_list_by_origins(datastore: &DataStoreConn, get_roles: &IdGet) -> RolesOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_roles_by_origins_v1($1)",
            &[&(get_roles.get_id() as String)],
        ).map_err(Error::RoleGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_roles(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_roles(row: &postgres::rows::Row) -> Result<Roles> {
    let mut roles = Roles::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    roles.set_id(id.to_string() as String);
    roles.set_name(row.get("name"));
    roles.set_description(row.get("description"));
    roles.set_created_at(created_at.to_rfc3339());

    Ok(roles)
}
