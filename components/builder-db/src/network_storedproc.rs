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
             name text,
             network_type text,
             subnet_ip text,
             netmask text,
             gateway text,
             bridge_hosts text,
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
                name text,
                network_type text,
                subnet_ip text,
                netmask text,
                gateway text,
                bridge_hosts text,
                status text
            ) RETURNS SETOF networks AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO networks(name,network_type,subnet_ip,netmask,gateway,bridge_hosts,status)
                                        VALUES (name,network_type,subnet_ip,netmask,gateway,bridge_hosts,status)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;


        ui.para("[✓] insert_network_v1");

        migrator.migrate(
            "networksrv",
            r#"CREATE OR REPLACE FUNCTION get_networks_v1() RETURNS SETOF networks AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM networks;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.para("[✓] get_networks_v1");

        ui.end("NetworkProcedures");

        Ok(())
    }
}
