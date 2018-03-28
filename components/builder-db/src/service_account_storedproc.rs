// Copyright 2018 The Rio Advancement Inc

//stored procedures service account
use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct ServiceAccountProcedure;

impl ServiceAccountProcedure {
    pub fn new() -> Result<ServiceAccountProcedure> {
        Ok(ServiceAccountProcedure)
    }
}

impl Migratable for ServiceAccountProcedure {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("ServiceAccountProcedure");
        // The core asms table
        migrator.migrate(
            "servicesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS sec_id_seq;"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE TABLE  IF NOT EXISTS secrets (
             id bigint PRIMARY KEY DEFAULT next_id_v1('sec_id_seq'),
             secret_type text,
             data jsonb,
             metadata jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] secret");

        // Insert a new job into the jobs table
        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION insert_secret_v1 (
                secret_type text,
                data jsonb,
                metadata jsonb,
                object_meta jsonb,
                type_meta jsonb
            ) RETURNS SETOF secrets AS $$
            BEGIN
                                    RETURN QUERY INSERT INTO secrets(secret_type,data,metadata,object_meta,type_meta)
                                        VALUES (secret_type,data,metadata,object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;
        debug!("=> [✓] fn: insert_secret_v1");

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_secret_v1 (sid bigint) RETURNS SETOF secrets AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM secrets WHERE id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_secrets_v1() RETURNS SETOF secrets AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM secrets;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_secrets_by_origin_v1 (origin text, name text) RETURNS SETOF secrets AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM secrets WHERE object_meta ->> 'name' = name AND metadata ->> 'origin' = origin ;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_secrets_by_account_v1(obj_id text) RETURNS SETOF secrets AS $$
                        BEGIN
                         RETURN QUERY SELECT * FROM secrets WHERE object_meta ->> 'account'=obj_id;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_secrets_by_origin_v1(obj_id text) RETURNS SETOF secrets AS $$
                        BEGIN
                         RETURN QUERY SELECT * FROM secrets WHERE metadata ->> 'origin'=obj_id;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS service_id_seq;"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE TABLE  IF NOT EXISTS service_accounts(
             id bigint PRIMARY KEY DEFAULT next_id_v1('service_id_seq'),
             secrets jsonb,
             object_meta jsonb,
             type_meta jsonb,
             metadata jsonb,
             roles text[],
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] service_accounts");

        // Insert a new service account into the service accounts table
        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION insert_service_account_v1 (
                secrets jsonb,
                object_meta jsonb,
                type_meta jsonb,
                metadata jsonb,
                roles text[]
            ) RETURNS SETOF service_accounts AS $$
            BEGIN
                 RETURN QUERY INSERT INTO service_accounts(secrets,object_meta,type_meta,metadata,roles)
                     VALUES (secrets,object_meta,type_meta,metadata, roles)
                     RETURNING *;
                 RETURN;
            END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION update_service_account_v1 (aid bigint,sa_secrets jsonb,asm_object_meta jsonb) RETURNS SETOF service_accounts AS $$
                            BEGIN
                                RETURN QUERY UPDATE service_accounts SET secrets=sa_secrets,object_meta = asm_object_meta,updated_at=now() WHERE id=aid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_service_account_v1() RETURNS SETOF service_accounts AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM service_accounts;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_service_account_by_id_v1 (sid bigint) RETURNS SETOF service_accounts AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM service_accounts WHERE id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_serviceaccount_by_originid_v1(ser_name text,acc_id text) RETURNS SETOF service_accounts AS $$
                        BEGIN
                         RETURN QUERY SELECT * FROM service_accounts WHERE object_meta ->> 'name'=ser_name;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_permission_by_service_account_v1 (serv_name text) RETURNS SETOF permissions AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM permissions WHERE role_id IN(SELECT id FROM roles WHERE name = ANY((SELECT roles FROM service_accounts WHERE object_meta ->> 'name'=serv_name)::text[]));
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;
        migrator.migrate(
            "servicesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS end_id_seq;"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE TABLE  IF NOT EXISTS endpoints (
             id bigint PRIMARY KEY DEFAULT next_id_v1('end_id_seq'),
             subsets jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] endpoints");

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION insert_endpoints_v1 (
                subsets  jsonb,
                object_meta jsonb,
                type_meta jsonb
                        ) RETURNS SETOF endpoints AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO endpoints(subsets,object_meta,type_meta)
                                        VALUES (subsets,object_meta,type_meta )
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_endpoints_v1() RETURNS SETOF endpoints AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM endpoints;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;
        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_endpoint_v1 (eid bigint) RETURNS SETOF endpoints AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM endpoints WHERE id = eid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_endpoints_by_account_v1(account_id text) RETURNS SETOF endpoints AS $$
                        BEGIN
                         RETURN QUERY SELECT * FROM endpoints WHERE object_meta ->> 'account'=account_id;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_endpoints_by_assebmly_v1(pid text) RETURNS SETOF endpoints AS $$
                        BEGIN
                         RETURN QUERY SELECT * FROM endpoints  WHERE object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',pid)))::jsonb;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS serv_id_seq;"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE TABLE  IF NOT EXISTS services (
             id bigint PRIMARY KEY DEFAULT next_id_v1('serv_id_seq'),
             spec jsonb,
             metadata jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;
        ui.para("[✓] services");

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION insert_services_v1 (
                spec  jsonb,
                metadata  jsonb,
                status  jsonb,
                object_meta jsonb,
                type_meta jsonb
                        ) RETURNS SETOF services AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO services(spec,metadata,status,object_meta,type_meta)
                                        VALUES (spec,metadata,status,object_meta,type_meta )
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_services_v1 (sid bigint) RETURNS SETOF services AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM services WHERE id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_services_list_v1() RETURNS SETOF services AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM services;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        //use this if  api for get service by asm_fac
        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_services_by_assembly_factory_v1(pid text) RETURNS SETOF services AS $$
                        BEGIN
                         RETURN QUERY SELECT * FROM services WHERE object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',pid)))::jsonb;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION update_servive_by_v1 (sid bigint, spec_data jsonb,serv_metadata jsonb,status_data jsonb,object_meta_data jsonb) RETURNS SETOF services AS $$
                            BEGIN
                                RETURN QUERY UPDATE services SET spec=spec_data,metadata=serv_metadata,status=status_data,object_meta=object_meta_data,updated_at=now() WHERE id=sid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS set_map_id_seq;"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE TABLE  IF NOT EXISTS settings_map (
         id bigint UNIQUE PRIMARY KEY DEFAULT next_id_v1('set_map_id_seq'),
         data jsonb,
         metadata jsonb,
         object_meta jsonb,
         type_meta jsonb,
         updated_at timestamptz,
         created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] settings_map");

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION insert_settings_map_v1 (
                metadata  jsonb,
                data  jsonb,
                object_meta jsonb,
                type_meta jsonb
                        ) RETURNS SETOF settings_map AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO settings_map(metadata,data,object_meta,type_meta)
                                        VALUES (metadata,data,object_meta,type_meta )
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_settings_map_v1 (origin text, name text) RETURNS SETOF settings_map AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM settings_map WHERE object_meta ->> 'name' = name AND metadata ->> 'origin' = origin ;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_settings_maps_v1() RETURNS SETOF settings_map AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM settings_map;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.end("ServiceAccountProcedure");

        Ok(())
    }
}
