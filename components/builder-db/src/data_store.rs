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
    pub pool: Pool,
    pub async: AsyncServer,
}

impl DataStoreConn {
    pub fn new() -> Result<DataStoreConn> {
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@2");
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

    pub fn setup(&self) ->Result<()>{
        Ok(())
    }

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }
}
