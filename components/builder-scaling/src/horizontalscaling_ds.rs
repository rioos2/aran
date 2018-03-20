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

use super::{HorizontalScalingOutput, HorizontalScalingOutputList};

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore { db: db }
    }

    pub fn create(&self, hs: &scale::HorizontalScaling) -> HorizontalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_hs_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(hs.get_scale_type() as String),
                &(hs.get_state() as String),
                &(serde_json::to_value(hs.get_metadata()).unwrap()),
                &(serde_json::to_value(hs.get_spec()).unwrap()),
                &(serde_json::to_value(hs.get_status()).unwrap()),
                &(serde_json::to_value(hs.object_meta()).unwrap()),
                &(serde_json::to_value(hs.type_meta()).unwrap()),
            ],
        ).map_err(Error::HSCreate)?;
        if rows.len() > 0 {
            let hs = self.row_to_hs(&rows.get(0))?;
            return Ok(Some(hs));
        }
        Ok(None)
    }

    pub fn status_update(&self, upd: &scale::StatusUpdate) -> HorizontalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_hs_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(&upd.get_status()).unwrap()),
            ],
        ).map_err(Error::HSSetStatus)?;
        if rows.len() > 0 {
            let hs = self.row_to_hs(&rows.get(0))?;
            return Ok(Some(hs));
        }
        Ok(None)
    }

    pub fn update(&self, hs: &scale::HorizontalScaling) -> HorizontalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_hs_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(hs.get_id().parse::<i64>().unwrap()),
                &(hs.get_scale_type() as String),
                &(hs.get_state() as String),
                &(serde_json::to_value(hs.get_metadata()).unwrap()),
                &(serde_json::to_value(hs.get_spec()).unwrap()),
                &(serde_json::to_value(hs.get_status()).unwrap()),
                &(serde_json::to_value(hs.object_meta()).unwrap()),
            ],
        ).map_err(Error::HSUpdate)?;
        if rows.len() > 0 {
            let hscale = self.row_to_hs(&rows.get(0))?;
            return Ok(Some(hscale));
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> HorizontalScalingOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_hs_v1()", &[]).map_err(
            Error::HSGet,
        )?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_hs(&row)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    ///TO-DO need to use in future
    /*pub fn show_by_assembly_factory(&self, scale_get: &IdGet) -> HorizontalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_scale_by_asmfacid_v1($1)",
            &[&(scale_get.get_id() as String)],
        ).map_err(Error::HSGet)?;

        if rows.len() > 0 {
            let scale = self.row_to_hs(&rows.get(0))?;
            return Ok(Some(scale));
        }
        Ok(None)
    }*/

    pub fn show(&self, scale_get: &IdGet) -> HorizontalScalingOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_horizontal_scaling_v1($1)",
            &[&(scale_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::HSGet)?;

        if rows.len() > 0 {
            let scale = self.row_to_hs(&rows.get(0))?;
            return Ok(Some(scale));
        }
        Ok(None)
    }

    fn row_to_hs(&self, row: &postgres::rows::Row) -> Result<scale::HorizontalScaling> {
        let mut hs = scale::HorizontalScaling::with(
            serde_json::from_value(row.get("type_meta")).unwrap(),
            serde_json::from_value(row.get("object_meta")).unwrap(),
        );

        let id: i64 = row.get("id");
        let created_at = row.get::<&str, DateTime<Utc>>("created_at");

        hs.set_id(id.to_string() as String);
        hs.set_scale_type(row.get("scale_type"));
        hs.set_state(row.get("state"));
        hs.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
        hs.set_spec(serde_json::from_value(row.get("spec")).unwrap());
        hs.set_status(serde_json::from_value(row.get("status")).unwrap());
        hs.set_created_at(created_at.to_rfc3339());

        Ok(hs)
    }
}
