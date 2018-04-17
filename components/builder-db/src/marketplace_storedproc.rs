// Copyright 2018 The Rio Advancement Inc

//stored procedures for marketplaces

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct MarketPlaceProcedures;

impl MarketPlaceProcedures {
    pub fn new() -> Result<MarketPlaceProcedures> {
        Ok(MarketPlaceProcedures)
    }
}

impl Migratable for MarketPlaceProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("MarketPlaceProcedures");

        // The core package table
        migrator.migrate(
            "marketplacesrv",
            r#"CREATE SEQUENCE IF NOT EXISTS market_id_seq;"#,
        )?;

        migrator.migrate(
            "marketplacesrv",
            r#"CREATE TABLE  IF NOT EXISTS marketplaces (
             id bigint PRIMARY KEY DEFAULT next_id_v1('market_id_seq'),
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
             metadata jsonb,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;
        ui.para("[✓] marketplaces");

        migrator.migrate(
            "marketplacesrv",
            r#"CREATE OR REPLACE FUNCTION insert_marketplace_v1 (
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
           metadata jsonb
                   ) RETURNS SETOF marketplaces AS $$
                           BEGIN
                               RETURN QUERY INSERT INTO marketplaces(type_meta, object_meta, category,version, characteristics, icon,description,ports,envs,lifecycle,status,metadata)
                                   VALUES (type_meta, object_meta, category,version, characteristics, icon,description,ports,envs,lifecycle,status,metadata)
                                   RETURNING *;
                               RETURN;
                           END
                       $$ LANGUAGE plpgsql VOLATILE
                       "#,
        )?;

        migrator.migrate(
            "marketplacesrv",
            r#"CREATE OR REPLACE FUNCTION get_marketplace_v1(mid bigint) RETURNS SETOF marketplaces AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM marketplaces WHERE id=mid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;
        migrator.migrate(
            "marketplacesrv",
            r#"CREATE OR REPLACE FUNCTION get_marketplaces_v1() RETURNS SETOF marketplaces AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM marketplaces;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        ui.end("MarketPlaceProcedures");

        Ok(())
    }
}
