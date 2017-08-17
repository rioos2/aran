// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protocol::authsrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct AuthorizeDS;

impl AuthorizeDS {
    pub fn roles_create(datastore: &DataStoreConn, roles: &authsrv::Roles) -> Result<Option<authsrv::Roles>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: roles_create ");
        let rows = &conn.query(
            "SELECT * FROM insert_role_v1 ($1,$2)",
            &[
                &(roles.get_name() as String),
                &(roles.get_description() as String),
            ],
        ).map_err(Error::RolesCreate)?;

        debug!(">● ROWS: roles_create =>\n{:?}", &rows);
        for row in rows {
            let mut roles_create = row_to_roles(&row)?;
            return Ok(Some(roles_create));
        }
        Ok(None)
    }
    pub fn permissions_create(datastore: &DataStoreConn, permissions: &authsrv::Permissions) -> Result<Option<authsrv::Permissions>> {
        let conn = datastore.pool.get_shard(0)?;
        let role_id = permissions.get_role_id().parse::<i64>().unwrap();
        debug!("◖☩ START: permission_create ");
        let rows = &conn.query(
            "SELECT * FROM insert_permission_v1 ($1,$2,$3)",
            &[
                &role_id,
                &(permissions.get_name() as String),
                &(permissions.get_description() as String),
            ],
        ).map_err(Error::PermissionsCreate)?;

        debug!(">● ROWS: permission_create =>\n{:?}", &rows);
        for row in rows {
            let mut permissions_create = row_to_permissions(&row)?;
            return Ok(Some(permissions_create));
        }
        Ok(None)
    }
}


fn row_to_roles(row: &postgres::rows::Row) -> Result<authsrv::Roles> {
    let mut roles = authsrv::Roles::new();
    debug!("◖☩ START: row_to_roles");

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let description: String = row.get("description");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    roles.set_id(id.to_string() as String);
    roles.set_name(name as String);
    roles.set_description(description as String);
    roles.set_created_at(created_at.to_rfc3339());

    debug!("◖☩ ASM: row_to_roles =>\n{:?}", roles);
    debug!("◖☩ DONE: row_to_roles");
    Ok(roles)
}


fn row_to_permissions(row: &postgres::rows::Row) -> Result<authsrv::Permissions> {
    let mut permissions = authsrv::Permissions::new();
    debug!("◖☩ START: row_to_permissions");

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let role_id: i64 = row.get("role_id");
    let description: String = row.get("description");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    permissions.set_id(id.to_string() as String);
    permissions.set_role_id(role_id.to_string() as String);
    permissions.set_name(name as String);
    permissions.set_description(description as String);
    permissions.set_created_at(created_at.to_rfc3339());

    debug!("◖☩ ASM: row_to_permissions =>\n{:?}", permissions);
    debug!("◖☩ DONE: row_to_permissions");
    Ok(permissions)
}
