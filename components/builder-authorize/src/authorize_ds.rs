// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Authorization [roles, permissions].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{authsrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;

pub struct AuthorizeDS;

impl AuthorizeDS {
    pub fn roles_create(datastore: &DataStoreConn, roles: &authsrv::Roles) -> Result<Option<authsrv::Roles>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_role_v1 ($1,$2)",
            &[
                &(roles.get_name() as String),
                &(roles.get_description() as String),
            ],
        ).map_err(Error::RolesCreate)?;

        for row in rows {
            let roles_create = row_to_roles(&row)?;
            return Ok(Some(roles_create));
        }
        Ok(None)
    }

    pub fn roles_show(datastore: &DataStoreConn, get_roles: &asmsrv::IdGet) -> Result<Option<authsrv::Roles>> {
        let conn = datastore.pool.get_shard(0)?;
        let role_id = get_roles.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_role_v1($1)", &[&role_id])
            .map_err(Error::RoleGet)?;

        for row in rows {
            let roles_get = row_to_roles(&row)?;
            return Ok(Some(roles_get));
        }
        Ok(None)
    }


    pub fn get_role_by_name(datastore: &DataStoreConn, roles: &Vec<String>) -> Result<Option<authsrv::PermissionsGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;
        let mut response = authsrv::PermissionsGetResponse::new();
        let mut perms_collection = Vec::new();
        for role in roles {
            let rows = &conn.query("SELECT * FROM get_permission_by_role_name_v1($1)", &[&role])
                .map_err(Error::RoleGet)?;
            for row in rows {
                let per_get = row_to_permissions(&row)?;
                perms_collection.push(per_get);
            }
        }
        response.set_permissions(
            perms_collection,
            "PermissionList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }

    pub fn roles_list(datastore: &DataStoreConn) -> Result<Option<authsrv::RolesGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_roles_v1()", &[]).map_err(
            Error::RolesGet,
        )?;

        let mut response = authsrv::RolesGetResponse::new();

        let mut roles_collection = Vec::new();

        for row in rows {
            let roles = row_to_roles(&row)?;
            roles_collection.push(roles);
        }
        response.set_roles(roles_collection, "RolesList".to_string(), "v1".to_string());
        Ok(Some(response))
    }

    pub fn permissions_create(datastore: &DataStoreConn, permissions: &authsrv::Permissions) -> Result<Option<authsrv::Permissions>> {
        let conn = datastore.pool.get_shard(0)?;
        let role_id = permissions.get_role_id().parse::<i64>().unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_permission_v1 ($1,$2,$3)",
            &[
                &role_id,
                &(permissions.get_name() as String),
                &(permissions.get_description() as String),
            ],
        ).map_err(Error::PermissionsCreate)?;

        for row in rows {
            let permissions_create = row_to_permissions(&row)?;
            return Ok(Some(permissions_create));
        }
        Ok(None)
    }

    pub fn permissions_list(datastore: &DataStoreConn) -> Result<Option<authsrv::PermissionsGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_permissions_v1()", &[])
            .map_err(Error::PermissionsGet)?;

        let mut response = authsrv::PermissionsGetResponse::new();

        let mut perm_collection = Vec::new();

        for row in rows {
            let perm = row_to_permissions(&row)?;
            perm_collection.push(perm);
        }
        response.set_permissions(
            perm_collection,
            "PermissionList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }

    pub fn get_rolebased_permissions(datastore: &DataStoreConn, get_permission: &asmsrv::IdGet) -> Result<Option<authsrv::Permissions>> {
        let conn = datastore.pool.get_shard(0)?;

        let role_id = get_permission.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_permission_for_role_v1($1)", &[&role_id])
            .map_err(Error::RolePermissionsGet)?;

        for row in rows {
            let permissions_get = row_to_permissions(&row)?;
            return Ok(Some(permissions_get));
        }
        Ok(None)
    }

    pub fn permissions_show(datastore: &DataStoreConn, get_perms: &asmsrv::IdGet) -> Result<Option<authsrv::Permissions>> {
        let conn = datastore.pool.get_shard(0)?;
        let perm_id = get_perms.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_permission_v1($1)", &[&perm_id])
            .map_err(Error::PermissionGet)?;

        for row in rows {
            let perm_get = row_to_permissions(&row)?;
            return Ok(Some(perm_get));
        }
        Ok(None)
    }

    pub fn get_specfic_permission_based_role(datastore: &DataStoreConn, get_perms: &asmsrv::IdGet) -> Result<Option<authsrv::Permissions>> {
        let conn = datastore.pool.get_shard(0)?;
        let perm_id = get_perms.get_id().parse::<i64>().unwrap();
        let role_id = get_perms.get_name().parse::<i64>().unwrap();
        let rows = &conn.query(
            "SELECT * FROM get_specfic_permission_role_v1($1,$2)",
            &[&perm_id, &role_id],
        ).map_err(Error::PermissionGet)?;

        for row in rows {
            let perm_get = row_to_permissions(&row)?;
            return Ok(Some(perm_get));
        }
        Ok(None)
    }
}


fn row_to_roles(row: &postgres::rows::Row) -> Result<authsrv::Roles> {
    let mut roles = authsrv::Roles::new();

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let description: String = row.get("description");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    roles.set_id(id.to_string() as String);
    roles.set_name(name as String);
    roles.set_description(description as String);
    roles.set_created_at(created_at.to_rfc3339());


    Ok(roles)
}


fn row_to_permissions(row: &postgres::rows::Row) -> Result<authsrv::Permissions> {
    let mut permissions = authsrv::Permissions::new();

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


    Ok(permissions)
}
