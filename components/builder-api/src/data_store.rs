use db::async::{AsyncServer, EventOutcome};
use db::error::{Error as DbError, Result as DbResult};
use db::pool::Pool;

pub struct DataStoreBroker;

impl Key for DataStoreBroker {
    type Value = DataStoreConn;
}

impl BeforeMiddleware for DataStoreBroker {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let ds = DataStoreConn::new().unwrap();
        req.extensions.insert::<DataStoreBroker>(ds);
        Ok(())
    }
}


pub struct DataStoreConn {
    pool: Pool,
    pub async: AsyncServer,
}

impl DataStoreConn {
    pub fn new() -> Result<DataStore> {
        let shards = (0..SHARD_COUNT).collect();
        let mut datastore = DataStore::default();

        let pool = Pool::new(datastore, shards.clone())?;

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
        Ok(())
    }

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }
}
