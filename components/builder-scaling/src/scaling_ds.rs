// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protocol::scalesrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct ScalingDS;

impl ScalingDS {
    pub fn hs_create(datastore: &DataStoreConn, hs: &scalesrv::HorizontalScaling) -> Result<Option<scalesrv::HorizontalScaling>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(hs.get_spec()).unwrap();
        let status_str = serde_json::to_string(hs.get_status()).unwrap();
        debug!("◖☩ START: hs_create ");
        let rows = &conn.query(
            "SELECT * FROM insert_hs_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(hs.get_name() as String),
                &(hs.get_description() as String),
                &(hs.get_tags() as Vec<String>),
                &(hs.get_hs_type() as String),
                &(hs.get_representation_skew() as String),
                &(hs.get_target_resource() as String),
                &(hs.get_metadata() as Vec<String>),
                &(spec_str as String),
                &(status_str as String),
            ],
        ).map_err(Error::HSCreate)?;

        debug!(">● ROWS: hs_create =>\n{:?}", &rows);
        let hs = row_to_hs(&rows.get(0))?;
        debug!("◖☩ DONE: hs_create ");
        return Ok(Some(hs.clone()));
    }

    pub fn hs_list(datastore: &DataStoreConn) -> Result<Option<scalesrv::HorizontalScalingGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_hs_v1()", &[]).map_err(
            Error::HSGet,
        )?;

        let mut response = scalesrv::HorizontalScalingGetResponse::new();

        let mut hs_collection = Vec::new();

        debug!(">● ROWS: assemby_list =>\n{:?}", &rows);
        for row in rows {
            hs_collection.push(row_to_hs(&row)?)
        }
        response.set_hs_collection(hs_collection);
        Ok(Some(response))
    }

    pub fn hs_status_update(datastore: &DataStoreConn, hs: &scalesrv::HorizontalScaling) -> Result<()> {
        let conn = datastore.pool.get_shard(0)?;
        let id = hs.get_id() as i64;
        let status_str = serde_json::to_string(hs.get_status()).unwrap();
        conn.execute(
            "SELECT set_hs_status_v1($1, $2)",
            &[&id, &(status_str as String)],
        ).map_err(Error::HSSetStatus)?;
        Ok(())
    }
}

fn row_to_hs(row: &postgres::rows::Row) -> Result<scalesrv::HorizontalScaling> {
    let mut hs = scalesrv::HorizontalScaling::new();
    debug!("◖☩ START: row_to_hs");

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let hs_type: String = row.get("hs_type");
    let representation_skew: String = row.get("representation_skew");
    let target_resource: String = row.get("target_resource");
    let metadata: Vec<String> = row.get("metadata");
    let status: String = row.get("status");
    let spec: String = row.get("spec");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    hs.set_id(id as u64);
    hs.set_name(name as String);
    hs.set_description(description as String);
    hs.set_tags(tags as Vec<String>);
    hs.set_hs_type(hs_type as String);
    hs.set_representation_skew(representation_skew as String);
    hs.set_target_resource(target_resource as String);
    hs.set_metadata(metadata as Vec<String>);
    let spec_obj: scalesrv::Spec = serde_json::from_str(&spec).unwrap();
    let status_obj: scalesrv::Status = serde_json::from_str(&status).unwrap();
    hs.set_spec(spec_obj);
    hs.set_status(status_obj);
    hs.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ ASM: row_to_hs =>\n{:?}", hs);
    debug!("◖☩ DONE: row_to_hs");
    Ok(hs)
}
