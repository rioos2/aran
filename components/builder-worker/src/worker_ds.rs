// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use error::{Result, Error};
// use postgres;
// use postgres::notification;
// use db::error::{Error as DbError, Result as DbResult};
use db::data_store::DataStoreConn;
// use db::async::{AsyncServer, EventOutcome};
use db::pool::Pool;
//use serde_json;
// use std::io;
// use std::io::Write;

pub struct WorkerDS {}

impl WorkerDS {
    pub fn worker_stream(datastore: &DataStoreConn) -> Result<()> {
/*        let conn = datastore.pool.get_shard(0)?;
        let data = &conn.query("LISTEN assemblyfac", &[]).map_err(
            DbError::AsyncFunctionCheck,
        )?;
        debug!(">● ROWS: hs_create =>\n{:?}", &data);
        datastore.async.register("sync_jobs".to_string(), sync_jobs);
        datastore.start_async();
*/        Ok(())

    }
}

/*fn sync_jobs(pool: Pool) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(0)?;
        let n = conn.notifications();

        let mut it = n.iter();

        // for i in 0..99999999 {
        //     let a = it.next();
        //     match a {
        //         Some(b) => {
        //             println!("{:?}", b);
        //         }
        //         _ => {} // execute a noop query to pick up new messages
        //     }
        // }
        result = EventOutcome::Retry;
    }
    Ok(result)
}
*/
