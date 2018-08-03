use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::base::{IdGet, MetaFields};
use protocol::api::origin;

use db::data_store::DataStoreConn;
use postgres;
use serde_json;

use super::{OriginOutput, OriginOutputList};

pub struct OriginDS;

impl OriginDS {
    pub fn create(
        datastore: &DataStoreConn,
        org_create: &origin::Origin,
    ) -> Result<Option<origin::Origin>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_origin_v1($1,$2,$3)",
            &[
                &(org_create.get_name() as String),
                &(serde_json::to_value(org_create.type_meta()).unwrap()),
                &(serde_json::to_value(org_create.object_meta()).unwrap()),
            ],
        ).map_err(Error::OriginCreate)?;
        if rows.len() > 0 {
            let origin = row_to_origin(&rows.get(0))?;
            return Ok(Some(origin));
        }
        Ok(None)
    }

    pub fn list_blank(datastore: &DataStoreConn) -> OriginOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_origins_v1()", &[])
            .map_err(Error::OriginGetResponse)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_origin(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }


    pub fn list(datastore: &DataStoreConn, get_account: &IdGet) -> OriginOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_origin_by_account_v1($1)",
            &[&(get_account.get_name() as String)]
            ).map_err(Error::OriginGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_origin(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, get_origin: &IdGet) -> OriginOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_origin_v1($1)", &[&get_origin.get_id()])
            .map_err(Error::OriginGet)?;
        if rows.len() > 0 {
            for row in rows {
                let origin = row_to_origin(&row)?;
                return Ok(Some(origin));
            }
        }
        Ok(None)
    }
}

fn row_to_origin(row: &postgres::rows::Row) -> Result<origin::Origin> {
    let mut origin_data = origin::Origin::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    origin_data.set_org_name(row.get("name"));
    origin_data.set_id(id.to_string() as String);
    origin_data.set_created_at(created_at.to_rfc3339());
    Ok(origin_data)
}
