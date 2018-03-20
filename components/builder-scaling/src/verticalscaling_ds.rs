// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::api::scale;
use protocol::api::base::MetaFields;

use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use protocol::api::base::IdGet;
use super::{VerticalScalingOutput, VerticalScalingOutputList};

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}
impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore { db: db }
    }
    pub fn create(&self, vertical: &scale::VerticalScaling) -> VerticalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_vs_v1($1,$2,$3,$4,$5,$6,$7,$8)",
            &[
                &(vertical.get_scale_type() as String),
                &(vertical.get_state() as String),
                &(serde_json::to_value(vertical.get_update_policy()).unwrap()),
                &(serde_json::to_value(vertical.get_metadata()).unwrap()),
                &(serde_json::to_value(vertical.get_spec()).unwrap()),
                &(serde_json::to_value(vertical.get_status()).unwrap()),
                &(serde_json::to_value(vertical.object_meta()).unwrap()),
                &(serde_json::to_value(vertical.type_meta()).unwrap()),
            ],
        ).map_err(Error::VSCreate)?;
        if rows.len() > 0 {
            let vs = self.row_to_vertical(&rows.get(0))?;
            return Ok(Some(vs));
        }
        Ok(None)
    }

    pub fn show(&self, scale_get: &IdGet) -> VerticalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_vertical_scaling_v1($1)",
            &[&(scale_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::HSGet)?;

        if rows.len() > 0 {
            let scale = self.row_to_vertical(&rows.get(0))?;
            return Ok(Some(scale));
        }
        Ok(None)
    }

    pub fn status_update(&self, upd: &scale::VerticalScalingStatusUpdate) -> VerticalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_vs_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(&upd.get_status()).unwrap()),
            ],
        ).map_err(Error::VSSetStatus)?;
        if rows.len() > 0 {
            let vs = self.row_to_vertical(&rows.get(0))?;
            return Ok(Some(vs));
        }
        Ok(None)
    }

    pub fn update(&self, vs: &scale::VerticalScaling) -> VerticalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_vs_v1($1,$2,$3,$4,$5,$6,$7,$8)",
            &[
                &(vs.get_id().parse::<i64>().unwrap()),
                &(vs.get_scale_type() as String),
                &(vs.get_state() as String),
                &(serde_json::to_value(vs.get_update_policy()).unwrap()),
                &(serde_json::to_value(vs.get_metadata()).unwrap()),
                &(serde_json::to_value(vs.get_spec()).unwrap()),
                &(serde_json::to_value(vs.get_status()).unwrap()),
                &(serde_json::to_value(vs.object_meta()).unwrap()),
            ],
        ).map_err(Error::VSUpdate)?;
        if rows.len() > 0 {
            let vscale = self.row_to_vertical(&rows.get(0))?;
            return Ok(Some(vscale));
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> VerticalScalingOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_vs_v1()", &[])
            .map_err(Error::VSGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_vertical(&row)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
    fn row_to_vertical(&self, row: &postgres::rows::Row) -> Result<scale::VerticalScaling> {
        let mut vertical = scale::VerticalScaling::with(
            serde_json::from_value(row.get("type_meta")).unwrap(),
            serde_json::from_value(row.get("object_meta")).unwrap(),
        );

        let id: i64 = row.get("id");
        let created_at = row.get::<&str, DateTime<Utc>>("created_at");

        vertical.set_id(id.to_string() as String);
        vertical.set_scale_type(row.get("scale_type"));
        vertical.set_state(row.get("state"));
        vertical.set_update_policy(serde_json::from_value(row.get("update_policy")).unwrap());
        vertical.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
        vertical.set_spec(serde_json::from_value(row.get("spec")).unwrap());
        vertical.set_status(serde_json::from_value(row.get("status")).unwrap());
        vertical.set_created_at(created_at.to_rfc3339());

        Ok(vertical)
    }
}
