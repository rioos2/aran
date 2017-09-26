// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct NetworkProcedures;

impl NetworkProcedures {
    pub fn new() -> Result<NetworkProcedures> {
        Ok(NetworkProcedures)
    }
}

impl Migratable for NetworkProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("NetworkProcedures");

        migrator.migrate(
            "netsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS net_id_seq;"#,
        )?;

        migrator.migrate(
            "nodesrv",
            r#"CREATE TABLE  IF NOT EXISTS networks (
             id bigint PRIMARY KEY DEFAULT next_id_v1('net_id_seq'),
             object_meta text,
             type_meta text,
             name text,
             host_ip text,
             storage_type text,
             parameters text,
             status text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] network");


        // Insert a new job into the jobs table
        migrator.migrate(
            "nodesrv",
            r#"CREATE OR REPLACE FUNCTION insert_network_v1 (
                object_meta text,
                type_meta text,
                name text,
                host_ip text,
                storage_type text,
                parameters text,
                status text
            ) RETURNS SETOF networks AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO networks(object_meta, type_meta,name,host_ip,storage_type,parameters,status)
                                        VALUES (object_meta, type_meta,name,host_ip,storage_type,parameters,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;


        ui.para("[✓] insert_network_v1");

        ui.end("NetworkProcedures");

        Ok(())
    }
}
