// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The PostgreSQL backend for the Jobsrv.

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
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(Error::DbTransactionStart)?;

        self.async.register("sync_jobs".to_string(), sync_jobs);

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

    /// Get a job from the database. If the job does not exist, but the database was active, we'll
    /// get a None result.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the job cannot be selected from the database
    pub fn get_job(&self, get_job: &jobsrv::JobGet) -> Result<Option<jobsrv::Job>> {
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

    /// Get the 50 most recently-created jobs for a given project
    /// (specified as an origin-qualified name, e.g., "core/nginx").
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If a row returned cannot be translated into a Job
    pub fn get_jobs_for_project(
        &self,
        project: &jobsrv::ProjectJobsGet,
    ) -> Result<jobsrv::ProjectJobsGetResponse> {
        let conn = self.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_jobs_for_project_v1($1)",
            &[&(project.get_name())],
        ).map_err(Error::ProjectJobsGet)?;

        let mut response = jobsrv::ProjectJobsGetResponse::new();
        let mut jobs = protobuf::RepeatedField::new();
        for row in rows {
            jobs.push(row_to_job(&row)?)
        }
        response.set_jobs(jobs);
        Ok(response)
    }

    /// Get a list of pending jobs, up to a maximum count of jobs.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the pending jobs cannot be selected from the database
    /// * If the row returned cannot be translated into a Job
    pub fn pending_jobs(&self, count: i32) -> Result<Vec<jobsrv::Job>> {
        let mut jobs = Vec::new();
        let conn = self.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM pending_jobs_v1($1)", &[&count])
            .map_err(Error::JobPending)?;
        for row in rows {
            let job = row_to_job(&row)?;
            jobs.push(job);
        }
        Ok(jobs)
    }

    /// Reset any Dispatched jobs back to Pending state
    /// This is used for recovery scenario
    ///
    /// # Errors
    /// * If a connection cannot be gotten from the pool
    /// * If the dispatched jobs cannot be selected from the database
    pub fn reset_jobs(&self) -> Result<()> {
        let conn = self.pool.get_shard(0)?;
        conn.query("SELECT reset_jobs_v1()", &[]).map_err(
            Error::JobReset,
        )?;
        Ok(())
    }

    /// Updates a job. Currently, this entails updating the state,
    /// build start and stop times, and recording the identifier of
    /// the package the job produced, if any.
    ///
    /// # Errors
    ///
    /// * If a connection cannot be gotten from the pool
    /// * If the job cannot be updated in the database
    pub fn update_job(&self, job: &jobsrv::Job) -> Result<()> {
        let conn = self.pool.get_shard(0)?;
        let job_id = job.get_id() as i64;
        let job_state = match job.get_state() {
            jobsrv::JobState::Dispatched => "Dispatched",
            jobsrv::JobState::Pending => "Pending",
            jobsrv::JobState::Processing => "Processing",
            jobsrv::JobState::Complete => "Complete",
            jobsrv::JobState::Rejected => "Rejected",
            jobsrv::JobState::Failed => "Failed",
        };

        // Note: the following fields may all be NULL. As currently
        // coded, if they are NULL, then the corresponding fields in
        // the database will also be updated to be NULL. This should
        // be OK, though, because they shouldn't be changing anyway.
        let build_started_at = match job.has_build_started_at() {
            true => Some(
                DateTime::<UTC>::from_str(job.get_build_started_at()).unwrap(),
            ),
            false => None,
        };
        let build_finished_at = match job.has_build_finished_at() {
            true => Some(
                DateTime::<UTC>::from_str(job.get_build_finished_at()).unwrap(),
            ),
            false => None,
        };
        let ident = match job.has_package_ident() {
            true => Some(job.get_package_ident().to_string()),
            false => None,
        };

        let (err_code, err_msg) = if job.has_error() {
            (
                Some(job.get_error().get_code() as i32),
                Some(job.get_error().get_msg()),
            )
        } else {
            (None, None)
        };

        conn.execute(
            "SELECT update_job_v2($1, $2, $3, $4, $5, $6, $7)",
            &[
                &job_id,
                &job_state,
                &build_started_at,
                &build_finished_at,
                &ident,
                &err_code,
                &err_msg,
            ],
        ).map_err(Error::JobSetState)?;

        self.async.schedule("sync_jobs")?;

        Ok(())
    }

    /// Marks a given job's logs as having been archived. The location
    /// and mechanism for retrieval are dependent on the configured archiving
    /// mechanism.
    pub fn mark_as_archived(&self, job_id: u64) -> Result<()> {
        let conn = self.pool.get_shard(0)?;
        conn.execute("SELECT mark_as_archived_v1($1)", &[&(job_id as i64)])
            .map_err(Error::JobMarkArchived)?;
        Ok(())
    }
}

