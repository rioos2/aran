// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::{Result, Error};
use migration::{Migratable, Migrator};

pub struct ScaleProcedures;

impl ScaleProcedures {
    pub fn new() -> Result<ScaleProcedures> {
        Ok(ScaleProcedures)
    }
}

impl Migratable for ScaleProcedures {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: scalesrv");
        // The core asms table
        migrator.migrate(
            "scalesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS hs_id_seq;"#,
        )?;

        debug!("=> [✓] hs_id_seq");

        migrator.migrate(
            "scalesrv",
            r#"CREATE TABLE  IF NOT EXISTS horizontal_scaling (
             id bigint PRIMARY KEY DEFAULT next_id_v1('hs_id_seq'),
             name text,
             description text,
             tags text[],
             hs_type text,
             representation_skew text,
             target_resource text,
             metadata text[],
             spec text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        debug!("=> [✓] horizontal_scaling");


        // Insert a new job into the jobs table
        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION insert_hs_v1 (
                name text,
                description text,
                tags text[],
                hs_type text,
                representation_skew text,
                target_resource text,
                metadata text[],
                spec text,
                status text
                        ) RETURNS SETOF horizontal_scaling AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO horizontal_scaling(name,description,tags,hs_type,representation_skew,target_resource,metadata,spec,status)
                                        VALUES (name,description,tags,hs_type,representation_skew,target_resource,metadata,spec,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;
        debug!("=> [✓] fn: insert_hs_v1");

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION get_hs_v1() RETURNS SETOF horizontal_scaling AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM horizontal_scaling;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_hs_v1");

        // The core plans table
        debug!("=> DONE: scalesrv");

        Ok(())
    }
}
