// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;


pub struct LinkersDS;

impl LinkersDS {
    pub fn create(datastore: &DataStoreConn, services_create: &servicesrv::Services) -> Result<Option<servicesrv::Services>> {
        let conn = datastore.pool.get_shard(0)?;
        let asmid = services_create.get_spec().get_selector().get(
            &servicesrv::RIO_ASM_FAC_ID.to_string(),
        );
        let rows = &conn.query(
            "SELECT * FROM insert_services_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(services_create.get_object_meta().get_origin() as String),
                &(asmid.unwrap().parse::<i64>().unwrap()),
                &(serde_json::to_string(services_create.get_spec()).unwrap()),
                &(serde_json::to_string(services_create.get_status()).unwrap()),
                &(serde_json::to_string(services_create.get_object_meta()).unwrap()),
                &(serde_json::to_string(services_create.get_type_meta()).unwrap()),
            ],
        ).map_err(Error::ServicesCreate)?;

        if rows.len() > 0 {
            for row in rows {
                let end = row_to_services(&row);
                return Ok(Some(end));
            }
        }
        Ok(None)

    }
    pub fn show(datastore: &DataStoreConn, services_get: &asmsrv::IdGet) -> Result<Option<servicesrv::Services>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_services_v1($1)",
            &[&(services_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ServicesGet)?;
        if rows.len() > 0 {
            for row in rows {
                let end = row_to_services(&row);
                return Ok(Some(end));
            }
        }
        Ok(None)
    }
    pub fn list(datastore: &DataStoreConn) -> Result<Option<servicesrv::ServicesGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_services_list_v1()", &[])
            .map_err(Error::ServicesGetResponse)?;

        let mut response = servicesrv::ServicesGetResponse::new();

        let mut services_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                services_collection.push(row_to_services(&row))
            }
            response.set_services_collection(services_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_by_origin(datastore: &DataStoreConn, services_get: &asmsrv::IdGet) -> Result<Option<servicesrv::ServicesGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_services_by_origin_v1($1)",
            &[&(services_get.get_id() as String)],
        ).map_err(Error::ServicesGetResponse)?;

        let mut response = servicesrv::ServicesGetResponse::new();

        let mut services_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                services_collection.push(row_to_services(&row))
            }
            response.set_services_collection(services_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn list_by_assembly_factory(datastore: &DataStoreConn, services_get: &asmsrv::IdGet) -> Result<Option<servicesrv::ServicesGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_services_by_assembly_factory_v1($1)",
            &[&(services_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ServicesGetResponse)?;

        let mut response = servicesrv::ServicesGetResponse::new();

        let mut services_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                services_collection.push(row_to_services(&row))
            }
            response.set_services_collection(services_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
}



fn row_to_services(row: &postgres::rows::Row) -> servicesrv::Services {
    let mut services = servicesrv::Services::new();
    let id: i64 = row.get("id");
    let spec: String = row.get("spec");
    let status: String = row.get("status");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    services.set_id(id.to_string());
    services.set_spec(serde_json::from_str(&spec).unwrap());
    services.set_status(serde_json::from_str(&status).unwrap());
    services.set_object_meta(serde_json::from_str(&object_meta).unwrap());
    services.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    services.set_created_at(created_at.to_rfc3339());

    services
}
