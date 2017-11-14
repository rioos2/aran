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
        ui.begin("Storageprocedure");

        migrator.migrate(
            "storagesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS storage_id_seq;"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE TABLE  IF NOT EXISTS storages (
             id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('storage_id_seq'),
             name text,
             host_ip text,
             storage_type text,
             parameters text,
             storage_info text,
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
                name text,
                host_ip text,
                storage_type text,
                parameters text,
                storage_info text,
                status text
            ) RETURNS SETOF storages AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO storages(name,host_ip,storage_type,parameters,storage_info,status)
                                        VALUES (name,host_ip,storage_type,parameters,storage_info,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;


        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_storages_v1() RETURNS SETOF storages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_storages_by_ip_v1 (hostip text) RETURNS SETOF storages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages WHERE host_ip = hostip;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_storages_by_ip_v1");

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_storage_v1 (sid bigint) RETURNS SETOF storages AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages WHERE id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;



        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION set_storage_status_v1 (sid bigint, storage_status text) RETURNS SETOF storages AS $$
                            BEGIN
                                RETURN QUERY UPDATE storages SET status=storage_status, updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION update_storage_v1(
            sid bigint,
            s_name text,
            s_host_ip text,
            s_storage_type text,
            s_parameters text,
            s_storage_info text) RETURNS SETOF storages AS $$
                            BEGIN
                                RETURN QUERY UPDATE storages SET name=s_name,host_ip=s_host_ip,storage_type=s_storage_type,parameters=s_parameters,storage_info=s_storage_info,updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;


        migrator.migrate(
            "storagesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS dc_id_seq;"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE TABLE  IF NOT EXISTS data_center (
             id bigint PRIMARY KEY DEFAULT next_id_v1('dc_id_seq'),
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


        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION insert_dc_v1 (
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
                                    RETURN QUERY INSERT INTO data_center(name,nodes,networks,enabled,storage,advanced_settings,flag,currency,status)
                                        VALUES (name,nodes,networks,enabled,storage,advanced_settings,flag,currency,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;


        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_data_centers_v1() RETURNS SETOF data_center AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM data_center;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_data_center_v1(did bigint) RETURNS SETOF data_center AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM data_center WHERE id = did;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;


        migrator.migrate(
            "storagesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS storages_pool_id_seq;"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE TABLE  IF NOT EXISTS storages_pool (
             id bigint PRIMARY KEY DEFAULT next_id_v1('storages_pool_id_seq'),
             name text,
             connector_id bigint REFERENCES storages(id),
             parameters text,
             storage_info text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] storages_pool");


        // Insert a new job into the jobs table
        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION insert_storage_pool_v1 (
                name text,
                connector_id bigint,
                parameters text,
                storage_info text,
                status text
            ) RETURNS SETOF storages_pool AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO storages_pool(name,connector_id,parameters,storage_info,status)
                                        VALUES (name,connector_id,parameters,storage_info,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;


        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_storage_pool_v1 (sid bigint) RETURNS SETOF storages_pool AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages_pool WHERE connector_id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_storage_pool_all_v1() RETURNS SETOF storages_pool AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages_pool;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION set_storage_pool_status_v1 (sid bigint, sp_status text) RETURNS SETOF storages_pool AS $$
                            BEGIN
                                RETURN QUERY UPDATE storages_pool SET status=sp_status, updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;


        ui.end("StorageProcedure");

        Ok(())
    }
}
