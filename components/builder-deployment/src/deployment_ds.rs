// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use postgres;
use protobuf;
use protocol::net::{NetOk, NetError, ErrCode};
use protocol::message::asmsrv::{Assembly, AssemblyGet};
use std::str::FromStr;
use protobuf::ProtobufEnum;
use db::config::DataStore;


pub struct DeploymentDS;

impl DeploymentDS {
    /// Create an assembly in the  database. If the assembly is created, we'll
    /// return the Assembly result.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the assembly cannot be created
    pub fn assembly_create(datastore: &DataStore, assembly: &Assembly) {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "INSERT INTO assembly($1, $2, $3, $4, $5)",
            &[&(assembly.get_id() as i64)],
        //TO-DO: Create custom errors AssemblyCreate
        ).map_err(Error::AssemblyCreate)?;

        Ok(Some(assembly));
    }

    pub fn get_assembly(get_assembly: &asmsrv::AssemblyGet) -> Result<Option<asmsrv::Assembly>> {
        let conn = self.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_job_v1($1)",
            &[&(get_job.get_id() as i64)],
        ).map_err(Error::JobGet)?;
        for row in rows {
            let job = row_to_job(&row)?;
            return Ok(Some(job));
        }
        Ok(None)
    }

}
