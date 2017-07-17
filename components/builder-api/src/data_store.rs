use db::async::{AsyncServer, EventOutcome};
use db::error::{Error as DbError, Result as DbResult};
use db::pool::Pool;

pub struct DataStoreConn {
    pool: Pool,
    pub async: AsyncServer,
}

impl DataStoreConn {
    pub fn new(config: &Config) -> Result<DataStore> {
        let pool = Pool::new(&config.datastore, config.shards.clone())?;
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }
    /// Create a new DataStore from a pre-existing pool; useful for testing the database.
    pub fn from_pool(pool: Pool) -> Result<DataStore> {
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }

    pub fn setup(&self) -> Result<()> {
        // let conn = self.pool.get_raw()?;
        // let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        //
        // self.async.register("sync_jobs".to_string(), sync_jobs);

        Ok(())
    }

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }
}
