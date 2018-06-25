// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Deployment - Endpoint
use super::super::{EndPointOutput, EndPointOutputList};
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::{Error, Result};
use postgres;
use protocol::api::base::MetaFields;
use protocol::api::{base, endpoints};
use serde_json;

pub struct DataStore;

impl DataStore {
    pub fn create(db: &DataStoreConn, endpoints_create: &endpoints::EndPoints) -> EndPointOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_endpoints_v1($1,$2,$3)",
            &[
                &(serde_json::to_value(endpoints_create.get_subsets()).unwrap()),
                &(serde_json::to_value(endpoints_create.object_meta()).unwrap()),
                &(serde_json::to_value(endpoints_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::EndPointsCreate)?;
        if rows.len() > 0 {
            let end = row_to_endpoints(&rows.get(0))?;
            return Ok(Some(end));
        }
        Ok(None)
    }

    pub fn list_blank(db: &DataStoreConn) -> EndPointOutputList {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_endpoints_v1()", &[])
            .map_err(Error::EndPointsGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_endpoints(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn show(db: &DataStoreConn, endpoints_get: &base::IdGet) -> EndPointOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_endpoint_v1($1)",
            &[&(endpoints_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::EndPointsGet)?;
        if rows.len() > 0 {
            for row in rows {
                let end = row_to_endpoints(&row)?;
                return Ok(Some(end));
            }
        }
        Ok(None)
    }

    pub fn list(db: &DataStoreConn, endpoints_get: &base::IdGet) -> EndPointOutputList {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_endpoints_by_account_v1($1)",
            &[&(endpoints_get.get_id() as String)],
        ).map_err(Error::EndPointsGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_endpoints(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn show_by_assembly(db: &DataStoreConn, endpoints_get: &base::IdGet) -> EndPointOutput {
        let conn = db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_endpoints_by_assebmly_v1($1)",
            &[&(endpoints_get.get_id() as String)],
        ).map_err(Error::EndPointsGet)?;

        if rows.len() > 0 {
            for row in rows {
                let response = row_to_endpoints(&row)?;
                return Ok(Some(response));
            }
        }
        Ok(Some(endpoints::EndPoints::new()))
    }
}

fn row_to_endpoints(row: &postgres::rows::Row) -> Result<endpoints::EndPoints> {
    let mut endpoints = endpoints::EndPoints::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    endpoints.set_id(id.to_string());
    endpoints.set_subsets(serde_json::from_value(row.get("subsets")).unwrap());
    endpoints.set_created_at(created_at.to_rfc3339());

    Ok(endpoints)
}
