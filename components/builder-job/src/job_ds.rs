// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use chrono::prelude::*;

use error::{Result, Error};
use protocol::api::job;
use protocol::api::base::{IdGet, MetaFields, StatusUpdate};

use postgres;
use db::data_store::DataStoreConn;
use serde_json;

use super::{JobOutput, JobOutputList};

pub struct JobDS;

impl JobDS {
    pub fn create(datastore: &DataStoreConn, jobs_create: &job::Jobs) -> JobOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_jobs_v1($1,$2,$3,$4)",
            &[
                &(serde_json::to_value(jobs_create.get_spec()).unwrap()),
                &(serde_json::to_value(jobs_create.get_status()).unwrap()),
                &(serde_json::to_value(jobs_create.object_meta()).unwrap()),
                &(serde_json::to_value(jobs_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::JobsCreate)?;

        if rows.len() > 0 {
            let jobs = row_to_jobs(&rows.get(0))?;
            return Ok(Some(jobs));
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn) -> JobOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_jobs_v1()", &[])
            .map_err(Error::JobsGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_jobs(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn status_update(datastore: &DataStoreConn, job: &StatusUpdate) -> JobOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_job_status_v1($1, $2)",
            &[
                &(job.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(job.get_status()).unwrap()),
            ],
        ).map_err(Error::JobSetStatus)?;
        if rows.len() > 0 {
            let jobs = row_to_jobs(&rows.get(0))?;
            return Ok(Some(jobs));
        }
        Ok(None)
    }

    pub fn show_by_node(datastore: &DataStoreConn, job_get: &IdGet) -> JobOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_jobs_by_node_v1($1)",
            &[&(job_get.get_id() as String)],
        ).map_err(Error::JobsGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_jobs(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, job_get: &IdGet) -> JobOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_job_v1($1)",
            &[&(job_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::JobsGet)?;

        if rows.len() > 0 {
            let jobs = row_to_jobs(&rows.get(0))?;
            return Ok(Some(jobs));
        }
        Ok(None)
    }
}

fn row_to_jobs(row: &postgres::rows::Row) -> Result<job::Jobs> {
    let mut job_create = job::Jobs::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    job_create.set_id(id.to_string());
    job_create.set_created_at(created_at.to_rfc3339());
    job_create.set_spec(serde_json::from_value(row.get("spec")).unwrap());
    job_create.set_status(serde_json::from_value(row.get("status")).unwrap());

    Ok(job_create)
}
