// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct ScaleProcedures;

impl ScaleProcedures {
    pub fn new() -> Result<ScaleProcedures> {
        Ok(ScaleProcedures)
    }
}

impl Migratable for ScaleProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("Scaleprocedure");
        // The core asms table
        migrator.migrate(
            "scalesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS hs_id_seq;"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE TABLE  IF NOT EXISTS horizontal_scaling (
             id bigint PRIMARY KEY DEFAULT next_id_v1('hs_id_seq'),
             name text,
             description text,
             tags text[],
             scale_type text,
             representation_skew text,
             state text,
             metadata text[],
             spec text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] horizontal_scaling");


        // Insert a new job into the jobs table
        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION insert_hs_v1 (
                name text,
                description text,
                tags text[],
                scale_type text,
                representation_skew text,
                state text,
                metadata text[],
                spec text,
                status text
                        ) RETURNS SETOF horizontal_scaling AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO horizontal_scaling(name,description,tags,scale_type,representation_skew,state,metadata,spec,status)
                                        VALUES (name,description,tags,scale_type,representation_skew,state,metadata,spec,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION get_hs_v1() RETURNS SETOF horizontal_scaling AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM horizontal_scaling;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION set_hs_status_v1 (hid bigint, hs_status text) RETURNS void AS $$
                            BEGIN
                                UPDATE horizontal_scaling SET status=hs_status, updated_at=now() WHERE id=hid;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION update_hs_v1 (hid bigint,name text,description text,tags text[],scale_type text,representation_skew text,state text,metadata text[],spec text) RETURNS void AS $$
                            BEGIN
                                UPDATE horizontal_scaling SET name=name, description=description,tags=tags, scale_type=scale_type, representation_skew=representation_skew, state=state,metadata=metadata,spec=spec, updated_at=now() WHERE id=hid;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;



        ui.end("Scaleprocedure");

        Ok(())
    }
}
