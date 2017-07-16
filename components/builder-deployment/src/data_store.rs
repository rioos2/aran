// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the DeploymentDS.

use chrono::{DateTime, UTC};
use config::Config;
use db::async::{AsyncServer, EventOutcome};
use db::error::{Error as DbError, Result as DbResult};
use db::pool::Pool;
use error::{Result, Error};
use hab_net::routing::Broker;
use postgres;
use protobuf;
use protocol::net::{NetOk, NetError, ErrCode};
use protocol::{originsrv, jobsrv, scheduler};
use std::str::FromStr;
use protobuf::ProtobufEnum;

use db::config::DataStoreCfg;

//TO-DO: 1. Copy this DataStore (struct, impl Drop, new, from_pool)  to buidler-api
//TO-DO: 2. Rename this  DataStore struct to DeploymentDS, with one field  datastore: DataStore

/// DataStore inherints being Send + Sync by virtue of having only one member, the pool itself.
#[derive(Debug, Clone)]
pub struct DataStore {
    pool: Pool,
    pub async: AsyncServer,
}

impl Drop for DataStore {
    fn drop(&mut self) {
        self.async.stop();
    }
}

impl DataStore {
    /// Create a new DataStore.
    ///
    /// * Can fail if the pool cannot be created
    /// * Blocks creation of the datastore on the existince of the pool; might wait indefinetly.
    pub fn new(config: &Config) -> Result<DataStore> {
        let pool = Pool::new(&config.datastore, config.shards.clone())?;
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }

    /// Create a new DataStore from a pre-existing pool; useful for testing the database.
    pub fn from_pool(pool: Pool) -> Result<DataStore> {
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }

    /// Setup the datastore.
    ///
    /// This includes all the schema, along with stored procedures for data
    /// access.
    pub fn setup(&self) -> Result<()> {
        // let conn = self.pool.get_raw()?;
        // let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        //
        // self.async.register("sync_jobs".to_string(), sync_jobs);

        Ok(())
    }

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }

    /// Create a new job. Sets the state to Pending.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the job cannot be created
    /// * If the job has an unknown VCS type
    pub fn create_job(&self, job: &jobsrv::Job) -> Result<jobsrv::Job> {

        let conn = self.pool.get_shard(0)?;

        if job.get_project().get_vcs_type() == "git" {
            let project = job.get_project();

            let rows = conn.query(
                "SELECT * FROM insert_job_v1($1, $2, $3, $4, $5, $6, $7)",
                &[
                    &(job.get_owner_id() as i64),
                    &(project.get_id() as i64),
                    &project.get_name(),
                    &(project.get_owner_id() as i64),
                    &project.get_plan_path(),
                    &project.get_vcs_type(),
                    &vec![project.get_vcs_data()],
                ],
            ).map_err(Error::JobCreate)?;
            let job = row_to_job(&rows.get(0))?;
            return Ok(job);
        } else {
            return Err(Error::UnknownVCS);
        }
    }

    /// Create an assembly in the  database. If the assembly is created, we'll
    /// return the Assembly result.
    ///
    /// # Errors
    ///
    /// * If the pool has no connections available
    /// * If the assembly cannot be created
    pub fn assembly_create(datastore: Datastore, assembly: &asmsrv::Assembly) {
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
