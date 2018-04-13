// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::linker;
use protocol::api::base::{IdGet, MetaFields};

use postgres;
use db::data_store::DataStoreConn;

use serde_json;

use super::super::{ServiceOutput, ServiceOutputList};

pub struct DataStore;

impl DataStore {
    pub fn create(db: &DataStoreConn, services_create: &linker::Services) -> ServiceOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_services_v1($1,$2,$3,$4,$5)",
            &[
                &(serde_json::to_value(services_create.get_spec()).unwrap()),
                &(serde_json::to_value(services_create.get_metadata()).unwrap()),
                &(serde_json::to_value(services_create.get_status()).unwrap()),
                &(serde_json::to_value(services_create.object_meta()).unwrap()),
                &(serde_json::to_value(services_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::ServicesCreate)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(row_to_services(&row)));
            }
        }
        Ok(None)
    }

    pub fn show(db: &DataStoreConn, services_get: &IdGet) -> ServiceOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_services_v1($1)",
            &[&(services_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ServicesGet)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(row_to_services(&row)));
            }
        }
        Ok(None)
    }

    pub fn list_blank(db: &DataStoreConn) -> ServiceOutputList {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_services_list_v1()", &[])
            .map_err(Error::ServicesGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_services(&row))
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_by_assembly_factory(db: &DataStoreConn, services_get: &IdGet) -> ServiceOutputList {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_services_by_assembly_factory_v1($1)",
            &[&(services_get.get_id() as String)],
        ).map_err(Error::ServicesGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_services(&row))
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn update(db: &DataStoreConn, service: &linker::Services) -> Result<Option<linker::Services>> {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM update_servive_by_v1($1,$2,$3,$4,$5)",
            &[
                &(service.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(service.get_spec()).unwrap()),
                &(serde_json::to_value(service.get_metadata()).unwrap()),
                &(serde_json::to_value(service.get_status()).unwrap()),
                &(serde_json::to_value(service.object_meta()).unwrap()),
            ],
        ).map_err(Error::ServicesUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let end = row_to_services(&row);
                return Ok(Some(end));
            }
        }
        Ok(None)
    }
}

fn row_to_services(row: &postgres::rows::Row) -> linker::Services {
    let mut services = linker::Services::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    services.set_id(id.to_string());
    services.set_spec(serde_json::from_value(row.get("spec")).unwrap());
    services.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    services.set_status(serde_json::from_value(row.get("status")).unwrap());
    services.set_created_at(created_at.to_rfc3339());

    services
}
