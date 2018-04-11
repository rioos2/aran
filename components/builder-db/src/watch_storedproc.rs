// Copyright 2018 The Rio Advancement Inc

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct WatchProcedures;

impl WatchProcedures {
    pub fn new() -> Result<WatchProcedures> {
        Ok(WatchProcedures)
    }
}

impl Migratable for WatchProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        debug!("=> START: worksrv");
        // The core asms table

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION assembly_factory_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('assemblyfactorys_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER assembly_factory_notify_insert AFTER INSERT OR UPDATE OR DELETE ON assembly_factory FOR EACH ROW EXECUTE PROCEDURE assembly_factory_update_notify();"#,
        )?;

        ui.para("[✓] assemblyfactory watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION assembly_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('assemblys_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER assembly_notify_insert AFTER INSERT OR UPDATE OR DELETE ON assemblys FOR EACH ROW EXECUTE PROCEDURE assembly_update_notify();"#,
        )?;

        ui.para("[✓] assemblys watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION node_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('nodes_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER node_notify_insert AFTER INSERT OR UPDATE OR DELETE ON nodes FOR EACH ROW EXECUTE PROCEDURE node_update_notify();"#,
        )?;

        ui.para("[✓] nodes watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION services_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('services_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER services_notify_insert AFTER INSERT OR UPDATE OR DELETE ON services FOR EACH ROW EXECUTE PROCEDURE services_update_notify();"#,
        )?;

        ui.para("[✓] services watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION jobs_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('jobs_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER jobs_notify_insert AFTER INSERT OR UPDATE OR DELETE ON jobs FOR EACH ROW EXECUTE PROCEDURE jobs_update_notify();"#,
        )?;

        ui.para("[✓] jobs watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION secrets_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('secrets_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER secrets_notify_insert AFTER INSERT OR UPDATE OR DELETE ON secrets FOR EACH ROW EXECUTE PROCEDURE secrets_update_notify();"#,
        )?;

        ui.para("[✓] secrets watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION hs_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('horizontalscaling_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER hs_notify_insert AFTER INSERT OR UPDATE OR DELETE ON horizontal_scalings FOR EACH ROW EXECUTE PROCEDURE hs_update_notify();"#,
        )?;

        ui.para("[✓] Horizonl scaling watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION storagespool_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('storagespool_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER storagespool_notify_insert AFTER INSERT OR UPDATE OR DELETE ON storages_pool FOR EACH ROW EXECUTE PROCEDURE storagespool_update_notify();"#,
        )?;

        ui.para("[✓] Storagespool watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION storageconnectors_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('storageconnectors_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER storageconnectors_notify_insert AFTER INSERT OR UPDATE OR DELETE ON storages FOR EACH ROW EXECUTE PROCEDURE storageconnectors_update_notify();"#,
        )?;

        ui.para("[✓] Storageconnectors watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION datacenters_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('datacenters_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER datacenters_notify_insert AFTER INSERT OR UPDATE OR DELETE ON data_centers FOR EACH ROW EXECUTE PROCEDURE datacenters_update_notify();"#,
        )?;

        ui.para("[✓] Datacenters watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION verticalscaling_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('verticalscaling_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER verticalscaling_notify_insert AFTER INSERT OR UPDATE OR DELETE ON vertical_scalings FOR EACH ROW EXECUTE PROCEDURE verticalscaling_update_notify();"#,
        )?;

        ui.para("[✓] Verticalscaling watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION settingsmap_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('settingsmap_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER settingsmap_notify_insert AFTER INSERT OR UPDATE OR DELETE ON settings_map FOR EACH ROW EXECUTE PROCEDURE settingsmap_update_notify();"#,
        )?;

        ui.para("[✓] Settingsmap watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION endpoints_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('endpoints_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER endpoints_notify_insert AFTER INSERT OR UPDATE OR DELETE ON endpoints FOR EACH ROW EXECUTE PROCEDURE endpoints_update_notify();"#,
        )?;

        ui.para("[✓] Endpoints watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION origins_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('origins_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER origins_notify_insert AFTER INSERT OR UPDATE OR DELETE ON origins FOR EACH ROW EXECUTE PROCEDURE origins_update_notify();"#,
        )?;

        ui.para("[✓] Origins watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION plans_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('plans_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER plans_notify_insert AFTER INSERT OR UPDATE OR DELETE ON plan_factory FOR EACH ROW EXECUTE PROCEDURE plans_update_notify();"#,
        )?;

        ui.para("[✓] plans watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION networks_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('networks_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER networks_notify_insert AFTER INSERT OR UPDATE OR DELETE ON networks FOR EACH ROW EXECUTE PROCEDURE networks_update_notify();"#,
        )?;

        ui.para("[✓] Networks watch started");

        migrator.migrate(
            "worksrv",
            r#"CREATE OR REPLACE FUNCTION serviceaccounts_update_notify() RETURNS trigger AS $$
                DECLARE
                    id bigint;
                BEGIN
                    IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
                        id = NEW.id;
                    ELSE
                        id = OLD.id;
                    END IF;
                    PERFORM pg_notify('serviceaccounts_trigger', json_build_object('table', TG_TABLE_NAME, 'data', id, 'type', TG_OP)::text);
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql"#,
        )?;

        migrator.migrate(
            "worksrv",
            r#"CREATE TRIGGER serviceaccounts_notify_insert AFTER INSERT OR UPDATE OR DELETE ON service_accounts FOR EACH ROW EXECUTE PROCEDURE serviceaccounts_update_notify();"#,
        )?;

        ui.para("[✓] Service Accounts watch started");

        debug!("=> [✓] trigger");
        debug!("=> DONE: worksrv");

        Ok(())
    }
}
