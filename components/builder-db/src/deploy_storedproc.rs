// Copyright (c) 2017 RioCorp Inc.

//stored procedures for assemblys, assembly_factory, components

use error::{Result, Error};
use migration::{Migratable, Migrator};

pub struct DeployProcedures;

impl DeployProcedures {

    pub fn new() -> Result<DeployProcedures> {
        Ok(DeployProcedures)
    }
}

impl Migratable for DeployProcedures {

    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: asmsrv");

        // The core asms table
        migrator.migrate(
            "asmsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS asm_id_seq;"#,
        )?;

        debug!("=> [✓] asm_id_seq");

        migrator.migrate(
            "asmsrv",
            r#"CREATE TABLE  IF NOT EXISTS assembly (
         id bigint PRIMARY KEY DEFAULT next_id_v1('asm_id_seq'),
         uri text,
         name text,
         description text,
         tags text[],
         representation_skew text,
         external_management_resource text,
         component_collection text[],
         plan text,
         operation_collection text[],
         sensor_collection text[],
         metadata text,
         updated_at timestamptz,
         created_at timestamptz DEFAULT now())"#,
        )?;

        debug!("=> [✓] assembly");

        // Insert a new job into the jobs table
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION insert_assembly_v1 (
                        name text,
                        uri text,
                        description text,
                        tags text[],
                        external_management_resource text,
                        representation_skew text,
                        component_collection text[],
                        plan text,
                        operation_collection text[],
                        sensor_collection text[],
                        metadata text
                    ) RETURNS SETOF assembly AS $$
                            BEGIN
                                RETURN QUERY INSERT INTO assembly(name, uri, description, tags, external_management_resource, representation_skew, component_collection,plan,operation_collection,sensor_collection,metadata)
                                    VALUES (name,uri, description, tags, external_management_resource, representation_skew, component_collection,plan,operation_collection,sensor_collection,metadata)
                                    RETURNING *;
                                RETURN;
                            END
                        $$ LANGUAGE plpgsql VOLATILE
                        "#,
        )?;
        debug!("=> [✓] assembly");

        // Just make sure you always address the columns by name, not by position.
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assembly_v1 (aid bigint) RETURNS SETOF assembly AS $$
                    BEGIN
                      RETURN QUERY SELECT * FROM assembly WHERE id = aid;
                      RETURN;
                    END
                    $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_assembly_v1");
        debug!("=> DONE: asmsrv");

        Ok(())
    }
}
