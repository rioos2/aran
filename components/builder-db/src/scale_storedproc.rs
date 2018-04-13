// Copyright 2018 The Rio Advancement Inc

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
            r#"CREATE TABLE  IF NOT EXISTS horizontal_scalings (
             id bigint PRIMARY KEY DEFAULT next_id_v1('hs_id_seq'),
             scale_type text,
             state text,
             metadata jsonb,
             spec jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] horizontal_scalings");

        // Insert a new job into the jobs table
        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION insert_hs_v1 (
                scale_type text,
                state text,
                metadata jsonb,
                spec jsonb,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
                        ) RETURNS SETOF horizontal_scalings AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO horizontal_scalings(scale_type,state,metadata,spec,status,object_meta,type_meta)
                                        VALUES (scale_type,state,metadata,spec,status,object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION get_horizontal_scaling_v1(hid bigint) RETURNS SETOF horizontal_scalings AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM horizontal_scalings WHERE id=hid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION get_hs_v1() RETURNS SETOF horizontal_scalings AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM horizontal_scalings;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION set_hs_status_v1 (hid bigint, hs_status jsonb) RETURNS SETOF horizontal_scalings AS $$
                            BEGIN
                                RETURN QUERY UPDATE horizontal_scalings SET status=hs_status, updated_at=now() WHERE id=hid
                            RETURNING *;
                            RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION update_hs_v1 (
                hid bigint,
                hs_scale_type text,
                hs_state text,
                hs_metadata jsonb,
                hs_spec jsonb,
                hs_status jsonb,
                hs_object_meta jsonb) RETURNS SETOF horizontal_scalings AS $$
                            BEGIN
                                RETURN QUERY UPDATE horizontal_scalings SET   scale_type=hs_scale_type, state=hs_state,metadata=hs_metadata,spec=hs_spec, status=hs_status,object_meta =hs_object_meta,updated_at=now() WHERE id=hid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS vs_id_seq;"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE TABLE  IF NOT EXISTS vertical_scalings (
             id bigint PRIMARY KEY DEFAULT next_id_v1('vs_id_seq'),
             scale_type text,
             state text,
             update_policy jsonb,
             metadata jsonb,
             spec jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] vertical_scalings");

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION insert_vs_v1 (
                scale_type text,
                state text,
                update_policy jsonb,
                metadata jsonb,
                spec jsonb,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
                        ) RETURNS SETOF vertical_scalings AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO vertical_scalings(scale_type,state,update_policy,metadata,spec,status,object_meta,type_meta)
                                        VALUES (scale_type,state,update_policy,metadata,spec,status,object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION get_vs_v1() RETURNS SETOF vertical_scalings AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM vertical_scalings;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION set_vs_status_v1 (vid bigint, hs_status jsonb) RETURNS SETOF vertical_scalings AS $$
                            BEGIN
                                RETURN QUERY UPDATE vertical_scalings SET status=hs_status, updated_at=now() WHERE id=vid
                            RETURNING *;
                            RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION get_vertical_scaling_v1(vid bigint) RETURNS SETOF vertical_scalings AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM vertical_scalings WHERE id=vid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "scalesrv",
            r#"CREATE OR REPLACE FUNCTION update_vs_v1 (
                vid bigint,
                vs_scale_type text,
                vs_state text,
                vs_update_policy jsonb,
                vs_metadata jsonb,
                vs_spec jsonb,
                vs_status jsonb,
                vs_object_meta jsonb) RETURNS SETOF vertical_scalings AS $$
                            BEGIN
                                RETURN QUERY UPDATE vertical_scalings SET  scale_type=vs_scale_type, state=vs_state,update_policy=vs_update_policy,metadata=vs_metadata,spec=vs_spec, status=vs_status,object_meta =vs_object_meta,updated_at=now() WHERE id=vid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.end("ScaleProcedure");

        Ok(())
    }
}
