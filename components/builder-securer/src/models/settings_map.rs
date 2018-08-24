// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::Error;
use postgres;
use protocol::api::base::IdGet;
use protocol::api::base::MetaFields;
use protocol::api::{base, settings_map};
use serde_json;

use super::super::SettingsMapOutput;

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore {
            db: db,
        }
    }

    pub fn create(&self, settings_map_create: &settings_map::SettingsMap) -> SettingsMapOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_settings_map_v1($1,$2,$3,$4)",
            &[
                &(serde_json::to_value(settings_map_create.get_metadata()).unwrap()),
                &(serde_json::to_value(settings_map_create.get_data()).unwrap()),
                &(serde_json::to_value(&settings_map_create.object_meta()).unwrap()),
                &(serde_json::to_value(&settings_map_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::SettingsMapCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let settings = row_to_settings_map(&row);
                return Ok(Some(settings));
            }
        }
        Ok(None)
    }
    pub fn show(&self, get_settingsmap: &base::IdGet) -> SettingsMapOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_settings_map_by_name_v1($1,$2)",
            &[&(get_settingsmap.get_name()), &(get_settingsmap.get_id())],
        ).map_err(Error::SettingsMapGet)?;
        if rows.len() > 0 {
            for row in rows {
                let set_map = row_to_settings_map(&row);
                return Ok(Some(set_map));
            }
        }
        Ok(None)
    }

    pub fn show_by_id(&self, get_settingsmap: &IdGet) -> SettingsMapOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_settings_map_v1($1)",
            &[&(get_settingsmap.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::SettingsMapGet)?;
        if rows.len() > 0 {
            let set_map = row_to_settings_map(&rows.get(0));
            return Ok(Some(set_map));
        }
        Ok(None)
    }
}
fn row_to_settings_map(row: &postgres::rows::Row) -> settings_map::SettingsMap {
    let mut set_map = settings_map::SettingsMap::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    set_map.set_id(id.to_string());
    set_map.set_data(serde_json::from_value(row.get("data")).unwrap());
    set_map.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    set_map.set_created_at(created_at.to_rfc3339());

    set_map
}
