// Copyright 2018 The Rio Advancement Inc

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct DevtoolingProcedures;

impl DevtoolingProcedures {
    pub fn new() -> Result<DevtoolingProcedures> {
        Ok(DevtoolingProcedures)
    }
}

impl Migratable for DevtoolingProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("BuildConfigProcedures");

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS build_config_id_seq;"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE TABLE  IF NOT EXISTS build_configs (
             id bigint PRIMARY KEY DEFAULT next_id_v1('build_config_id_seq'),
             meta_data jsonb,
             spec jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] build_configs");

        // Insert a new buildconfigs into the build_configs table
        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION insert_build_config_v1 (
                meta_data jsonb,
                spec jsonb,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb

            ) RETURNS SETOF build_configs AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO build_configs(meta_data,spec,status,object_meta, type_meta)
                                        VALUES (meta_data,spec, status,object_meta, type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_build_configs_v1() RETURNS SETOF build_configs AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM build_configs;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_build_config_v1(bid bigint) RETURNS SETOF build_configs AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM build_configs where id = bid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;


        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_build_config_by_assembly_factory_v1(aid text) RETURNS SETOF build_configs AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM build_configs where object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',aid)))::jsonb;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION update_build_config_by_v1 (sid bigint, build_spec jsonb, build_status jsonb,build_meta_data jsonb,build_object_meta jsonb) RETURNS SETOF build_configs AS $$
                            BEGIN
                                RETURN QUERY UPDATE build_configs SET spec=build_spec, status=build_status,meta_data=build_meta_data,object_meta=build_object_meta,updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION set_build_configs_status_v1 (bid bigint, bc_status jsonb) RETURNS SETOF build_configs AS $$
                            BEGIN
                                RETURN QUERY UPDATE build_configs SET status=bc_status, updated_at=now() WHERE id=bid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS build_id_seq;"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE TABLE  IF NOT EXISTS builds (
             id bigint PRIMARY KEY DEFAULT next_id_v1('build_id_seq'),
             status jsonb,
             spec jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] builds");

        // Insert a new build into the builds table
        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION insert_build_v1 (
                status jsonb,
                spec jsonb,
                object_meta jsonb,
                type_meta jsonb

            ) RETURNS SETOF builds AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO builds(status,spec,object_meta, type_meta)
                                        VALUES (status,spec, object_meta, type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_builds_v1() RETURNS SETOF builds AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM builds;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_build_v1(bid bigint) RETURNS SETOF builds AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM builds where id = bid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;


        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_build_by_build_config_v1(bid text) RETURNS SETOF builds AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM builds where object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',bid)))::jsonb;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;
        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION update_build_by_v1 (sid bigint, build_spec jsonb,build_status jsonb,build_object_meta jsonb) RETURNS SETOF builds AS $$
                            BEGIN
                                RETURN QUERY UPDATE builds SET spec=build_spec,status=build_status,object_meta=build_object_meta,updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;
        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION update_build_status_by_v1 (bid bigint, build_status jsonb) RETURNS SETOF builds AS $$
                            BEGIN
                                RETURN QUERY UPDATE builds SET status=build_status, updated_at=now() WHERE id=bid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS imageref_id_seq;"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE TABLE  IF NOT EXISTS image_references (
             id bigint PRIMARY KEY DEFAULT next_id_v1('imageref_id_seq'),
             spec jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] image_references");

        // Insert a new image ref into the image reference table
        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION insert_image_ref_v1 (
                status jsonb,
                spec jsonb,
                object_meta jsonb,
                type_meta jsonb

            ) RETURNS SETOF image_references AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO image_references(spec,status,object_meta,type_meta)
                                        VALUES (spec,status,object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_image_ref_v1(iid bigint) RETURNS SETOF image_references AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM image_references where id = iid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_image_ref_by_v1() RETURNS SETOF image_references AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM image_references;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION update_image_ref_by_v1 (iid bigint, image_spec jsonb,image_status jsonb,image_object_meta jsonb) RETURNS SETOF image_references AS $$
                            BEGIN
                                RETURN QUERY UPDATE image_references SET spec=image_spec,status=image_status,object_meta=image_object_meta,updated_at=now() WHERE id=iid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;
        //
        migrator.migrate(
            "buildconfsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS image_marks_id_seq;"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE TABLE  IF NOT EXISTS image_marks (
                id bigint PRIMARY KEY DEFAULT next_id_v1('image_marks_id_seq'),
                tags jsonb,
                generation bigint,
                conditions jsonb,
                lookup_policy bool,
                image jsonb,
                object_meta jsonb,
                type_meta jsonb,
                updated_at timestamptz,
                created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] image_marks");

        // Insert a new image mark into the image_marks table
        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION insert_image_marks_v1 (
        tags jsonb,
        generation bigint,
        conditions jsonb,
        lookup_policy bool,
        image jsonb,
        object_meta jsonb,
        type_meta jsonb
    ) RETURNS SETOF image_marks AS $$
                        BEGIN
                            RETURN QUERY INSERT INTO image_marks(tags,generation,conditions,lookup_policy,image,object_meta,type_meta)
                                VALUES (tags,generation,conditions,lookup_policy,image,object_meta,type_meta)
                                RETURNING *;
                            RETURN;
                        END
                    $$ LANGUAGE plpgsql VOLATILE
                    "#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_image_marks_v1(iid bigint) RETURNS SETOF image_marks AS $$
                BEGIN
                  RETURN QUERY SELECT * FROM image_marks where id = iid;
                  RETURN;
                END
                $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_image_marks_v1() RETURNS SETOF image_marks AS $$
                BEGIN
                  RETURN QUERY SELECT * FROM image_marks;
                  RETURN;
                END
                $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION update_image_marks_by_v1 (iid bigint, image_tags jsonb,image_generation bigint,image_conditions jsonb, image_look_up_policy bool,image_data jsonb,image_object_meta jsonb) RETURNS SETOF image_marks AS $$
                    BEGIN
                        RETURN QUERY UPDATE image_marks SET tags=image_tags,generation=image_generation,conditions=image_conditions,lookup_policy=image_look_up_policy,image=image_data, object_meta=image_object_meta,updated_at=now() WHERE id=iid
                        RETURNING *;
                        RETURN;
                    END
                 $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "buildconfsrv",
            r#"CREATE OR REPLACE FUNCTION get_image_marks_by_build_v1(aid text) RETURNS SETOF image_marks AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM image_marks where object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',aid)))::jsonb;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.end("BuildConfigProcedures");

        Ok(())
    }
}
