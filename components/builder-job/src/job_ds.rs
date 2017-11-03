// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{jobsrv,DEFAULT_API_VERSION};
use postgres;
use db::data_store::DataStoreConn;
use rio_net::metrics::prometheus::PrometheusClient;
use rio_net::metrics::collector::{Collector, CollectorScope};
use serde_json;
pub const JOB: &'static str = "JOB";


pub struct JobDS;

impl JobDS {

    pub fn jobs_create(datastore: &DataStoreConn, jobs_create: &jobsrv::Jobs) -> Result<Option<jobsrv::Jobs>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_jobs_v1($1,$2,$3,$4)",
            &[
                &(serde_json::to_string(jobs_create.get_spec()).unwrap()),
                &(serde_json::to_string(jobs_create.get_status()).unwrap()),
                &(serde_json::to_string(jobs_create.get_object_meta()).unwrap()),
                &(serde_json::to_string(jobs_create.get_type_meta()).unwrap()),

            ],
        ).map_err(Error::JobsCreate)?;

        if rows.len() > 0 {
        let jobs = row_to_jobs(&rows.get(0))?;
        return Ok(Some(jobs));
    }
    Ok(None)
    }

pub fn jobs_get(datastore: &DataStoreConn) -> Result<Option<jobsrv::JobGetResponse>> {
    let conn = datastore.pool.get_shard(0)?;

    let rows = &conn.query("SELECT * FROM get_jobs_v1()", &[])
        .map_err(Error::JobsGet)?;

    let mut response = jobsrv::JobGetResponse::new();

    let mut jobs_collection = Vec::new();

    if rows.len() > 0 {
        for row in rows {

            jobs_collection.push(row_to_jobs(&row)?)
        }
        response.set_jobs_collection(jobs_collection);
        return Ok(Some(response));
    }
    Ok(None)
}
pub fn jobs_status_update(datastore: &DataStoreConn, job: &jobsrv::Jobs) -> Result<Option<jobsrv::Jobs>> {
    let conn = datastore.pool.get_shard(0)?;
    let rows = &conn.query(
        "SELECT * FROM set_job_status_v1($1, $2)",
        &[
            &(job.get_id().parse::<i64>().unwrap()),
            &(serde_json::to_string(job.get_status()).unwrap()),
        ],
    ).map_err(Error:: JobSetStatus)?;
    if rows.len() > 0 {
        let jobs = row_to_jobs(&rows.get(0))?;
        return Ok(Some(jobs));
}
    Ok(None)
}


}

fn row_to_jobs(row: &postgres::rows::Row) -> Result<jobsrv::Jobs> {
    let mut job_create = jobsrv::Jobs::new();

    let id: i64 = row.get("id");
    let status: String = row.get("status");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");
    let spec: String = row.get("spec");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    job_create.set_id(id.to_string());
    job_create.set_spec(serde_json::from_str(&spec).unwrap());
    job_create.set_status(serde_json::from_str(&status).unwrap());
    job_create.set_object_meta(serde_json::from_str(&object_meta).unwrap());
    job_create.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    job_create.set_created_at(created_at.to_rfc3339());

    Ok(job_create)
}
