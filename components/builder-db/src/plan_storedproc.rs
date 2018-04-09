// Copyright 2018 The Rio Advancement Inc

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};
use common::ui::UI;

pub struct PlanProcedures;

impl PlanProcedures {
    pub fn new() -> Result<PlanProcedures> {
        Ok(PlanProcedures)
    }
}

impl Migratable for PlanProcedures {
    fn migrate(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        ui.begin("Planprocedure");

        // The core plans table

        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION insert_plan_factory_v1 (
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
           status jsonb
                   ) RETURNS SETOF plan_factory AS $$
                           BEGIN
                               RETURN QUERY INSERT INTO plan_factory(type_meta, object_meta, category,version, characteristics, icon,description,ports,envs,lifecycle,status)
                                   VALUES (type_meta, object_meta, category,version, characteristics, icon,description,ports,envs,lifecycle,status)
                                   RETURNING *;
                               RETURN;
                           END
                       $$ LANGUAGE plpgsql VOLATILE
                       "#,
        )?;


        migrator.migrate(
            "accountsrv",
            r#"CREATE OR REPLACE FUNCTION select_or_insert_plan_v1 (
                pname text,
                ptype_meta jsonb,
                pobject_meta jsonb,
                pcategory text,
                pversion text,
                pcharacteristics jsonb,
                picon text,
                pdescription text,
                pports jsonb,
                penvs jsonb,
                plifecycle jsonb,
                pstatus jsonb
                ) RETURNS SETOF plan_factory AS $$
                DECLARE
                 existing_plan plan_factory%rowtype;
                    BEGIN
                        SELECT  * INTO existing_plan FROM plan_factory WHERE object_meta ->> 'name' = pname;
                       IF FOUND THEN
                          RETURN QUERY UPDATE plan_factory SET type_meta=ptype_meta,object_meta=pobject_meta,category=pcategory,characteristics=pcharacteristics,icon=picon,status=pstatus,
                          description=pdescription,ports=pports,envs=penvs,lifecycle=plifecycle,updated_at=now() WHERE  object_meta ->> 'name' = pname RETURNING *;
                       ELSE
                       RETURN QUERY  INSERT INTO plan_factory(type_meta, object_meta, category,version, characteristics, icon,description,ports,envs,lifecycle,status)
                       VALUES (ptype_meta, pobject_meta, pcategory,pversion, pcharacteristics, picon,pdescription,pports,penvs,plifecycle,pstatus) ON CONFLICT DO NOTHING RETURNING *;
                       END IF;
                       RETURN;
                    END
                $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION get_plan_v1(pid bigint) RETURNS SETOF plan_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM plan_factory WHERE id=pid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;
        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION get_plans_v1() RETURNS SETOF plan_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM plan_factory;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION set_plan_status_v1 (pid bigint, plan_status jsonb) RETURNS SETOF plan_factory AS $$
                            BEGIN
                                RETURN QUERY UPDATE plan_factory SET status=plan_status, updated_at=now() WHERE id=pid
                                RETURNING *;
                                RETURN;
                            END
                         $$ LANGUAGE plpgsql VOLATILE"#,
        )?;

        ui.end("PlanProcedure");

        Ok(())
    }
}
