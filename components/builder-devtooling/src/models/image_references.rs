// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Build config
use chrono::prelude::*;
use error::{Result, Error};
use protocol::api::devtool::ImageReferences;
use protocol::api::base::MetaFields;
use protocol::api::base::IdGet;

use postgres;
use db::data_store::DataStoreConn;
use serde_json;

use super::super::{ImageReferencesOutput, ImageReferencesOutputList};

pub struct DataStore;

impl DataStore {
    pub fn create(datastore: &DataStoreConn, image_create: &ImageReferences) -> ImageReferencesOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_image_ref_v1($1,$2,$3,$4)",
            &[
                &(serde_json::to_value(image_create.get_status()).unwrap()),
                &(serde_json::to_value(image_create.get_spec()).unwrap()),
                &(serde_json::to_value(image_create.object_meta()).unwrap()),
                &(serde_json::to_value(image_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::ImageRefCreate)?;
        if rows.len() > 0 {
            let image = row_to_image_ref(&rows.get(0))?;
            return Ok(Some(image.clone()));
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, img_get: &IdGet) -> ImageReferencesOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_image_ref_v1($1)",
            &[&(img_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ImageRefGet)?;

        if rows.len() > 0 {
            let image = row_to_image_ref(&rows.get(0))?;
            return Ok(Some(image));
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn) -> ImageReferencesOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_image_ref_by_v1()", &[])
            .map_err(Error::ImageRefGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_image_ref(&row)?)
            }
            return Ok(Some(response));
        }

        Ok(None)
    }
    pub fn update(db: &DataStoreConn, image_update: &ImageReferences) -> ImageReferencesOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM update_image_ref_by_v1($1,$2,$3,$4)",
            &[
                &(image_update.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(image_update.get_spec()).unwrap()),
                &(serde_json::to_value(image_update.get_status()).unwrap()),
                &(serde_json::to_value(image_update.object_meta()).unwrap()),
            ],
        ).map_err(Error::ImageRefUpdate)?;


        if rows.len() > 0 {
            let image = row_to_image_ref(&rows.get(0))?;
            return Ok(Some(image));
        }
        Ok(None)
    }
    pub fn show_by_build_config(datastore: &DataStoreConn, img_get: &IdGet) -> ImageReferencesOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_image_ref_by_build_config_v1($1)",
            &[&(img_get.get_id() as String)],
        ).map_err(Error::ImageRefGet)?;

        if rows.len() > 0 {
            let image = row_to_image_ref(&rows.get(0))?;
            return Ok(Some(image));
        }
        Ok(None)
    }
}

fn row_to_image_ref(row: &postgres::rows::Row) -> Result<ImageReferences> {
    let mut imageref = ImageReferences::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    imageref.set_id(id.to_string());
    imageref.set_status(serde_json::from_value(row.get("status")).unwrap());
    imageref.set_created_at(created_at.to_rfc3339());
    Ok(imageref)
}
