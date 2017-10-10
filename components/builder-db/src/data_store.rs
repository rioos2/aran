// Copyright (c) 2017 RioCorp Inc.

use async::AsyncServer;
use error::{Result, Error};
use pool::Pool;
use config::DataStore;
use common::ui::UI;
use protocol::SHARD_COUNT;
use migration::{Migratable, Migrator};
use auth_storedproc::*;
use deploy_storedproc::*;
use plan_storedproc::*;
use scale_storedproc::*;
use node_storedproc::*;
use service_account_storedproc::*;
use network_storedproc::*;
use storage_storedproc::*;


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
    pub fn setup(&self, ui: &mut UI) -> Result<&DataStoreConn> {
        ui.heading("Database");
        ui.begin("Auto Migration...");
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;

        self.setup_fromsrvs(&mut migrator, ui).unwrap();

        migrator.finish()?;
        ui.end("Database setup complete.");

        Ok(self)
    }

    //this returns trait objects Migratable,
    //rust has a difference between trait, trait objects as its statically typed
    fn setup_fromsrvs(&self, migrator: &mut Migrator, ui: &mut UI) -> Result<()> {
        //wanted to do it using an identifier string that when matched will call the
        //migratable interface trait objects
        //this is more like flatMap in scala and we are good now.
        AuthProcedures::new()?.migrate(migrator, ui)?;
        DeployProcedures::new()?.migrate(migrator, ui)?;
        PlanProcedures::new()?.migrate(migrator, ui)?;
        ScaleProcedures::new()?.migrate(migrator, ui)?;
        NodeProcedures::new()?.migrate(migrator, ui)?;
        NetworkProcedures::new()?.migrate(migrator, ui)?;
        StorageProcedures::new()?.migrate(migrator, ui)?;
        ServiceAccountProcedure::new()?.migrate(migrator, ui)?;
//        WorkerProcedures::new()?.migrate(migrator)?;
        Ok(())
    }
}
