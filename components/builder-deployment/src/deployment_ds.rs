// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protobuf;
use protocol::net::{NetOk, NetError, ErrCode};
use protocol::message::asmsrv;
use std::str::FromStr;
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
            "INSERT INTO assembly(name) values($1)",
            &[&(assembly.get_name() as String)],
        ).map_err(Error::AssemblyCreate)?;

        Ok(Some(assembly.clone()))
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
