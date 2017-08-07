// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protocol::scalesrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use serde::Serialize;

pub struct ScalingDS;

impl ScalingDS {
    /// Create an assembly in the  database. If the assembly is created, we'll
    /// return the Assembly result.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the assembly cannot be created
    pub fn hs_create(datastore: &DataStoreConn, hs: &scalesrv::HorizontalScaling) -> Result<Option<scalesrv::HorizontalScaling>> {
        let conn = datastore.pool.get_shard(0)?;
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
                &(hs.get_spec_as_string() as String),
                &(hs.get_status() as String),
            ],
        ).map_err(Error::HSCreate)?;

        debug!(">● ROWS: hs_create =>\n{:?}", &rows);
        let hs = row_to_hs(&rows.get(0))?;
        debug!("◖☩ DONE: hs_create ");
        return Ok(Some(hs.clone()));
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
    let spec_str: String = row.get("spec");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    hs.set_id(id as u64);
    hs.set_name(name as String);
    hs.set_description(description as String);
    hs.set_tags(tags as Vec<String>);
    hs.set_hs_type(hs_type as String);
    hs.set_representation_skew(representation_skew as String);
    hs.set_target_resource(target_resource as String);
    hs.set_metadata(metadata as Vec<String>);
    hs.set_status(status as String);
    hs.set_spec_as_string(spec_str as String);
    // let back: scalesrv::Spec = serde_json::<scalesrv::Spec>::from_str(&spec_str).unwrap();
    // println!("-----------------------------------------{:?}", back);
    // hs.set_spec_as_obj(back);
    hs.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ ASM: row_to_hs =>\n{:?}", hs);
    debug!("◖☩ DONE: row_to_hs");
    Ok(hs)
}
