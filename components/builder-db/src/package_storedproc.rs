// Copyright 2018 The Rio Advancement Inc

//stored procedures for packages
#![allow(unused_must_use)]
use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct PackageProcedures;

impl PackageProcedures {
    pub fn new() -> Result<PackageProcedures> {
        Ok(PackageProcedures)
    }
}

impl Migratable for PackageProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("PackageProcedures");

        // The core package table
        migrator.migrate(
            "packagesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS package_id_seq;"#,
        )?;

        migrator.migrate(
            "packagesrv",
            r#"CREATE TABLE  IF NOT EXISTS packages (
             id bigint PRIMARY KEY DEFAULT next_id_v1('package_id_seq'),
             version_number text,
             extension text,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] package");

        migrator.migrate(
            "packagesrv",
            r#"CREATE OR REPLACE FUNCTION insert_package_v1 (
           type_meta jsonb,
           object_meta jsonb,
           version_number text,
           extension text
       ) RETURNS SETOF packages AS $$
                           BEGIN
                               RETURN QUERY INSERT INTO packages(type_meta, object_meta, version_number,extension)
                                   VALUES (type_meta, object_meta, version_number,extension)
                                   RETURNING *;
                               RETURN;
                           END
                       $$ LANGUAGE plpgsql VOLATILE
                       "#,
        )?;

        migrator.migrate(
            "marketplacesrv",
            r#"CREATE OR REPLACE FUNCTION get_package_v1(pid bigint) RETURNS SETOF packages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM packages WHERE id=pid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.end("PackageProcedure");

        Ok(())
    }
}
