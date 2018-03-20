// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Build config
use chrono::prelude::*;
use error::{Result, Error};
use protocol::api::devtool::BuildConfig;
use protocol::api::base::{MetaFields, StatusUpdate};
use protocol::api::base::IdGet;

use postgres;
use db::data_store::DataStoreConn;
use serde_json;

use super::super::{BuildConfigOutputList, BuildConfigOutput};

pub struct DataStore;

impl DataStore {
    pub fn create(datastore: &DataStoreConn, build_create: &BuildConfig) -> BuildConfigOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_build_config_v1($1,$2,$3,$4,$5)",
            &[
                &(serde_json::to_value(build_create.get_meta_data()).unwrap()),
                &(serde_json::to_value(build_create.get_spec()).unwrap()),
                &(serde_json::to_value(build_create.get_status()).unwrap()),
                &(serde_json::to_value(build_create.object_meta()).unwrap()),
                &(serde_json::to_value(build_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::BuildConfigCreate)?;
        if rows.len() > 0 {
            let build_conf = row_to_build_config(&rows.get(0))?;
            return Ok(Some(build_conf.clone()));
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn) -> BuildConfigOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_build_configs_v1()", &[])
            .map_err(Error::BuildConfigGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_build_config(&row)?)
            }
            return Ok(Some(response));
        }

        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, bc_get: &IdGet) -> BuildConfigOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_build_config_v1($1)",
            &[&(bc_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::BuildConfigGet)?;

        if rows.len() > 0 {
            let bc = row_to_build_config(&rows.get(0))?;
            return Ok(Some(bc));
        }
        Ok(None)
    }

    pub fn show_by_assemblyfactory(datastore: &DataStoreConn, bf_get: &IdGet) -> BuildConfigOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_build_config_by_assembly_factory_v1($1)",
            &[&(bf_get.get_id() as String)],
        ).map_err(Error::BuildConfigGet)?;

        if rows.len() > 0 {
            let build = row_to_build_config(&rows.get(0))?;
            return Ok(Some(build));
        }
        Ok(None)
    }

    pub fn update(db: &DataStoreConn, build_update: &BuildConfig) -> BuildConfigOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM update_build_config_by_v1($1,$2,$3,$4,$5)",
            &[
                &(build_update.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(build_update.get_spec()).unwrap()),
                &(serde_json::to_value(build_update.get_status()).unwrap()),
                &(serde_json::to_value(build_update.get_meta_data()).unwrap()),
                &(serde_json::to_value(build_update.object_meta()).unwrap()),
            ],
        ).map_err(Error::BuildConfigUpdate)?;


        if rows.len() > 0 {
            let bc = row_to_build_config(&rows.get(0))?;
            return Ok(Some(bc));
        }
        Ok(None)
    }
    pub fn status_update(db: &DataStoreConn, upd: &StatusUpdate) -> BuildConfigOutput {
        let conn = db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_build_configs_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::BuildConfigUpdate)?;

        if rows.len() > 0 {
            let bc = row_to_build_config(&rows.get(0))?;
            return Ok(Some(bc));
        }
        Ok(None)
    }
}

fn row_to_build_config(row: &postgres::rows::Row) -> Result<BuildConfig> {
    let mut build_config = BuildConfig::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    build_config.set_id(id.to_string());
    build_config.set_meta_data(serde_json::from_value(row.get("meta_data")).unwrap());
    build_config.set_spec(serde_json::from_value(row.get("spec")).unwrap());
    build_config.set_status(serde_json::from_value(row.get("status")).unwrap());
    build_config.set_created_at(created_at.to_rfc3339());
    Ok(build_config)
}
