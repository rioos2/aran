use std::env;
use async::{AsyncServer};
use error::{Error as DbError, Result as DbResult};
use error::{Result, Error};
use pool::Pool;
use config::DataStore;
use iron::Handler;
use iron::headers::{self, Authorization, Bearer};
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::Status;
use iron::typemap::Key;
use protocol::{Routable, RouteKey, ShardId, SHARD_COUNT};
use migration::Migrator;


pub struct DataStoreBroker;

impl Key for DataStoreBroker {
    type Value = DataStoreConn;
}

impl BeforeMiddleware for DataStoreBroker {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let ds = DataStoreConn::new().unwrap();
        ds.setup().unwrap();
        req.extensions.insert::<DataStoreBroker>(ds);
        Ok(())
    }
}


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
pub fn setup(&self) -> Result<()> {
    let conn = self.pool.get_raw()?;
    let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
    let mut migrator = Migrator::new(xact, self.pool.shards.clone());

    migrator.setup()?;

    // The core jobs table
       migrator.migrate(
           "asmsrv",
           r#"CREATE TABLE  IF NOT EXISTS assembly (
             id serial PRIMARY KEY,
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

    // Insert a new job into the jobs table
    migrator.migrate("asmsrv",
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
                            "#)?;
    // Hey, Adam - why did you do `select *` here? Isn't that bad?
    //
    // So glad you asked. In this case, it's better - essentially we have an API call that
    // returns a job object, which is flattened into the table structure above. We then
    // translate those job rows into Job structs. Since the table design is purely additive,
    // this allows us to add data to the table without having to re-roll functions that
    // generate Job structs, and keeps things DRY.
    //
    // Just make sure you always address the columns by name, not by position.
    migrator.migrate(
        "asmsrv",
        r#"CREATE OR REPLACE FUNCTION get_assembly_v1 (aid uuid) RETURNS SETOF assembly AS $$
                        BEGIN
                          RETURN QUERY SELECT * FROM assembly WHERE id = aid;
                          RETURN;
                        END
                        $$ LANGUAGE plpgsql STABLE"#,
    )?;

    migrator.finish()?;
    Ok(())
}

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }
}
