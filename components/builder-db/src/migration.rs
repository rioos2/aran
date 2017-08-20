// Copyright (c) 2017 RioCorp Inc.


use postgres;
use protocol::ShardId;

use rcore::crypto::hash::hash_string;
use error::{Error, Result};

// Because Van Halen is awesome, and I love Sammy Hagar.
static SETUP_LOCK: i64 = 5150;

#[derive(Debug)]
pub struct Migrator<'a> {
    xact: postgres::transaction::Transaction<'a>,
    shards: Vec<ShardId>,
}


/// Provides an interface for migrating the implementing type for `Migratables`.
pub trait Migratable {
    fn migrate(&self, migrator: &mut Migrator) -> Result<()>;
}


impl<'a> Migrator<'a> {
    pub fn new(xact: postgres::transaction::Transaction<'a>, shards: Vec<ShardId>) -> Migrator {
        Migrator {
            xact: xact,
            shards: shards,
        }
    }

    fn schema_prefix(&self) -> &'static str {
        "shard"
    }

    pub fn finish(self) -> Result<()> {
        self.xact.commit().map_err(Error::TransactionCommit)
    }

    pub fn setup(&self) -> Result<()> {
        self.xact
            .execute("SET search_path TO public", &[])
            .map_err(Error::SchemaSwitch)?;

        // We take this lock because there can be one, and only one, migration
        // running at a time. No parallel changes allowed.
        self.xact
            .execute("SELECT pg_advisory_xact_lock($1)", &[&SETUP_LOCK])
            .map_err(Error::MigrationLock)?;

        let schema_prefix = self.schema_prefix();
        for shard in self.shards.iter() {
            let schema_xact = self.xact.transaction().map_err(Error::TransactionCreate)?;
            schema_xact
                .execute("SET search_path TO public", &[])
                .map_err(Error::SchemaSwitch)?;

            let schema_name = format!("{}_{}", schema_prefix, shard);
            let sql_create_schema = format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name);
            match schema_xact.execute(&sql_create_schema, &[]) {
                Ok(_) => {}
                Err(postgres::error::Error::Db(db_error)) => {
                    match db_error.code {
                        postgres::error::SqlState::UniqueViolation => {
                            debug!(
                                "This is a concurrency bug with schema creation - you can \
                                    ignore it"
                            )
                        }
                        _ => return Err(Error::SchemaCreate(postgres::error::Error::Db(db_error))),
                    }
                }
                Err(e) => return Err(Error::SchemaCreate(e)),
            }

            let set_search_path = format!("SET search_path TO {}", schema_name);
            schema_xact.execute(&set_search_path, &[]).map_err(
                Error::SchemaSwitch,
            )?;

            debug!("=> START: Migration.");
            schema_xact
                .execute(
                    r#"CREATE SEQUENCE IF NOT EXISTS builder_db_migrations_id_seq"#,
                    &[],
                )
                .map_err(Error::MigrationTable)?;

            debug!("=> [✓] builder_db_migrations_id_seq");

            let next_id_v1 = format!(
                r#"CREATE OR REPLACE FUNCTION next_id_v1(sequence_id regclass, OUT result bigint) AS $$
                DECLARE
                    our_epoch bigint := 1409266191000;
                    seq_id bigint;
                    now_millis bigint;
                    shard_id int := {};
                BEGIN
                    SELECT nextval(sequence_id) % 1024 INTO seq_id;
                    SELECT FLOOR(EXTRACT(EPOCH FROM clock_timestamp()) * 1000) INTO now_millis;
                    result := (now_millis - our_epoch) << 23;
                    result := result | (seq_id << 13);
                    result := result | (shard_id);
                END;
                $$ LANGUAGE PLPGSQL;"#,
                shard
            );
            schema_xact.execute(&next_id_v1, &[]).map_err(
                Error::FunctionCreate,
            )?;

            debug!("=> [✓] fn: next_id_v1 in {}", schema_name);


            schema_xact
                .execute(
                    r#"CREATE TABLE IF NOT EXISTS builder_db_migrations (
                id bigint PRIMARY KEY DEFAULT next_id_v1('builder_db_migrations_id_seq'),
                prefix text NOT NULL,
                hashed_content varchar(64),
                created_at timestamptz DEFAULT now(),
                updated_at timestamptz)"#,
                    &[],
                )
                .map_err(Error::MigrationTable)?;

            debug!("=> [✓] builder_db_migrations in {}", schema_name);


            schema_xact
                .execute(
                    r#"CREATE OR REPLACE FUNCTION migration_has_run_v1(p text, hsh text) RETURNS bool AS $$
            DECLARE
                result BOOLEAN;
            BEGIN
                SELECT true FROM builder_db_migrations WHERE prefix = p AND hashed_content = hsh INTO result;
                RETURN result;
            END
            $$ LANGUAGE plpgsql STABLE
            "#,
                    &[],
                )
                .map_err(Error::FunctionCreate)?;
            schema_xact.commit().map_err(Error::TransactionCommit)?;
            debug!("=> DONE: Migration");

        }
        Ok(())
    }


    pub fn migrate(&mut self, prefix: &str, sql: &str) -> Result<()> {
        let hashed_content = hash_string(sql);

        for shard in self.shards.iter() {
            let schema_prefix = self.schema_prefix();
            let schema_name = format!("{}_{}", schema_prefix, shard);
            let set_search_path = format!("SET search_path TO {}", schema_name);
            self.xact.execute(&set_search_path, &[]).map_err(
                Error::SchemaSwitch,
            )?;

            let result = check_migration_has_run(&self.xact, prefix, &hashed_content)?;

            if !result.is_some() {
                self.xact.execute(sql, &[]).map_err(Error::Migration)?;
                self.xact
                    .execute(
                        "INSERT INTO builder_db_migrations (prefix, hashed_content) VALUES \
                              ($1, $2)",
                        &[&prefix, &hashed_content],
                    )
                    .map_err(Error::MigrationTracking)?;
            }
        }

        Ok(())
    }
}

fn check_migration_has_run(xact: &postgres::transaction::Transaction, prefix: &str, hsh: &str) -> Result<Option<bool>> {
    let check_result = xact.query("SELECT migration_has_run_v1($1, $2)", &[&prefix, &hsh])
        .map_err(Error::MigrationCheck)?;
    Ok(check_result.get(0).get(0))
}
