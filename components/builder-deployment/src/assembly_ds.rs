// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{asmsrv, plansrv, servicesrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct AssemblyDS;

impl AssemblyDS {
    pub fn create(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14)",
            &[
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_origin() as String),
                &(assembly.get_tags() as Vec<String>),
                &(assembly.get_selector() as Vec<String>),
                &(assembly.get_node() as String),
                &(serde_json::to_string(assembly.get_urls()).unwrap()),
                &(serde_json::to_string(assembly.get_status()).unwrap()),
                &(serde_json::to_string(assembly.get_volumes()).unwrap()),
                &(assembly.get_instance_id() as String),
                &(serde_json::to_string(assembly.get_type_meta()).unwrap()),
                &(serde_json::to_string(assembly.get_object_meta()).unwrap()),
            ],
        ).map_err(Error::AssemblyCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_assembly_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(assembly.get_id().parse::<i64>().unwrap()),
                &(assembly.get_name() as String),
                &(assembly.get_uri() as String),
                &(assembly.get_description() as String),
                &(assembly.get_parent_id() as String),
                &(assembly.get_tags() as Vec<String>),
                &(assembly.get_node() as String),
                &(serde_json::to_string(assembly.get_urls()).unwrap()),
                &(serde_json::to_string(assembly.get_volumes()).unwrap()),
            ],
        ).map_err(Error::AssemblyUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, get_assembly: &asmsrv::IdGet) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_assembly_v1($1)",
            &[&(get_assembly.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyGet)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn) -> Result<Option<asmsrv::AssemblysGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_assemblys_v1()", &[])
            .map_err(Error::AssemblyGet)?;

        let mut response = asmsrv::AssemblysGetResponse::new();

        let mut assemblys_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                assemblys_collection.push(assembly);
            }
            response.set_assemblys(assemblys_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn show_by_origin(datastore: &DataStoreConn, assemblys_get: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblysGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblys_by_origin_v1($1)",
            &[&(assemblys_get.get_id() as String)],
        ).map_err(Error::AssemblyGet)?;

        let mut response = asmsrv::AssemblysGetResponse::new();

        let mut assemblys_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                assemblys_collection.push(assembly);
            }
            response.set_assemblys(assemblys_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn show_by_services(datastore: &DataStoreConn, assemblys_get: &asmsrv::IdGet) -> Result<Option<asmsrv::AssemblysGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblys_by_services_v1($1)",
            &[&(assemblys_get.get_id() as String)],
        ).map_err(Error::AssemblyGet)?;

        let mut response = asmsrv::AssemblysGetResponse::new();

        let mut assemblys_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                assemblys_collection.push(assembly);
            }
            response.set_assemblys(assemblys_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }


    pub fn status_update(datastore: &DataStoreConn, assembly: &asmsrv::Assembly) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_assembly_status_v1($1, $2)",
            &[
                &(assembly.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_string(assembly.get_status()).unwrap()),
            ],
        ).map_err(Error::AsmSetStatus)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = Self::collect_spec(&row, &datastore)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }


    fn collect_spec(row: &postgres::rows::Row, datastore: &DataStoreConn) -> Result<asmsrv::Assembly> {
        let mut assembly = row_to_assembly(&row)?;
        let mut asm_fac_get = asmsrv::IdGet::new();
        asm_fac_get.set_id(assembly.get_parent_id());
        let data = Self::assembly_factory_show(&datastore, &asm_fac_get)?;
        let mut endpoint_get = asmsrv::IdGet::new();
        endpoint_get.set_id(assembly.get_id());
        let endpoints = ServiceAccountDS::endpoints_show_by_asm_id(&datastore, &endpoint_get)
            .map_err(Error::EndPoints)?;
        assembly.set_spec(data);
        assembly.set_endpoints(endpoints);
        Ok(assembly)
    }

}

fn row_to_assembly(row: &postgres::rows::Row) -> Result<asmsrv::Assembly> {
    let mut assembly = asmsrv::Assembly::new();

    let id: i64 = row.get("id");
    let origin: i64 = row.get("origin_id");
    let urls: String = row.get("urls");
    let status: String = row.get("status");
    let node: String = row.get("node");
    let volume: String = row.get("volumes");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    assembly.set_id(id.to_string());
    assembly.set_name(row.get("name"));
    assembly.set_urls(serde_json::from_str(&urls).unwrap());
    assembly.set_uri(row.get("uri"));
    assembly.set_tags(row.get("tags"));
    assembly.set_selector(row.get("selector"));
    let mut obj: asmsrv::ObjectMeta = serde_json::from_str(&object_meta).unwrap();
    obj.set_name(id.to_string());
    assembly.set_object_meta(obj);
    assembly.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    assembly.set_description(row.get("description"));
    assembly.set_parent_id(row.get("parent_id"));
    assembly.set_origin(origin.to_string());
    assembly.set_status(serde_json::from_str(&status).unwrap());
    assembly.set_volumes(serde_json::from_str(&volume).unwrap());
    assembly.set_node(node as String);
    assembly.set_created_at(created_at.to_rfc3339());

    Ok(assembly)
}
