// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};

pub struct SecretProcedures;

impl SecretProcedures {
    pub fn new() -> Result<SecretProcedures> {
        Ok(SecretProcedures)
    }
}

impl Migratable for SecretProcedures {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: secretsrv");
        // The core asms table
        migrator.migrate(
            "secretsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS sec_id_seq;"#,
        )?;

        debug!("=> [✓] sec_id_seq");

        migrator.migrate(
            "secretsrv",
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
            "secretsrv",
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

        debug!("=> DONE: secretsrv");

        Ok(())
    }
}
