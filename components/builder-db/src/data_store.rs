use std::env;
use async::AsyncServer;
use error::{Result, Error};
use pool::Pool;
use config::DataStore;
use iron::typemap::Key;
use protocol::SHARD_COUNT;
use migration::Migrator;


pub struct DataStoreBroker;

impl Key for DataStoreBroker {
    type Value = DataStoreConn;
}

#[derive(Clone)]
pub struct DataStoreConn {
    pub pool: Pool,
    pub async: AsyncServer,
}

impl DataStoreConn {
    pub fn new() -> Result<DataStoreConn> {
        let datastore = DataStore::default();
        let pool = Pool::new(&datastore, (0..SHARD_COUNT).collect())?;
        let ap = pool.clone();
        Ok(DataStoreConn {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }
    // Create a new DataStore from a pre-existing pool; useful for testing the database.
    pub fn from_pool(pool: Pool) -> Result<DataStoreConn> {
        let ap = pool.clone();
        Ok(DataStoreConn {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }

    /// Setup the datastore.
    /// This includes all the schema and data migrations, along with stored procedures for data
    /// access.
    pub fn setup(&self) -> Result<&DataStoreConn> {
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;
        debug!("=> START: asmsrv");

        // The core asms table
        migrator.migrate(
            "asmsrv",
            r#"CREATE SEQUENCE IF NOT EXISTS asm_id_seq;"#,
        )?;

        debug!("=> [✓] asm_id_seq");

        migrator.migrate(
            "asmsrv",
            r#"CREATE TABLE  IF NOT EXISTS assembly (
             id bigint PRIMARY KEY DEFAULT next_id_v1('asm_id_seq'),
             uri text,
             name text,
             description text,
             tags text[],
             representation_skew text,
             external_management_resource text,
             component_collection text[],
             plan text,
             operation_collection text[],
             sensor_collection text[],
             metadata text,
             updated_at timestamptz,
             created_at timestamptz DEFAULT now())"#,
        )?;

        debug!("=> [✓] assembly");

        // Insert a new job into the jobs table
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION insert_assembly_v1 (
                            name text,
                            uri text,
                            description text,
                            tags text[],
                            external_management_resource text,
                            representation_skew text,
                            component_collection text[],
                            plan text,
                            operation_collection text[],
                            sensor_collection text[],
                            metadata text
                        ) RETURNS SETOF assembly AS $$
                                BEGIN
                                    RETURN QUERY INSERT INTO assembly(name, uri, description, tags, external_management_resource, representation_skew, component_collection,plan,operation_collection,sensor_collection,metadata)
                                        VALUES (name,uri, description, tags, external_management_resource, representation_skew, component_collection,plan,operation_collection,sensor_collection,metadata)
                                        RETURNING *;
                                    RETURN;
                                END
                            $$ LANGUAGE plpgsql VOLATILE
                            "#,
        )?;
        debug!("=> [✓] assembly");

        // Just make sure you always address the columns by name, not by position.
        migrator.migrate(
            "asmsrv",
            r#"CREATE OR REPLACE FUNCTION get_assembly_v1 (aid bigint) RETURNS SETOF assembly AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assembly WHERE id = aid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
        )?;

        debug!("=> [✓] fn: get_assembly_v1");
        migrator.finish()?;
        debug!("=> DONE: asmsrv");

        Ok(self)
    }

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }
}