/// Translate a database `jobs` row to a `jobsrv::Job`.
///
/// # Errors
///
/// * If the job state is unknown
/// * If the VCS type is unknown
fn row_to_job(row: &postgres::rows::Row) -> Result<jobsrv::Job> {
    let mut job = jobsrv::Job::new();
    let id: i64 = row.get("id");
    job.set_id(id as u64);
    let owner_id: i64 = row.get("owner_id");
    job.set_owner_id(owner_id as u64);
    let js: String = row.get("job_state");
    let job_state = match &js[..] {
        "Dispatched" => jobsrv::JobState::Dispatched,
        "Pending" => jobsrv::JobState::Pending,
        "Processing" => jobsrv::JobState::Processing,
        "Complete" => jobsrv::JobState::Complete,
        "Rejected" => jobsrv::JobState::Rejected,
        "Failed" => jobsrv::JobState::Failed,
        _ => return Err(Error::UnknownJobState),
    };
    job.set_state(job_state);

    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    job.set_created_at(created_at.to_rfc3339());

    // Note: these may be null (e.g., a job is scheduled, but hasn't
    // started; a job has started and is currently running)
    if let Some(Ok(start)) = row.get_opt::<&str, DateTime<UTC>>("build_started_at") {
        job.set_build_started_at(start.to_rfc3339());
    }
    if let Some(Ok(stop)) = row.get_opt::<&str, DateTime<UTC>>("build_finished_at") {
        job.set_build_finished_at(stop.to_rfc3339());
    }

    // package_ident will only be present if the build succeeded
    if let Some(Ok(ident_str)) = row.get_opt::<&str, String>("package_ident") {
        let ident: originsrv::OriginPackageIdent = ident_str.parse().unwrap();
        job.set_package_ident(ident);
    }

    let mut project = originsrv::OriginProject::new();
    let project_id: i64 = row.get("project_id");
    project.set_id(project_id as u64);

    // only 'project_name' exists in the jobs table, but it's just
    // "origin/name", so we can set those fields in the Project
    // struct.
    //
    // 'package_ident' may be null, though, so we shouldn't use it to
    // get the origin and name.
    let name: String = row.get("project_name");
    let name_for_split = name.clone();
    let name_split: Vec<&str> = name_for_split.split("/").collect();
    project.set_origin_name(name_split[0].to_string());
    project.set_package_name(name_split[1].to_string());
    project.set_name(name);

    let project_owner_id: i64 = row.get("project_owner_id");
    project.set_owner_id(project_owner_id as u64);
    project.set_plan_path(row.get("project_plan_path"));

    let rvcs: String = row.get("vcs");
    match rvcs.as_ref() {
        "git" => {
            let mut vcsa: Vec<String> = row.get("vcs_arguments");
            project.set_vcs_type(String::from("git"));
            project.set_vcs_data(vcsa.remove(0));
        }
        e => {
            error!("Unknown VCS, {}", e);
            return Err(Error::UnknownVCS);
        }
    }
    job.set_project(project);

    if let Some(Ok(err_msg)) = row.get_opt::<&str, String>("net_error_msg") {
        let err_code: i32 = row.get("net_error_code");
        let mut err = NetError::new();

        if let Some(net_err_code) = ErrCode::from_i32(err_code) {
            err.set_code(net_err_code);
            err.set_msg(err_msg);
            job.set_error(err);
        }
    }

    job.set_is_archived(row.get("archived"));

    Ok(job)
}

fn sync_jobs(pool: Pool) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query("SELECT * FROM sync_jobs_v1()", &[]).map_err(
            DbError::AsyncFunctionCheck,
        )?;
        if rows.len() > 0 {
            let mut bconn = Broker::connect()?;
            let mut request = scheduler::JobStatus::new();
            for row in rows.iter() {
                let job = match row_to_job(&row) {
                    Ok(job) => job,
                    Err(e) => {
                        warn!("Failed to convert row to job {}", e);
                        return Ok(EventOutcome::Retry);
                    }
                };
                let id = job.get_id();
                request.set_job(job);
                match bconn.route::<scheduler::JobStatus, NetOk>(&request) {
                    Ok(_) => {
                        conn.query("SELECT * FROM set_jobs_sync_v1($1)", &[&(id as i64)])
                            .map_err(DbError::AsyncFunctionUpdate)?;
                        debug!("Updated scheduler service with job status, {:?}", request);
                    }
                    Err(e) => {
                        warn!(
                            "Failed to sync job status with the scheduler service, {:?}: {}",
                            request,
                            e
                        );
                        result = EventOutcome::Retry;
                    }
                }
            }
        }
    }
    Ok(result)
}
