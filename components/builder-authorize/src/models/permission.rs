// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [roles, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::Permissions;
use protocol::api::base::IdGet;

use postgres;
use db::data_store::DataStoreConn;
use super::super::{PermissionsOutput, PermissionsOutputList};

pub struct DataStore;

impl DataStore {
    pub fn permissions_create(datastore: &DataStoreConn, permissions: &Permissions) -> PermissionsOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_permission_v1 ($1,$2,$3)",
            &[
                &(permissions.get_role_id().parse::<i64>().unwrap()),
                &(permissions.get_name() as String),
                &(permissions.get_description() as String),
            ],
        ).map_err(Error::PermissionsCreate)?;

        if rows.len() > 0 {
            let permissions_create = row_to_permissions(&rows.get(0))?;
            return Ok(Some(permissions_create));
        }
        Ok(None)
    }

    pub fn permissions_list(datastore: &DataStoreConn) -> PermissionsOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_permissions_v1()", &[])
            .map_err(Error::PermissionsGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_permissions(&row)?)
            }
            return Ok(Some(response));
        }

        Ok(None) //this isn't needed as we will send an empty vec
    }

    pub fn get_rolebased_permissions(datastore: &DataStoreConn, get_permission: &IdGet) -> PermissionsOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_permission_for_role_v1($1)",
            &[&(get_permission.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::RolePermissionsGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_permissions(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn permissions_show(datastore: &DataStoreConn, get_perms: &IdGet) -> PermissionsOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_permission_v1($1)",
            &[&(get_perms.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::PermissionGet)?;

        if rows.len() > 0 {
            let permission = row_to_permissions(&rows.get(0))?;
            return Ok(Some(permission));
        }
        Ok(None)
    }

    pub fn get_specfic_permission_based_role(datastore: &DataStoreConn, get_perms: &IdGet) -> PermissionsOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_specfic_permission_role_v1($1,$2)",
            &[
                &(get_perms.get_id().parse::<i64>().unwrap()),
                &(get_perms.get_name().parse::<i64>().unwrap()),
            ],
        ).map_err(Error::PermissionGet)?;

        if rows.len() > 0 {
            let permission = row_to_permissions(&rows.get(0))?;
            return Ok(Some(permission));
        }
        Ok(None)
    }

    pub fn list_permission_by_email(datastore: &DataStoreConn, get_email: &IdGet) -> PermissionsOutputList {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_permission_by_email_v1($1)",
            &[&(get_email.get_id() as String)],
        ).map_err(Error::PermissionsGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_permissions(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    //Don't understand the this. ?
    // What is get_role_by_name
    pub fn get_role_by_name(datastore: &DataStoreConn, roles: &Vec<String>) -> PermissionsOutputList {
        let conn = datastore.pool.get_shard(0)?;
        for role in roles {
            //We iterate and return before all roles are iterated.
            let rows = &conn.query("SELECT * FROM get_permission_by_role_name_v1($1)", &[&role])
                .map_err(Error::RoleGet)?;

            let mut perms_collection = Vec::new();

            if rows.len() > 0 {
                for row in rows {
                    let per_get = row_to_permissions(&row)?;
                    perms_collection.push(per_get);
                }
            }
            return Ok(Some(perms_collection));
        }
        Ok(None)
    }
}

fn row_to_permissions(row: &postgres::rows::Row) -> Result<Permissions> {
    let mut permissions = Permissions::new();

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let role_id: i64 = row.get("role_id");
    let description: String = row.get("description");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    permissions.set_id(id.to_string() as String);
    permissions.set_role_id(role_id.to_string() as String);
    permissions.set_name(name as String);
    permissions.set_description(description as String);
    permissions.set_created_at(created_at.to_rfc3339());

    Ok(permissions)
}
