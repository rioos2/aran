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
            "storagesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS storage_id_seq;"#,
        )?;

        migrator.migrate(
            "storagesrv",
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
            "storagesrv",
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
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_storages_v1() RETURNS SETOF storages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_storages_v1");


        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_storage_v1 (sid bigint) RETURNS SETOF storages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages WHERE id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_storages_v1");

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION set_storage_status_v1 (sid bigint, storage_status text) RETURNS SETOF storages AS $$
                            BEGIN
                                RETURN QUERY UPDATE storages SET status=storage_status, updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.para("[✓] set_storage_status_v1");

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION update_storage_v1(
            sid bigint,
            s_object_meta text,
            s_type_meta text,
            s_name text,
            s_host_ip text,
            s_storage_type text,
            s_parameters text) RETURNS SETOF storages AS $$
                            BEGIN
                                RETURN QUERY UPDATE storages SET object_meta=s_object_meta,type_meta=s_type_meta,name=s_name,host_ip=s_host_ip,storage_type=s_storage_type,parameters=s_parameters,updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.para("[✓] update_storage_v1");


        migrator.migrate(
            "storagesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS dc_id_seq;"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE TABLE  IF NOT EXISTS data_center (
             id bigint PRIMARY KEY DEFAULT next_id_v1('dc_id_seq'),
             object_meta text,
             type_meta text,
             name text,
             nodes text[],
             networks text[],
             enabled bool,
             storage text,
             advanced_settings text,
             flag text,
             currency text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] data_center");

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION insert_dc_v1 (
                object_meta text,
                type_meta text,
                name text,
                nodes text[],
                networks text[],
                enabled bool,
                storage text,
                advanced_settings text,
                flag text,
                currency text,
                status text
            ) RETURNS SETOF data_center AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO data_center(object_meta,type_meta,name,nodes,networks,enabled,storage,advanced_settings,flag,currency,status)
                                        VALUES (object_meta,type_meta,name,nodes,networks,enabled,storage,advanced_settings,flag,currency,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;


        ui.para("[✓] insert_dc_v1");

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_data_centers_v1() RETURNS SETOF data_center AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM data_center;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_data_centers_v1");

        ui.end("StorageProcedures");

        Ok(())
    }
}
