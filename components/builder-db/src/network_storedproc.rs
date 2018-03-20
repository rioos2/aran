// Copyright 2018 The Rio Advancement Inc

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
        ui.begin("Networkprocedure");

        migrator.migrate(
            "netsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS net_id_seq;"#,
        )?;

        migrator.migrate(
            "netsrv",
            r#"CREATE TABLE  IF NOT EXISTS networks (
             id bigint PRIMARY KEY DEFAULT next_id_v1('net_id_seq'),
             network_type text,
             subnet_ip text,
             netmask text,
             gateway text,
             used_bits smallint[],
             bridge_hosts jsonb,
             status jsonb,
             type_meta jsonb,
             object_meta jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now()
             )"#,
        )?;

        ui.para("[✓] networks");

        // Insert a new job into the jobs table
        migrator.migrate(
            "netsrv",
            r#"CREATE OR REPLACE FUNCTION insert_network_v1 (
                network_type text,
                subnet_ip text,
                netmask text,
                gateway text,
                used_bits smallint[],
                bridge_hosts jsonb,
                status jsonb,
                object_meta jsonb,
                type_meta jsonb
            ) RETURNS SETOF networks AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO networks(network_type,subnet_ip,netmask,gateway,used_bits,bridge_hosts,status,object_meta, type_meta )
                                        VALUES (network_type,subnet_ip,netmask,gateway,used_bits,bridge_hosts,status, object_meta,type_meta)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;

        migrator.migrate(
            "netsrv",
            r#"CREATE OR REPLACE FUNCTION get_networks_v1() RETURNS SETOF networks AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM networks;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;


        migrator.migrate(
            "netsrv",
            r#"CREATE OR REPLACE FUNCTION update_net_v1(
            nid bigint,
            n_network_type text,
            n_subnet_ip text,
            n_netmask text,
            n_gateway text,
            n_used_bits smallint[],
            n_bridge_hosts jsonb,
            n_status jsonb,
            n_object_meta jsonb) RETURNS SETOF networks AS $$
                            BEGIN
                                RETURN QUERY UPDATE networks SET network_type=n_network_type,subnet_ip=n_subnet_ip,netmask=n_netmask,gateway=n_gateway,used_bits=n_used_bits,bridge_hosts=n_bridge_hosts,status = n_status,object_meta=n_object_meta,updated_at=now() WHERE id=nid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;



        migrator.migrate(
            "netsrv",
            r#"CREATE OR REPLACE FUNCTION get_network_v1(nid bigint) RETURNS SETOF networks AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM networks where id = nid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.end("NetworkProcedures");

        Ok(())
    }
}
