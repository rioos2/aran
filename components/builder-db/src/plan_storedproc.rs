// Copyright (c) 2017 RioCorp Inc.

//stored procedures for plan_factory

use error::Result;
use migration::{Migratable, Migrator};

pub struct PlanProcedures;

impl PlanProcedures {
    pub fn new() -> Result<PlanProcedures> {
        Ok(PlanProcedures)
    }
}

impl Migratable for PlanProcedures {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()> {
        debug!("=> START: plansrv");

        // The core plans table
        migrator.migrate(
            "plansrv",
            r#"CREATE SEQUENCE IF NOT EXISTS plan_id_seq;"#,
        )?;

        debug!("=> [✓] plan_id_seq");

        migrator.migrate(
            "plansrv",
            r#"CREATE TABLE  IF NOT EXISTS plan_factory (
             id bigint PRIMARY KEY DEFAULT next_id_v1('plan_id_seq'),
             name text,
             url text ,
             description text,
             tags text[],
             camp_version text,
             origin text,
             artifacts text[],
             services text[],
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;
        debug!("=> [✓] plan_factory");

        migrator.migrate(
            "plansrv",
            r#"INSERT INTO plan_factory(name,url,description,tags,camp_version,origin,artifacts,services)VALUES ('1_virtualmachine_ubuntu','/v3/plan/ubuntu','Ubuntu is a Debian-based Linux operating system.','{"linux", "ubuntu", "xenial", "14.04"}','1.2', 'vertice:2.0','{}',
            '{"{\"name\":\"Trusty\", \"description\":\"Ubuntu is a Debian-based Linux operating system.Trusty Tahr is the Ubuntu codename for version 14.04 LTS of the Ubuntu Linux-based operating system.\",\"href\":\"https://www.ubuntu.com\",\"characteristics\":{\"type\":\"org.megam.vm::provided_by\",\"14.04\":\"vertice\"}}"}')"#,
        )?;
        debug!("=> [✓] plan_factory_ubuntu");



        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION get_plan_v1 (plan_url text) RETURNS SETOF plan_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM plan_factory WHERE url = plan_url;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_plan_v1");

        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION get_plans_v1() RETURNS SETOF plan_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM plan_factory;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_plans_v1");

        // The core plans table
        debug!("=> DONE: plansrv");

        Ok(())
    }
}
