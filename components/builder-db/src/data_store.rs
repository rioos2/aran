// Copyright (c) 2017 RioCorp Inc.

use async::AsyncServer;
use error::{Result, Error};
use pool::Pool;
use config::DataStore;
use protocol::SHARD_COUNT;
use migration::{Migratable, Migrator};
use auth_storedproc::*;
use deploy_storedproc::*;
use plan_storedproc::*;
use scale_storedproc::*;
use node_storedproc::*;
use secret_storedproc::*;


/// A messaging Broker for proxying messages from clients to one or more `RouteSrv` and vice versa.
pub struct Broker {}

impl Broker {
    pub fn connect() -> Result<DataStoreConn> {
        let conn = DataStoreConn::new()?;
        Ok(conn)
    }
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

        self.setup_fromsrvs(&mut migrator).unwrap();

        migrator.finish()?;


        Ok(self)
    }

    //this returns trait objects Migratable,
    //rust has a difference between trait, trait objects as its statically typed
    fn setup_fromsrvs(&self, migrator: &mut Migrator) -> Result<()> {
        //wanted to do it using an identifier string that when matched will call the
        //migratable interface trait objects
        //this is more like flatMap in scala and we are good now.
        AuthProcedures::new()?.migrate(migrator)?;
        DeployProcedures::new()?.migrate(migrator)?;
        PlanProcedures::new()?.migrate(migrator)?;
        ScaleProcedures::new()?.migrate(migrator)?;
        NodeProcedures::new()?.migrate(migrator)?;
        SecretProcedures::new()?.migrate(migrator)?;
        Ok(())
    }
}
