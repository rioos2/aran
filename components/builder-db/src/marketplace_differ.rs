use std::path::PathBuf;
use std::fs::File;

use error::Result;
use serde_json;
use serde_yaml;
use chrono::prelude::*;

use data_store::DataStoreConn;

use protocol::api::blueprint;
use protocol::api::base::MetaFields;
const SYNC_ELAPSED_SECONDS: i64 = 180;

use rcore::crypto::default_rioconfig_key_path;

lazy_static! {
    static  ref MARKETPLACE_CACHE_FILE: PathBuf =  PathBuf::from(&*default_rioconfig_key_path(None).join("pullcache/marketplaces.yaml").to_str().unwrap());
}

#[derive(Debug, Deserialize)]
struct MarketPlaceDownload {
    kind: String,
    api_version: String,
    items: Vec<blueprint::Plan>,
    time_stamp: String,
}

pub struct MarketPlaceDiffer {
    conn: DataStoreConn,
}

impl MarketPlaceDiffer {
    pub fn new(conn: DataStoreConn) -> Self {
        MarketPlaceDiffer { conn: conn }
    }
    pub fn setup(&self) -> Result<()> {
        self.diff_and_create()?;
        Ok(())
    }
    fn diff_and_create(&self) -> Result<()> {
        let conn = self.conn.pool.get_shard(0)?;

        let file = File::open(&MARKETPLACE_CACHE_FILE.as_path())?;
        let u: MarketPlaceDownload = serde_yaml::from_reader(file)?;
        elapsed_or_return(u.time_stamp);
        u.items
            .iter()
            .map(|x| {
                &conn.query(
                    "SELECT * FROM select_or_insert_plan_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
                    &[
                        &(x.object_meta().name as String),
                        &(serde_json::to_value(x.type_meta()).unwrap()),
                        &(serde_json::to_value(x.object_meta()).unwrap()),
                        &(x.get_category() as String),
                        &(x.get_version() as String),
                        &(serde_json::to_value(x.get_characteristics()).unwrap()),
                        &(x.get_icon() as String),
                        &(x.get_description() as String),
                        &(serde_json::to_value(x.get_ports()).unwrap()),
                        &(serde_json::to_value(x.get_envs()).unwrap()),
                        &(serde_json::to_value(x.get_lifecycle()).unwrap()),
                        &(serde_json::to_value(x.get_status()).unwrap()),
                    ],
                ).unwrap();
            })
            .collect::<Vec<_>>();
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
