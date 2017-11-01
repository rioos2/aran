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
             origin_id bigint REFERENCES origins(id),
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
                origin_name text,
                scale_type text,
                representation_skew text,
                state text,
                metadata text[],
                spec text,
                status text
                        ) RETURNS SETOF horizontal_scaling AS $$
                        DECLARE
                           this_origin origins%rowtype;
                                BEGIN
                                SELECT * FROM origins WHERE origins.name = origin_name LIMIT 1 INTO this_origin;
                                    RETURN QUERY INSERT INTO horizontal_scaling(name,description,tags,origin_id, scale_type,representation_skew,state,metadata,spec,status)
                                        VALUES (name,description,tags, this_origin.id,scale_type,representation_skew,state,metadata,spec,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_hs_by_origin_v1(org_name text) RETURNS SETOF horizontal_scaling AS $$
                DECLARE
                this_origin origins%rowtype;
                        BEGIN
                         SELECT * FROM origins WHERE origins.name = org_name LIMIT 1 INTO this_origin;
                         RETURN QUERY SELECT * FROM horizontal_scaling WHERE origin_id=this_origin.id;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
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
            r#"CREATE OR REPLACE FUNCTION set_hs_status_v1 (hid bigint, hs_status text) RETURNS SETOF horizontal_scaling AS $$
                            BEGIN
                                RETURN QUERY UPDATE horizontal_scaling SET status=hs_status, updated_at=now() WHERE id=hid;
                            RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION update_hs_v1 (
                hid bigint,
                hs_name text,
                hs_description text,
                hs_tags text[],
                hs_scale_type text,
                hs_representation_skew text,
                hs_state text,
                hs_metadata text[],
                hs_spec text) RETURNS SETOF horizontal_scaling AS $$
                            BEGIN
                                RETURN QUERY UPDATE horizontal_scaling SET name=hs_name, description=hs_description,tags=hs_tags, scale_type=hs_scale_type, representation_skew=hs_representation_skew, state=hs_state,metadata=hs_metadata,spec=hs_spec, updated_at=now() WHERE id=hid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;



        ui.end("Scaleprocedure");

        Ok(())
    }
}
