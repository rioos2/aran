// Copyright 2018 The Rio Advancement Inc

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct VolumeProcedures;

impl VolumeProcedures {
    pub fn new() -> Result<VolumeProcedures> {
        Ok(VolumeProcedures)
    }
}

impl Migratable for VolumeProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("Volumeprocedure");

        migrator.migrate(
            "volumesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS volume_id_seq;"#,
        )?;

        migrator.migrate(
            "volumesrv",
            r#"CREATE TABLE  IF NOT EXISTS volumes (
             id bigint PRIMARY KEY DEFAULT next_id_v1('volume_id_seq'),
             mount_path text,
             allocated text,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] volumes");

        // Insert a new job into the Volume table
        migrator.migrate(
            "volumesrv",
            r#"CREATE OR REPLACE FUNCTION insert_volume_v1 (
                mount_path text,
                allocated text,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
            ) RETURNS SETOF volumes AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO volumes(mount_path,allocated,status,object_meta, type_meta)
                                        VALUES (mount_path,allocated,status, object_meta, type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "volumesrv",
            r#"CREATE OR REPLACE FUNCTION get_volume_v1(vid bigint) RETURNS SETOF volumes AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM volumes where id = vid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "volumerv",
            r#"CREATE OR REPLACE FUNCTION set_volume_status_v1 (vid bigint, volume_status jsonb) RETURNS SETOF volumes AS $$
                            BEGIN
                                RETURN QUERY UPDATE volumes SET status=volume_status, updated_at=now() WHERE id=vid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "volumesrv",
            r#"CREATE OR REPLACE FUNCTION get_volumes_by_assembly_v1(aid text) RETURNS SETOF volumes AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM volumes where object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',aid)))::jsonb;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "volumesrv",
            r#"CREATE OR REPLACE FUNCTION update_volume_v1 (vid bigint,vol_mount_path text,vol_allocated text,vol_status jsonb,vol_object_meta jsonb) RETURNS SETOF volumes AS $$
                            BEGIN
                                RETURN QUERY UPDATE volumes SET mount_path=vol_mount_path,allocated=vol_allocated,status=vol_status,object_meta = vol_object_meta,updated_at=now() WHERE id=vid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.end("VolumeProcedure");

        Ok(())
    }
}
