// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct JobProcedures;

impl JobProcedures {
    pub fn new() -> Result<JobProcedures> {
        Ok(JobProcedures)
    }
}

impl Migratable for JobProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("Jobprocedure");

        migrator.migrate(
            "jobsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS job_id_seq;"#,
        )?;

        migrator.migrate(
            "jobsrv",
            r#"CREATE TABLE  IF NOT EXISTS jobs (
             id bigint PRIMARY KEY DEFAULT next_id_v1('job_id_seq'),
             spec jsonb,
             status text,
             object_meta text,
             type_meta text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] jobs");


        // Insert a new job into the jobs table
        migrator.migrate(
            "jobsrv",
            r#"CREATE OR REPLACE FUNCTION insert_jobs_v1 (
                spec jsonb,
                status text,
                object_meta text,
                type_meta text

            ) RETURNS SETOF jobs AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO jobs(spec,status,object_meta, type_meta)
                                        VALUES (spec,status, object_meta, type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "jobsrv",
            r#"CREATE OR REPLACE FUNCTION get_jobs_v1() RETURNS SETOF jobs AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM jobs;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "jobsrv",
            r#"CREATE OR REPLACE FUNCTION get_jobs_by_node_v1(node text) RETURNS SETOF jobs AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM jobs WHERE spec ->> 'node_id' = node ;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;


        migrator.migrate(
            "jobsrv",
            r#"CREATE OR REPLACE FUNCTION set_job_status_v1 (jid bigint, job_status text) RETURNS SETOF jobs AS $$
                            BEGIN
                                RETURN QUERY UPDATE jobs SET status=job_status, updated_at=now() WHERE id=jid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;



        ui.end("JobProcedure");

        Ok(())
    }
}
