// Copyright 2018 The Rio Advancement Inc
embed_migrations!("appstore/migrations");

use super::diesel_pool::DieselPool;
use super::migration::shard_setup;
use config::DataStore;
use diesel::result::Error as Dre;
use diesel::Connection;
use error::Result;
use pool::Pool;
use protocol::{ShardId, SHARD_COUNT};
use std::io;

use protocol::cache::InMemoryExpander;

#[derive(Clone)]
pub struct DataStoreConn {
    pub pool: Pool,
    pub diesel_pool: DieselPool,
    pub expander: InMemoryExpander,
}

impl DataStoreConn {
    pub fn new() -> Result<DataStoreConn> {
        let datastore = DataStore::default();
        let diesel_pool = DieselPool::new(&datastore)?;
        let pool = Pool::new(&datastore, (0..SHARD_COUNT).collect())?;
        Ok(DataStoreConn {
            pool: pool,
            diesel_pool: diesel_pool,
            expander: InMemoryExpander::new(),
        })
    }

    /// Setup the datastore.
    /// This includes all the schema and data migrations, along with stored procedures for data
    /// access.
    pub fn setup(&self) -> Result<()> {
        let conn = self.diesel_pool.get_raw()?;
        let shard_id: ShardId = 0;
        let _ = conn.transaction::<_, Dre, _>(|| {
            shard_setup(&*conn, shard_id).unwrap();
            embedded_migrations::run_with_output(&*conn, &mut io::stdout()).unwrap();
            Ok(())
        });
        Ok(())
    }
}
