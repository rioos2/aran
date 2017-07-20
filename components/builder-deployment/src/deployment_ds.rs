// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protobuf;
use protocol::net::{NetOk, NetError, ErrCode};
use protocol::asmsrv;
use std::str::FromStr;
use postgres;
use protobuf::ProtobufEnum;
use db::data_store::DataStoreConn;
use db::error::{Error as DbError, Result as DbResult};

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
        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1, $2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[&(assembly.get_name() as String),&(assembly.get_uri() as String),&(assembly.get_description() as String),&(assembly.get_tags() as String),&(assembly.get_representation_skew() as String),&(assembly.get_external_management_resource() as String),&(assembly.get_component_collection() as String),&(assembly.get_plan() as String),&(assembly.get_operation_collection() as String),&(assembly.get_sensor_collection() as String),&(assembly.get_metadata() as String)],
        ).map_err(Error::AssemblyCreate)?;
        info!(".........................................{:?}",&rows);
        let assembly = row_to_assembly(&rows.get(0))?;
        return Ok(Some(assembly.clone()));
    }

    pub fn assembly_show(datastore: &DataStoreConn, get_assembly: &asmsrv::AssemblyGet) -> Result<Option<asmsrv::Assembly>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM assembly($1)",
            &[&(get_assembly.get_id() as i64)],
        ).map_err(Error::AssemblyGet)?;
        /*for row in rows {
            let job = row_to_job(&row)?;
            return Ok(Some(job));
        }*/
        Ok(None)
    }
}

fn row_to_assembly(row: &postgres::rows::Row) -> Result<asmsrv::Assembly> {
    let mut assembly = asmsrv::Assembly::new();
    let id: i64 = row.get("id");
    assembly.set_id(id as u64);

    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    assembly.set_created_at(created_at.to_rfc3339());

    let updated_at = row.get::<&str, DateTime<UTC>>("updated_at");
    assembly.set_updated_at(updated_at.to_rfc3339());

    Ok(assembly)
}
