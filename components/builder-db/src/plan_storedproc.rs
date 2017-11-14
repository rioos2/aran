// Copyright (c) 2017 RioCorp Inc.

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
            r#"CREATE SEQUENCE IF NOT EXISTS plan_id_seq;"#,
        )?;

        migrator.migrate(
            "plansrv",
            r#"CREATE TABLE  IF NOT EXISTS plan_factory (
             id bigint PRIMARY KEY DEFAULT next_id_v1('plan_id_seq'),
             group_name text,
             url text ,
             description text,
             tags text[],
             origin text,
             artifacts text[],
             services text[],
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;
        ui.para("[✓] plan_factory");

        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION insert_plan_factory_v1 (
                group_name text,
               url text ,
               description text,
               tags text[],
               origin text,
               artifacts text[],
                services text[]
                        ) RETURNS SETOF plan_factory AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO plan_factory(group_name, url, description,tags, origin, artifacts,services)
                                        VALUES (group_name, url, description,tags, origin, artifacts,services)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;



        migrator.migrate(
            "plansrv",
            r#"INSERT INTO plan_factory(group_name,url,description,tags,origin,artifacts,services)VALUES ('1_virtualmachine_ubuntu','/v3/plan/ubuntu','Ubuntu is a Debian-based Linux operating system','{"linux", "ubuntu", "xenial", "14.04"}','rioos:2.0','{}',
            '{"{\"name\":\"trusty\",\"description\":\"Ubuntu is a Debian-based Linux operating system. Trusty Tahr is the Ubuntu codename for version 14.04 LTS of the Ubuntu Linux-based operating system.\",\"href\":\"https://www.ubuntu.com\",\"characteristics\":{\"image\":\"ubuntu.png\",\"version\":\"14.04\"}}","{\"name\":\"Xenial\",\"description\":\"Ubuntu is a Debian-based Linux operating system. Trusty Tahr is the Ubuntu codename for version 16.04 LTS of the Ubuntu Linux-based operating system.\",\"href\":\"https://www.ubuntu.com\",\"characteristics\":{\"image\":\"ubuntu.png\",\"version\":\"16.04\"}}"}')"#,

        )?;

        ui.para("[✓] plan_factory_ubuntu");

        migrator.migrate(
            "plansrv",
            r#"INSERT INTO plan_factory(group_name,url,description,tags,origin,artifacts,services)VALUES ('1_virtualmachine_centos','/v3/plan/centos','centos operating system','{"centos"}','rioos:2.0','{}',
            '{"{\"name\":\"Centos\",\"description\":\"centos 7.4.\",\"href\":\"https://www.ubuntu.com\",\"characteristics\":{\"image\":\"centos.png\",\"version\":\"7.4\"}}"}')"#,

        )?;

        ui.para("[✓] plan_factory_centos");

        // migrator.migrate(
        //     "plansrv",
        //     r#"INSERT INTO plan_factory(group_name,url,description,tags,origin,artifacts,services)VALUES ('2_container_rioos','/v3/plan/rioos','tutum/hello-world is testing simple light weight docker container','{"tutum","hello-world"}','rioos:2.0','{}',
        //     '{"{\"name\":\"hello-world\",\"description\":\"tutum is a Debian-based simple container.\",\"href\":\https://www.tutum.com\",\"characteristics\":{\"os\":\"centos\",\"http.host.port\":\"8080\",\"http.container.port\":\"80\"}}"}')"#,
        //
        // )?;
        //
        // ui.para("[✓] plan_factory_container");

        migrator.migrate(
            "plansrv",
            r#"INSERT INTO plan_factory(group_name,url,description,tags,origin,artifacts,services)VALUES ('2_application_java','/v3/plan/java','The Apache Tomcat® software is an open source implementation of the Java Servlet, JavaServer Pages, Java Expression Language and Java WebSocket technologies.','{"tomcat","java","jdk"}', 'rioos:2.0','{}','{"{\"name\":\"tomcat\",\"description\":\"\",\"href\":\"http://tomcat.apache.org/\",\"characteristics\":{\"os\":\"centos\",\"http.port\":\"3000\",\"username\":\"megam\",\"password\":\"team4megam\",\"version\":\"4.2\",\"image\":\"java.png\"}}"}')"#,

        )?;

        ui.para("[✓] plan_factory_java");


        migrator.migrate(
            "plansrv",
            r#"INSERT INTO plan_factory(group_name,url,description,tags,origin,artifacts,services)VALUES ('2_application_rails','/v3/plan/rails','Rails is a web application framework written in Ruby.','{"rails", "ruby", "ror"}', 'rioos:2.0','{}','{"{\"name\":\"rails\",\"description\":\"\",\"href\":\"http://rubyonrails.org/\",\"characteristics\":{\"os\":\"centos\",\"http.port\":\"3000\",\"version\":\"4.2\",\"image\":\"rails.png\"}}"}')"#,

        )?;
        ui.para("[✓] plan_factory_rails");

        migrator.migrate(
            "plansrv",
            r#"CREATE OR REPLACE FUNCTION get_plan_v1 (plan_url text) RETURNS SETOF plan_factory AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM plan_factory WHERE url = plan_url;
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


        ui.end("PlanProcedure");

        Ok(())
    }
}
