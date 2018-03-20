// Copyright 2018 The Rio Advancement Inc

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
            r#"CREATE TABLE  IF NOT EXISTS assemblys (
             id bigint PRIMARY KEY DEFAULT next_id_v1('asm_id_seq'),
             type_meta jsonb,
             object_meta jsonb,
             selector text[],
             status jsonb,
             metadata jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] assemblys");

        // Insert a new job into the jobs table
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION insert_assembly_v1 (
                type_meta jsonb,
                object_meta jsonb,
                selector text[],
                status jsonb,
                metadata jsonb
                        ) RETURNS SETOF assemblys AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO assemblys(type_meta,object_meta,selector,status,metadata)
                                        VALUES (type_meta,object_meta,selector,status,metadata)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS plan_id_seq;"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE TABLE  IF NOT EXISTS plan_factory (
             id bigint PRIMARY KEY DEFAULT next_id_v1('plan_id_seq'),
             type_meta jsonb,
             object_meta jsonb,
             category text,
             version text,
             characteristics jsonb,
             icon text,
             description text,
             ports jsonb,
             envs jsonb,
             lifecycle jsonb,
             status jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;
        ui.para("[✓] plan_factory");

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assembly_v1 (aid bigint) RETURNS SETOF assemblys AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assemblys WHERE id = aid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assemblys_v1() RETURNS SETOF assemblys AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assemblys;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assemblys_by_parentid_v1 (pid text) RETURNS SETOF assemblys AS $$
                        BEGIN
                        RETURN QUERY SELECT * FROM assemblys  WHERE object_meta @> json_build_object('owner_references',json_build_array(json_build_object('uid',pid)))::jsonb;
                        RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION update_assembly_v1 (aid bigint,asm_selector text[],asm_status jsonb,asm_object_meta jsonb, asm_metadata jsonb) RETURNS SETOF assemblys AS $$
                            BEGIN
                                RETURN QUERY UPDATE assemblys SET selector=asm_selector,status=asm_status,object_meta = asm_object_meta,metadata=asm_metadata,updated_at=now() WHERE id=aid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION set_assembly_status_v1 (aid bigint, asm_status jsonb) RETURNS SETOF assemblys AS $$
                            BEGIN
                                RETURN QUERY UPDATE assemblys SET status=asm_status, updated_at=now() WHERE id=aid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assemblys_by_account_v1 (account_id text) RETURNS SETOF assemblys AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assemblys WHERE object_meta ->> 'account' = account_id;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
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
             object_meta jsonb,
             type_meta jsonb,
             replicas smallint,
             resources jsonb,
             metadata jsonb,
             status jsonb,
             secret jsonb,
             plan bigint REFERENCES plan_factory(id),
             spec jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        ui.para("[✓] assembly_factory");

        // Insert a new assembly_factory into the assembly_factory table
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION insert_assembly_factory_v1 (
                object_meta jsonb,
                type_meta jsonb,
                replicas smallint,
                resources jsonb,
                metadata jsonb,
                status jsonb,
                secret jsonb,
                plan bigint,
                spec jsonb
                        ) RETURNS SETOF assembly_factory AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO assembly_factory(object_meta,type_meta,replicas,resources,metadata,status,secret,plan,spec)
                                        VALUES (object_meta,type_meta,replicas,resources,metadata,status,secret,plan,spec)
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
            r#"CREATE OR REPLACE FUNCTION set_assembly_factorys_status_v1 (aid bigint, asm_fac_status jsonb)  RETURNS SETOF assembly_factory AS $$
                            BEGIN
                            RETURN QUERY UPDATE assembly_factory SET status=asm_fac_status, updated_at=now() WHERE id=aid
                            RETURNING *;
                            RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assembly_factory_by_account_v1 (account_id text) RETURNS SETOF assembly_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assembly_factory WHERE object_meta ->> 'account' = account_id;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.end("DeployProcedure");

        Ok(())
    }
}
