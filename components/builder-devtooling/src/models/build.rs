// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Build
use chrono::prelude::*;
use error::{Result, Error};
use protocol::api::devtool::{Build, BuildStatusUpdate};
use protocol::api::base::MetaFields;
use protocol::api::base::IdGet;

use postgres;
use db::data_store::DataStoreConn;
use serde_json;

use super::super::{BuildOutputList, BuildOutput};

pub struct DataStore;

impl DataStore {
    pub fn create(datastore: &DataStoreConn, build_create: &Build) -> BuildOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_build_v1($1,$2,$3,$4)",
            &[
                &(serde_json::to_value(build_create.get_status()).unwrap()),
                &(serde_json::to_value(build_create.get_spec()).unwrap()),
                &(serde_json::to_value(build_create.object_meta()).unwrap()),
                &(serde_json::to_value(build_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::BuildCreate)?;
        if rows.len() > 0 {
            let build = row_to_build(&rows.get(0))?;
            return Ok(Some(build.clone()));
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn) -> BuildOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_builds_v1()", &[]).map_err(
            Error::BuildGetResponse,
        )?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_build(&row)?)
            }
            return Ok(Some(response));
        }

        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, b_get: &IdGet) -> BuildOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_build_v1($1)",
            &[&(b_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::BuildGet)?;

        if rows.len() > 0 {
            let bc = row_to_build(&rows.get(0))?;
            return Ok(Some(bc));
        }
        Ok(None)
    }

    pub fn show_by_build_config(datastore: &DataStoreConn, b_get: &IdGet) -> BuildOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_build_by_build_config_v1($1)",
            &[&(b_get.get_id() as String)],
        ).map_err(Error::BuildGet)?;

        if rows.len() > 0 {
            let build = row_to_build(&rows.get(0))?;
            return Ok(Some(build));
        }
        Ok(None)
    }

    pub fn update(db: &DataStoreConn, build: &Build) -> BuildOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM update_build_by_v1($1,$2,$3,$4)",
            &[
                &(build.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(build.get_spec()).unwrap()),
                &(serde_json::to_value(build.get_status()).unwrap()),
                &(serde_json::to_value(build.object_meta()).unwrap()),
            ],
        ).map_err(Error::BuildUpdate)?;

        if rows.len() > 0 {
            let build = row_to_build(&rows.get(0))?;
            return Ok(Some(build));
        }
        Ok(None)
    }

    pub fn status_update(db: &DataStoreConn, status: &BuildStatusUpdate) -> BuildOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM update_build_status_by_v1($1,$2)",
            &[
                &(status.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(status.get_status()).unwrap()),
            ],
        ).map_err(Error::BuildStatusUpdate)?;

        if rows.len() > 0 {
            let build = row_to_build(&rows.get(0))?;
            return Ok(Some(build));
        }
        Ok(None)
    }
}

fn row_to_build(row: &postgres::rows::Row) -> Result<Build> {
    let mut build = Build::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    build.set_id(id.to_string());
    build.set_status(serde_json::from_value(row.get("status")).unwrap());
    build.set_spec(serde_json::from_value(row.get("spec")).unwrap());
    build.set_created_at(created_at.to_rfc3339());
    Ok(build)
}
