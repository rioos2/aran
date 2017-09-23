// Copyright (c) 2017 RioCorp Inc.

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
        ui.begin("NodeProcedure");

        migrator.migrate(
            "nodesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS node_id_seq;"#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE TABLE  IF NOT EXISTS node (
             id bigint PRIMARY KEY DEFAULT next_id_v1('node_id_seq'),
             spec text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now(),
             object_meta text,
             type_meta text)"#,
        )?;

        ui.para("[✓] node");


        // Insert a new job into the jobs table
        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION insert_node_v1 (
                spec text,
                status text,
                object_meta text,
                type_meta text
            ) RETURNS SETOF node AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO node(spec,status,object_meta, type_meta)
                                        VALUES (spec,status,object_meta, type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION get_nodes_v1() RETURNS SETOF node AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM node;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION set_node_status_v1 (nid bigint, node_status text) RETURNS void AS $$
                            BEGIN
                                UPDATE node SET status=node_status, updated_at=now() WHERE id=nid;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.end("Nodeprocedure");

        Ok(())
    }
}
