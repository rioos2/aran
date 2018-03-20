// Copyright 2018 The Rio Advancement Inc

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct NodeProcedures;

impl NodeProcedures {
    pub fn new() -> Result<NodeProcedures> {
        Ok(NodeProcedures)
    }
}

impl Migratable for NodeProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("Nodeprocedure");

        migrator.migrate(
            "nodesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS node_id_seq;"#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE TABLE  IF NOT EXISTS nodes (
             id bigint PRIMARY KEY DEFAULT next_id_v1('node_id_seq'),
             node_ip text,
             spec jsonb,
             status jsonb,
             object_meta jsonb,
             type_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
            )"#,
        )?;

        ui.para("[✓] nodes");

        // Insert a new job into the jobs table
        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION insert_node_v1 (
                node_ip text,
                spec jsonb,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
            ) RETURNS SETOF nodes AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO nodes(node_ip,spec,status,object_meta,type_meta)
                                        VALUES (node_ip,spec,status,object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION get_node_v1(nid bigint) RETURNS SETOF nodes AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM nodes where id = nid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;
        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION get_nodes_by_node_ip_v1(nodeip text) RETURNS SETOF nodes AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM nodes where node_ip = nodeip;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION get_nodes_v1() RETURNS SETOF nodes AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM nodes;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION set_node_status_v1 (nid bigint, node_status jsonb) RETURNS SETOF nodes AS $$
                            BEGIN
                                RETURN QUERY UPDATE nodes SET status=node_status, updated_at=now() WHERE id=nid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.end("NodeProcedure");

        Ok(())
    }
}
