// Copyright 2018 The Rio Advancement Inc

//! The AppStore Differ hook, responsible for keeping Rio/OS in sync
//  with the rio.appstore

use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::{Error, Result};
use hooks::BeforeHook;
use protocol::api::base::MetaFields;
use protocol::api::blueprint;
use rio_core::{crypto::default_rioconfig_key_path, fs::open_from};
use serde_json;
use serde_yaml;
use std::io::{Error as IOError, ErrorKind};
use std::path::PathBuf;

// The time in seconds to perform a sync again. Default is 3 minutes
const SYNC_ELAPSED_SECONDS: i64 = 180;

//The location of the downloaded appstores file 
lazy_static! {
    static ref APPSTORE_CACHE_FILE: PathBuf = PathBuf::from(&*default_rioconfig_key_path(None).join("pullcache/appstores.yaml").to_str().unwrap());
}

#[derive(Debug, Deserialize)]
struct MarketPlaceDownload {
    kind: String,
    api_version: String,
    items: Vec<blueprint::Plan>,
    time_stamp: String,
}

pub struct AppStore {
    conn: Box<DataStoreConn>,
}

impl AppStore {
    pub fn new(conn: Box<DataStoreConn>) -> Self {
        AppStore { conn: conn }
    }

    //TO-DO: This must be written like settings.rs. 
    //The DB connection shouldn't be used here. 
    //Baseically the file pullcache/appstores.yaml is verified for length and loaded as a secret
    fn diff_and_create(&self) -> Result<()> {
        let conn = self.conn.pool.get_shard(0)?;
        info!("Locating {:?}", APPSTORE_CACHE_FILE.to_str());

        let file = open_from(&APPSTORE_CACHE_FILE.as_path())?;
        if file.metadata()?.len() <= 0 {
            return Err(Error::IO(IOError::new(
                ErrorKind::Other,
                format!(
                    "oh no! {:?} is empty.\nRun `rioos-apiserver sync` to pull a fresh copy from Rio.AppStore.",
                    APPSTORE_CACHE_FILE.to_str()
                ),
            )));
        }
        let u: MarketPlaceDownload = serde_yaml::from_reader(file)?;
        info!("Loaded {:?}", APPSTORE_CACHE_FILE.to_str());

        elapsed_or_return(u.time_stamp);
        info!("Loooks like we elapsed, updating again with yaml {:?}", APPSTORE_CACHE_FILE.to_str());

        u.items
            .iter()
            .map(|x| {
                &conn
                    .query(
                        "SELECT * FROM select_or_insert_plan_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
                        &[
                            &(x.object_meta().name as String),
                            &(serde_json::to_value(x.type_meta()).unwrap()),
                            &(serde_json::to_value(x.object_meta()).unwrap()),
                            &(serde_json::to_value(x.get_plan()).unwrap()),
                            &(x.get_category() as String),
                            &(x.get_version() as String),
                            &(x.get_icon() as String),
                            &(x.get_description() as String),
                            &(serde_json::to_value(x.get_status()).unwrap()),
                        ],
                    )
                    .unwrap();
            })
            .collect::<Vec<_>>();
        Ok(())
    }
}

//The trait responsible for executing the appstore workload hook
//By default, we say that the statup hook needs to executed.
//This can be controlled by overriding satisfied function.
// In this case the satisfied is true, if the file pullcache/appstores.yaml exists
// The condition isn't active yet.
impl BeforeHook for AppStore {
    fn before(&self) -> Result<()> {
        self.diff_and_create()?;
        Ok(())
    }
}

fn elapsed_or_return(time: String) -> () {
    let now_time = DateTime::parse_from_rfc3339(&Utc::now().to_rfc3339().to_string()).unwrap();
    let time_stamp = DateTime::parse_from_rfc3339(&time.to_string()).unwrap();
    let diff = now_time.timestamp() - time_stamp.timestamp();
    if diff < SYNC_ELAPSED_SECONDS {
        return;
    }
}
