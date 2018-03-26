// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Build config
use chrono::prelude::*;
use error::{Result, Error};
use protocol::api::devtool::ImageMarks;
use protocol::api::base::MetaFields;
use protocol::api::base::IdGet;

use postgres;
use db::data_store::DataStoreConn;
use serde_json;

use super::super::{ImageMarksOutput, ImageMarksOutputList};

pub struct DataStore;

impl DataStore {
    pub fn create(datastore: &DataStoreConn, image_create: &ImageMarks) -> ImageMarksOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_image_marks_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(serde_json::to_value(image_create.get_tag()).unwrap()),
                &(image_create.get_generation() as i64),
                &(serde_json::to_value(image_create.get_conditions()).unwrap()),
                &(image_create.get_lookup_policy() as bool),
                &(serde_json::to_value(image_create.get_image()).unwrap()),
                &(serde_json::to_value(image_create.object_meta()).unwrap()),
                &(serde_json::to_value(image_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::ImageMarksCreate)?;
        if rows.len() > 0 {
            let image = row_to_image_marks(&rows.get(0))?;
            return Ok(Some(image.clone()));
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, img_get: &IdGet) -> ImageMarksOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_image_marks_v1($1)",
            &[&(img_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ImageMarksGet)?;

        if rows.len() > 0 {
            let image = row_to_image_marks(&rows.get(0))?;
            return Ok(Some(image));
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn) -> ImageMarksOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_image_marks_v1()", &[])
            .map_err(Error::ImageMarksGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_image_marks(&row)?)
            }
            return Ok(Some(response));
        }

        Ok(None)
    }
    pub fn update(db: &DataStoreConn, image_update: &ImageMarks) -> ImageMarksOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM update_image_marks_by_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(image_update.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(image_update.get_tag()).unwrap()),
                &(image_update.get_generation() as i64),
                &(serde_json::to_value(image_update.get_conditions()).unwrap()),
                &(image_update.get_lookup_policy() as bool),
                &(serde_json::to_value(image_update.get_image()).unwrap()),
                &(serde_json::to_value(image_update.object_meta()).unwrap()),
            ],
        ).map_err(Error::ImageMarksUpdate)?;


        if rows.len() > 0 {
            let image = row_to_image_marks(&rows.get(0))?;
            return Ok(Some(image));
        }
        Ok(None)
    }

    pub fn list_by_build(datastore: &DataStoreConn, img_get: &IdGet) -> ImageMarksOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_image_marks_by_build_v1($1)",
            &[&(img_get.get_id() as String)],
        ).map_err(Error::ImageMarksGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_image_marks(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_image_marks(row: &postgres::rows::Row) -> Result<ImageMarks> {
    let mut image_marks = ImageMarks::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    image_marks.set_id(id.to_string());
    image_marks.set_generation(row.get("generation"));
    image_marks.set_lookup_policy(row.get("lookup_policy"));
    image_marks.set_tag(serde_json::from_value(row.get("tags")).unwrap());
    image_marks.set_conditions(serde_json::from_value(row.get("conditions")).unwrap());
    image_marks.set_image(serde_json::from_value(row.get("image")).unwrap());
    image_marks.set_created_at(created_at.to_rfc3339());
    Ok(image_marks)
}
