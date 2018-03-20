// Copyright 2018 The Rio Advancement Inc

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
use job_storedproc::*;
use volume_storedproc::*;
use watch_storedproc::*;
use package_storedproc::*;
use marketplace_storedproc::*;
use devtooling_storedproc::*;
use system_secret::*;
use marketplace_differ::*;

use protocol::cache::InMemoryExpander;

#[derive(Clone)]
pub struct DataStoreConn {
    pub pool: Pool,
    pub expander: InMemoryExpander,
}

impl DataStoreConn {
    pub fn new() -> Result<DataStoreConn> {
        let datastore = DataStore::default();
        let pool = Pool::new(&datastore, (0..SHARD_COUNT).collect())?;
        Ok(DataStoreConn {
            pool: pool,
            expander: InMemoryExpander::new(),
        })
    }
    // Create a new DataStore from a pre-existing pool; useful for testing the database.
    pub fn from_pool(pool: Pool) -> Result<DataStoreConn> {
        Ok(DataStoreConn {
            pool: pool,
            expander: InMemoryExpander::new(),
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

        self.setup_fromsrvs(&mut migrator, ui)?;

        migrator.finish()?;
        ui.end("Database setup complete.");
        SystemSecret::new(self.clone()).setup()?;
        MarketPlaceDiffer::new(self.clone()).setup()?;
        Ok(self)
    }


    /// Setup the datastore.
    /// This includes all the schema and data migrations, along with stored procedures for data
    /// access.
    pub fn setup_marketplace(&self, ui: &mut UI) -> Result<&DataStoreConn> {
        ui.heading("Database");
        ui.begin("Auto Migration...");
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;

        self.setup_fromsrvs(&mut migrator, ui)?;

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
        JobProcedures::new()?.migrate(migrator, ui)?;
        VolumeProcedures::new()?.migrate(migrator, ui)?;
        WatchProcedures::new()?.migrate(migrator, ui)?;
        PackageProcedures::new()?.migrate(migrator, ui)?;
        MarketPlaceProcedures::new()?.migrate(migrator, ui)?;
        DevtoolingProcedures::new()?.migrate(migrator, ui)?;
        Ok(())
    }
}
