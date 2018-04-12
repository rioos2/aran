// Copyright 2018 The Rio Advancement Inc

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
             host_ip text,
             storage_type text,
             parameters jsonb,
             storage_info jsonb,
             node_info jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] storage");

        // Insert a new job into the jobs table
        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION insert_storage_v1 (
                host_ip text,
                storage_type text,
                parameters jsonb,
                storage_info jsonb,
                node_info jsonb,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
            ) RETURNS SETOF storages AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO storages(host_ip,storage_type,parameters,storage_info,node_info,status,object_meta,type_meta)
                                        VALUES (host_ip,storage_type,parameters,storage_info,node_info,status,object_meta,type_meta)
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
            r#"CREATE OR REPLACE FUNCTION set_storage_status_v1 (sid bigint, storage_status jsonb) RETURNS SETOF storages AS $$
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
            s_host_ip text,
            s_storage_type text,
            s_parameters jsonb,
            s_storage_info jsonb,
            s_node_info jsonb,
            s_status jsonb,
            s_object_meta jsonb) RETURNS SETOF storages AS $$
                            BEGIN
                                RETURN QUERY UPDATE storages SET host_ip=s_host_ip,storage_type=s_storage_type,parameters=s_parameters,storage_info=s_storage_info,node_info=s_node_info,status = s_status,object_meta=s_object_meta,updated_at=now() WHERE id=sid
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
            r#"CREATE TABLE  IF NOT EXISTS data_centers (
             id bigint PRIMARY KEY DEFAULT next_id_v1('dc_id_seq'),
             nodes text[],
             networks text[],
             enabled bool,
             storage text,
             advanced_settings jsonb,
             flag text,
             currency text,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION insert_dc_v1 (
                nodes text[],
                networks text[],
                enabled bool,
                storage text,
                advanced_settings jsonb,
                flag text,
                currency text,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
            ) RETURNS SETOF data_centers AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO data_centers(nodes,networks,enabled,storage,advanced_settings,flag,currency,status,object_meta,type_meta)
                                        VALUES (nodes,networks,enabled,storage,advanced_settings,flag,currency,status,object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_data_centers_v1() RETURNS SETOF data_centers AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM data_centers;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION get_data_center_v1(did bigint) RETURNS SETOF data_centers AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM data_centers WHERE id = did;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION update_datacenter_by_v1(
            dc_id bigint,
            dc_nodes text[],
            dc_networks text[],
            dc_enabled bool,
            dc_storage text,
            dc_advanced_settings jsonb,
            dc_flag text,
            dc_currency text,
            dc_status jsonb,
            dc_object_meta jsonb) RETURNS SETOF data_centers AS $$
                            BEGIN
                                RETURN QUERY UPDATE data_centers SET nodes=dc_nodes,networks=dc_networks,enabled=dc_enabled,storage=dc_storage,advanced_settings= dc_advanced_settings,flag=dc_flag,currency=dc_currency,status=dc_status,object_meta=dc_object_meta,updated_at=now() WHERE id=dc_id
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS storages_pool_id_seq;"#,
        )?;

        migrator.migrate(
            "storagesrv",
            r#"CREATE TABLE  IF NOT EXISTS storages_pool (
             id bigint PRIMARY KEY DEFAULT next_id_v1('storages_pool_id_seq'),
             connector_id bigint REFERENCES storages(id),
             parameters jsonb,
             remote_storage_disks jsonb,
             storage_info jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] storages_pool");

        // Insert a new job into the jobs table
        migrator.migrate(
            "storagesrv",
            r#"CREATE OR REPLACE FUNCTION insert_storage_pool_v1 (
                connector_id bigint,
                parameters jsonb,
                remote_storage_disks jsonb,
                storage_info jsonb,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
            ) RETURNS SETOF storages_pool AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO storages_pool(connector_id,parameters,remote_storage_disks,storage_info,status, object_meta,type_meta)
                                        VALUES (connector_id,parameters,remote_storage_disks,storage_info,status,object_meta,type_meta)
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
            r#"CREATE OR REPLACE FUNCTION get_storage_pool_by_id_v1 (sid bigint) RETURNS SETOF storages_pool AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM storages_pool WHERE id = sid;
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
            r#"CREATE OR REPLACE FUNCTION set_storage_pool_status_v1 (sid bigint, sp_status jsonb) RETURNS SETOF storages_pool AS $$
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
