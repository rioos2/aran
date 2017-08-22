// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};

pub struct NodeProcedures;

impl NodeProcedures {
    pub fn new() -> Result<NodeProcedures> {
        Ok(NodeProcedures)
    }
}

impl Migratable for NodeProcedures {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: nodesrv");
        // The core asms table
        migrator.migrate(
            "nodesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS node_id_seq;"#,
        )?;

        debug!("=> [✓] node_id_seq");

        migrator.migrate(
            "nodesrv",
            r#"CREATE TABLE  IF NOT EXISTS node (
             id bigint PRIMARY KEY DEFAULT next_id_v1('node_id_seq'),
             spec text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        debug!("=> [✓] node");


        // Insert a new job into the jobs table
        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION insert_node_v1 (
                spec text,
                status text
            ) RETURNS SETOF node AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO node(spec,status)
                                        VALUES (spec,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;
        debug!("=> [✓] fn: insert_node_v1");

        // The core plans table
        debug!("=> DONE: nodesrv");

        Ok(())
    }
}
