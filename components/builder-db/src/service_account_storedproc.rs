// Copyright (c) 2017 RioCorp Inc.

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
             origin_id bigint REFERENCES origins(id),
             data text,
             object_meta text,
             type_meta text,
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
                origin_name text,
                data text,
                object_meta text,
                type_meta text
            ) RETURNS SETOF secrets AS $$
            DECLARE
               this_origin origins%rowtype;
            BEGIN
                SELECT * FROM origins WHERE origins.name = origin_name LIMIT 1 INTO this_origin;
                                    RETURN QUERY INSERT INTO secrets(secret_type,origin_id,data,object_meta,type_meta)
                                        VALUES (secret_type,this_origin.id,data,object_meta,type_meta)
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
            r#"CREATE OR REPLACE FUNCTION get_secrets_by_origin_v1(org_name text) RETURNS SETOF secrets AS $$
                DECLARE
                this_origin origins%rowtype;
                        BEGIN
                         SELECT * FROM origins WHERE origins.name = org_name LIMIT 1 INTO this_origin;
                         RETURN QUERY SELECT * FROM secrets WHERE origin_id=this_origin.id;
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
            r#"CREATE TABLE  IF NOT EXISTS service_account(
             id bigint PRIMARY KEY DEFAULT next_id_v1('service_id_seq'),
             origin_id bigint REFERENCES origins(id),
             name text,
             secrets text,
             object_meta text,
             type_meta text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] service_account");

        // Insert a new job into the jobs table
        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION insert_service_account_v1 (
                origin_name text,
                name text,
                secrets text,
                object_meta text,
                type_meta text
            ) RETURNS SETOF service_account AS $$
            DECLARE
               this_origin origins%rowtype;
            BEGIN
                SELECT * FROM origins WHERE origins.name = origin_name LIMIT 1 INTO this_origin;
                 RETURN QUERY INSERT INTO service_account(origin_id,name,secrets,object_meta,type_meta)
                     VALUES (this_origin.id,name,secrets,object_meta,type_meta)
                     RETURNING *;
                 RETURN;
            END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_service_account_v1() RETURNS SETOF service_account AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM service_account;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_service_account_by_origin_v1(ser_name text,org_name text) RETURNS SETOF service_account AS $$
                DECLARE
                this_origin origins%rowtype;
                        BEGIN
                         SELECT * FROM origins WHERE origins.name = org_name LIMIT 1 INTO this_origin;
                         RETURN QUERY SELECT * FROM service_account WHERE origin_id=this_origin.id AND name=ser_name;
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
             origin_id bigint REFERENCES origins(id),
             target_ref bigint REFERENCES assembly(id),
             subsets text,
             object_meta text,
             type_meta text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] endpoints");

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION insert_endpoints_v1 (
                target_ref bigint,
                origin_name text,
                subsets  text,
                object_meta text,
                type_meta text
                        ) RETURNS SETOF endpoints AS $$
                        DECLARE
                           this_origin origins%rowtype;
                                BEGIN
                                SELECT * FROM origins WHERE origins.name = origin_name LIMIT 1 INTO this_origin;
                                    RETURN QUERY INSERT INTO endpoints(origin_id,target_ref,subsets,object_meta,type_meta)
                                        VALUES (this_origin.id,target_ref,subsets,object_meta,type_meta )
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
            r#"CREATE OR REPLACE FUNCTION get_endpoints_by_origin_v1(org_name text) RETURNS SETOF endpoints AS $$
                DECLARE
                this_origin origins%rowtype;
                        BEGIN
                         SELECT * FROM origins WHERE origins.name = org_name LIMIT 1 INTO this_origin;
                         RETURN QUERY SELECT * FROM endpoints WHERE origin_id=this_origin.id;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;


        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_endpoints_by_assebmly_v1(target bigint) RETURNS SETOF endpoints AS $$
                        BEGIN
                         RETURN QUERY SELECT * FROM endpoints WHERE target_ref=target;
                         RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;




        ui.end("ServiceAccountProcedure");

        Ok(())
    }
}
