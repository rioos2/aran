// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [roles, permissions].

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::authorize::{Permissions, PermissionsForAccount};
use protocol::api::base::IdGet;

use protocol::cache::{InMemoryExpander, PULL_DIRECTLY};

use super::super::{PermissionsOutput, PermissionsOutputList};
use db::data_store::DataStoreConn;
use postgres;

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

    pub fn create(&self, permissions: &Permissions) -> PermissionsOutput {
        let conn = self.db.pool.get_shard(0)?;
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

    //Return a permission for a permission_id
    pub fn show(&self, get_perms: &IdGet) -> PermissionsOutput {
        let conn = self.db.pool.get_shard(0)?;

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

    //Return a permission for a role_id and permission_id
    pub fn show_by_role(&self, get_perms: &IdGet) -> PermissionsOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_permission_by_role_v1($1,$2)",
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

    pub fn list_blank(&self) -> PermissionsOutputList {
        let conn = self.db.pool.get_shard(0)?;
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

    //This is a fascade method to list_by_email.
    pub fn list_by_email_fascade(&self, email: IdGet) -> PermissionsForAccount {
        let mut perms_for_account = PermissionsForAccount::new();
        perms_for_account.set_account_email(email.get_id());
        self.expander
            .with_permissions(&mut perms_for_account, PULL_DIRECTLY);
        perms_for_account
    }

    //This is a fascade method to list_by_email.
    pub fn list_by_email(&self, email: &IdGet) -> PermissionsOutputList {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            &"SELECT * FROM get_permission_by_email_v1($1)",
            &[&(email.get_id() as String)],
        ).map_err(Error::PermissionsGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_permissions(&row)?)
            }
            info!(
                "---------- STRT: Permission loader {} ----------",
                email.get_id()
            );
            info!("Loaded ! Permissions\n{:?}", response);
            info!(
                "---------- DONE: Permission loader {} ----------",
                email.get_id()
            );
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_by_role(&self, role_id: &IdGet) -> PermissionsOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_permissions_by_role_v1($1)",
            &[&(role_id.get_id().parse::<i64>().unwrap())],
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
