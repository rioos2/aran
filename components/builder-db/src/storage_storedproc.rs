// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct StorageProcedures;

impl StorageProcedures {
    pub fn new() -> Result<StorageProcedures> {
        Ok(StorageProcedures)
    }
}

impl Migratable for StorageProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("StorageProcedures");

        migrator.migrate(
            "netsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS storage_id_seq;"#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE TABLE  IF NOT EXISTS storages (
             id bigint PRIMARY KEY DEFAULT next_id_v1('storage_id_seq'),
             object_meta text,
             type_meta text,
             name text,
             host_ip text,
             storage_type text,
             parameters text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] storage");


        // Insert a new job into the jobs table
        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION insert_storage_v1 (
                object_meta text,
                type_meta text,
                name text,
                host_ip text,
                storage_type text,
                parameters text,
                status text
            ) RETURNS SETOF storages AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO storages(object_meta, type_meta,name,host_ip,storage_type,parameters,status)
                                        VALUES (object_meta, type_meta,name,host_ip,storage_type,parameters,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;


        ui.para("[✓] insert_storage_v1");

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_storages_v1() RETURNS SETOF storages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_storages_v1");

        ui.end("StorageProcedures");

        Ok(())
    }
}
