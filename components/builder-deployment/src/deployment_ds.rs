// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protocol::asmsrv;
use postgres;
use db::data_store::DataStoreConn;

pub struct DeploymentDS;

impl DeploymentDS {
    /// Create an assembly in the  database. If the assembly is created, we'll
    /// return the Assembly result.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the assembly cannot be created
    pub fn assembly_create(datastore: &DataStoreConn, assembly: &asmsrv::Assembly)->Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assemby_create ");

        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1, $2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[&(assembly.get_name() as String),&(assembly.get_uri() as String),&(assembly.get_description() as String),&(assembly.get_tags() as Vec<String>),&(assembly.get_representation_skew() as String),&(assembly.get_external_management_resource() as String),&(assembly.get_component_collection() as Vec<String>),&(assembly.get_plan() as String),&(assembly.get_operation_collection() as Vec<String>),&(assembly.get_sensor_collection() as Vec<String>),&(assembly.get_metadata() as String)],
        ).map_err(Error::AssemblyCreate)?;

        debug!(">● ROWS: assemby_create =>\n{:?}",&rows);
        let assembly = row_to_assembly(&rows.get(0))?;
        debug!("◖☩ DONE: assemby_create ");
        return Ok(Some(assembly.clone()));
    }

    pub fn assembly_show(datastore: &DataStoreConn, get_assembly: &asmsrv::AssemblyGet) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        debug!("◖☩ START: assemby_show {:?}", get_assembly.get_id());


        let rows = &conn.query(
            "SELECT * FROM get_assembly_v1($1)",
            &[&(get_assembly.get_id() as i64)],
        ).map_err(Error::AssemblyGet)?;

        debug!(">● ROWS: assemby_show =>\n{:?}",&rows);

        for row in rows {
            let job = row_to_assembly(&row)?;
            return Ok(Some(job));
        }

        Ok(None)
    }
}

fn row_to_assembly(row: &postgres::rows::Row) -> Result<asmsrv::Assembly> {
    let mut assembly = asmsrv::Assembly::new();
    debug!("◖☩ START: row_to_assemby");
    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let plan: String = row.get("plan");
    let uri: String = row.get("uri");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let representation_skew: String = row.get("representation_skew");
    let external_management_resource: String = row.get("external_management_resource");
    let component_collection: Vec<String> = row.get("component_collection");
    let operation_collection: Vec<String> = row.get("operation_collection");
    let sensor_collection: Vec<String> = row.get("sensor_collection");
    let metadata: String = row.get("metadata");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    assembly.set_id(id as u64);
    assembly.set_name(name as String);
    assembly.set_plan(plan as String);
    assembly.set_uri(uri as String);
    assembly.set_description(description as String);
    assembly.set_tags(tags as Vec<String>);
    assembly.set_component_collection(component_collection as Vec<String>);
    assembly.set_operation_collection(operation_collection as Vec<String>);
    assembly.set_sensor_collection(sensor_collection as Vec<String>);
    assembly.set_metadata(metadata as String);
    assembly.set_representation_skew(representation_skew as String);
    assembly.set_external_management_resource(external_management_resource as String);
    assembly.set_created_at(created_at.to_rfc3339());
    debug!("◖☩ ASM: row_to_assemby =>\n{:?}", assembly);

    debug!("◖☩ DONE: row_to_assemby");
    Ok(assembly)
}
