// Copyright 2018 The Rio Advancement Inc

use std::ops::{Deref, DerefMut};
use std::thread;
use std::time::Duration;
use std::fmt;

use fnv::FnvHasher;
use rand::{self, Rng};
use r2d2;
use r2d2_postgres::{self, PostgresConnectionManager, TlsMode};
use protocol::{ShardId, SHARD_COUNT};
// use postgres::tls::openssl::OpenSsl;
use config::DataStore;
use error::{Error, Result};

//
//This is for future use. Where we could shard in the database based on functionality
//
use protocol::api::routesrv::{Routable, RouteKey};

#[derive(Clone)]
pub struct Pool {
    inner: r2d2::Pool<PostgresConnectionManager>,
    pub shards: Vec<ShardId>,
}

impl fmt::Debug for Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Pool {{ inner: {:?}, shards: {:?} }}",
            self.inner,
            self.shards
        )
    }
}

impl Pool {
    pub fn new(config: &DataStore, shards: Vec<ShardId>) -> Result<Pool> {
        loop {
            // let openssl = OpenSsl::new().unwrap();
            //
            // let manager = PostgresConnectionManager::new(config, TlsMode::Require(Box::new(openssl)))?;
            let manager = PostgresConnectionManager::new(config, TlsMode::None)?;

            match r2d2::Pool::builder()
                .max_size(config.pool_size)
                .connection_timeout(Duration::from_secs(config.connection_timeout_sec))
                .build(manager) {
                Ok(pool) => {
                    return Ok(Pool {
                        inner: pool,
                        shards: shards,
                    })
                }
                Err(e) => {
                    error!(
                        "Error initializing connection pool to Postgres, will retry: {}",
                        e
                    )
                }
            }
            thread::sleep(Duration::from_millis(config.connection_retry_ms));
        }
    }

    pub fn get_raw(&self) -> Result<r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>> {
        let conn = self.inner.get().map_err(Error::ConnectionTimeout)?;
        Ok(conn)
    }

    pub fn get_shard(&self, shard_id: u32) -> Result<r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>> {
        let conn = self.inner.get().map_err(Error::ConnectionTimeout)?;
        debug!("Switching to shard {}", shard_id);

        let schema_name = format!("shard_{}", shard_id);
        let sql_search_path = format!("SET search_path TO {}", schema_name);
        conn.execute(&sql_search_path, &[]).map_err(
            Error::SchemaSwitch,
        )?;
        Ok(conn)
    }

    pub fn get<T: Routable>(&self, routable: &T) -> Result<r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>> {
        let optional_shard_id = routable.route_key().map(
            |k| k.hash(&mut FnvHasher::default()),
        );

        let shard_id = match optional_shard_id {
            Some(id) => (id % SHARD_COUNT as u64) as u32,
            None => {
                let mut rng = rand::thread_rng();
                match rng.choose(&self.shards) {
                    Some(shard) => *shard,
                    None => 0,
                }
            }
        };
        self.get_shard(shard_id)
    }
}

impl Deref for Pool {
    type Target = r2d2::Pool<PostgresConnectionManager>;

    fn deref(&self) -> &r2d2::Pool<PostgresConnectionManager> {
        &self.inner
    }
}

impl DerefMut for Pool {
    fn deref_mut(&mut self) -> &mut r2d2::Pool<PostgresConnectionManager> {
        &mut self.inner
    }
}
