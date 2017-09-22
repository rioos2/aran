// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};

pub struct ServiceAccountProcedure;

impl ServiceAccountProcedure {
    pub fn new() -> Result<ServiceAccountProcedure> {
        Ok(ServiceAccountProcedure)
    }
}

impl Migratable for ServiceAccountProcedure {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: servicesrv");
        // The core asms table
        migrator.migrate(
            "servicesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS sec_id_seq;"#,
        )?;

        debug!("=> [✓] sec_id_seq");

        migrator.migrate(
            "servicesrv",
            r#"CREATE TABLE  IF NOT EXISTS secret (
             id bigint PRIMARY KEY DEFAULT next_id_v1('sec_id_seq'),
             data text,
             object_meta text,
             type_meta text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        debug!("=> [✓] secret");


        // Insert a new job into the jobs table
        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION insert_secret_v1 (
                data text,
                object_meta text,
                type_meta text
            ) RETURNS SETOF secret AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO secret(data,object_meta,type_meta)
                                        VALUES (data,object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;
        debug!("=> [✓] fn: insert_secret_v1");

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_secret_v1 (sid bigint) RETURNS SETOF secret AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM secret WHERE id = sid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_secret_v1");


        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_secrets_v1() RETURNS SETOF secret AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM secret;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_secrets_v1");

        migrator.migrate(
            "servicesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS service_id_seq;"#,
        )?;

        debug!("=> [✓] ser_id_seq");

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

        debug!("=> [✓] service_account");

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
        debug!("=> [✓] fn: insert_service_account_v1");

        migrator.migrate(
            "servicesrv",
            r#"CREATE OR REPLACE FUNCTION get_service_account_v1() RETURNS SETOF service_account AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM service_account;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_service_account_v1");


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

        debug!("=> [✓] fn: get_service_account_by_origin_v1");


        debug!("=> DONE: servicesrv");

        Ok(())
    }
}
