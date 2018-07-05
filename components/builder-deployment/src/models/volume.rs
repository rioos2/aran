// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use chrono::prelude::*;

use error::{Error, Result};
use protocol::api::base::{IdGet, MetaFields, StatusUpdate};
use protocol::api::volume;

use db::data_store::DataStoreConn;
use postgres;
use serde_json;

use super::super::{VolumeOutput, VolumeOutputList};

pub struct DataStore;

impl DataStore {
    pub fn create(db: &DataStoreConn, volume_create: &volume::Volumes) -> VolumeOutput {
        let conn = db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_volume_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(volume_create.get_mount_path() as String),
                &(volume_create.get_allocated() as String),
                &(serde_json::to_value(volume_create.get_status()).unwrap()),
                &(serde_json::to_value(volume_create.object_meta()).unwrap()),
                &(serde_json::to_value(volume_create.type_meta()).unwrap()),
                &(serde_json::to_value(volume_create.get_source()).unwrap()),
            ],
        ).map_err(Error::VolumesCreate)?;

        if rows.len() > 0 {
            let volumes = row_to_volumes(&rows.get(0))?;
            return Ok(Some(volumes));
        }
        Ok(None)
    }

    pub fn show(db: &DataStoreConn, get_vol: &IdGet) -> VolumeOutput {
        let conn = db.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_volume_v1($1)", &[&(get_vol.get_id().parse::<i64>().unwrap())])
            .map_err(Error::VolumesGet)?;
        if rows.len() > 0 {
            let volumes = row_to_volumes(&rows.get(0))?;
            return Ok(Some(volumes));
        }
        Ok(None)
    }

    pub fn status_update(db: &DataStoreConn, vol: &StatusUpdate) -> VolumeOutput {
        let conn = db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_volume_status_v1($1, $2)",
            &[
                &(vol.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(vol.get_status()).unwrap()),
            ],
        ).map_err(Error::VolumeUpdate)?;
        if rows.len() > 0 {
            let volumes = row_to_volumes(&rows.get(0))?;
            return Ok(Some(volumes));
        }
        Ok(None)
    }

    pub fn show_by_assembly(db: &DataStoreConn, vol_get: &IdGet) -> VolumeOutputList {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_volumes_by_assembly_v1($1)", &[&(vol_get.get_id() as String)])
            .map_err(Error::VolumesGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_volumes(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn update(db: &DataStoreConn, volume: &volume::Volumes) -> VolumeOutput {
        let conn = db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_volume_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(volume.get_id().parse::<i64>().unwrap()),
                &(volume.get_mount_path() as String),
                &(volume.get_allocated() as String),
                &(serde_json::to_value(volume.get_status()).unwrap()),
                &(serde_json::to_value(volume.object_meta()).unwrap()),
                &(serde_json::to_value(volume.get_source()).unwrap()),
            ],
        ).map_err(Error::VolumeUpdate)?;

        if rows.len() > 0 {
            let volumes = row_to_volumes(&rows.get(0))?;
            return Ok(Some(volumes));
        }
        Ok(None)
    }
}

fn row_to_volumes(row: &postgres::rows::Row) -> Result<volume::Volumes> {
    let mut volumes = volume::Volumes::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    volumes.set_id(id.to_string());
    volumes.set_created_at(created_at.to_rfc3339());
    volumes.set_mount_path(row.get("mount_path"));
    volumes.set_allocated(row.get("allocated"));
    volumes.set_status(serde_json::from_value(row.get("status")).unwrap());
    volumes.set_source(serde_json::from_value(row.get("source")).unwrap());

    Ok(volumes)
}
