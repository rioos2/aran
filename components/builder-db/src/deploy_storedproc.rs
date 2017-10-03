// Copyright (c) 2017 RioCorp Inc.

//stored procedures for assemblys, assembly_factory, components

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct DeployProcedures;

impl DeployProcedures {
    pub fn new() -> Result<DeployProcedures> {
        Ok(DeployProcedures)
    }
}

impl Migratable for DeployProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("Deployprocedure");

        // The core asms table
        migrator.migrate(
            "asmsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS asm_id_seq;"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE TABLE  IF NOT EXISTS assembly (
             id bigint PRIMARY KEY DEFAULT next_id_v1('asm_id_seq'),
             name text,
             uri text,
             description text,
             parent_id text,
             tags text[],
             node text,
             ip text,
             urls text,
             status text,
             volumes text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] assembly");


        // Insert a new job into the jobs table
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION insert_assembly_v1 (
                name text,
                uri text,
                description text,
                parent_id text,
                tags text[],
                node text,
                ip text,
                urls text,
                status text,
                volumes text
                        ) RETURNS SETOF assembly AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO assembly(name, uri, description,parent_id, tags,node,ip,urls,status,volumes)
                                        VALUES (name, uri, description,parent_id, tags,node,ip,urls,status,volumes)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assembly_v1 (aid bigint) RETURNS SETOF assembly AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assembly WHERE id = aid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assemblys_v1() RETURNS SETOF assembly AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assembly;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION update_assembly_v1 (aid bigint, asm_name text, asm_uri text, asm_description text,asm_parent_id text, asm_tags text[],asm_node text,asm_ip text,asm_urls text,asm_volumes text) RETURNS SETOF assembly AS $$
                            BEGIN
                                RETURN QUERY UPDATE assembly SET name=asm_name,uri=asm_uri,description=asm_description,parent_id=asm_parent_id,tags=asm_tags,node=asm_node,ip=asm_ip,urls=asm_urls, volumes= asm_volumes,updated_at=now() WHERE id=aid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION set_assembly_status_v1 (aid bigint, asm_status text) RETURNS SETOF assembly AS $$
                            BEGIN
                                RETURN QUERY UPDATE assembly SET status=asm_status, updated_at=now() WHERE id=aid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        // The core asms_facttory table
        migrator.migrate(
            "asmsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS asm_fact_id_seq;"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE TABLE IF NOT EXISTS assembly_factory (
             id bigint PRIMARY KEY DEFAULT next_id_v1('asm_fact_id_seq'),
             name text,
             uri text,
             description text,
             tags text[],
             plan text,
             properties text,
             external_management_resource text[],
             component_collection text,
             opssettings text,
             replicas bigint,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] assembly_factory");

        // Insert a new assembly_factory into the assembly_factory table
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION insert_assembly_factory_v1 (
                name text,
                uri text,
                description text,
                tags text[],
                plan text,
                properties text,
                external_management_resource text[],
                component_collection text,
                opssettings text,
                replicas bigint,
                status text
                        ) RETURNS SETOF assembly_factory AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO assembly_factory(name, uri, description, tags, plan,properties,external_management_resource,component_collection,opssettings,replicas,status)
                                        VALUES (name, uri, description, tags,plan,properties,external_management_resource,component_collection,opssettings,replicas,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        // Just make sure you always address the columns by name, not by position.
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assembly_factory_v1 (aid bigint) RETURNS SETOF assembly_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assembly_factory WHERE id = aid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assemblys_factory_v1() RETURNS SETOF assembly_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assembly_factory;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION set_assembly_factorys_status_v1 (aid bigint, asm_fac_status text)  RETURNS SETOF assembly_factory AS $$
                            BEGIN
                            RETURN QUERY UPDATE assembly_factory SET status=asm_fac_status, updated_at=now() WHERE id=aid
                            RETURNING *;
                            RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.end("DeployProcedure");

        Ok(())
    }
}
