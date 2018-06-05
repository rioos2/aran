// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [blockchain, blockchainfactory].
use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::deploy;
use protocol::api::base::{IdGet, StatusUpdate, MetaFields};
use protocol::cache::{PULL_DIRECTLY, PULL_INVALDATED, PullFromCache, InMemoryExpander};

use postgres;
use db::data_store::DataStoreConn;

use serde_json;

use super::super::{BlockchainFactoryOutput, BlockchainFactoryOutputList};

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
    expander: &'a InMemoryExpander,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore {
            db: db,
            expander: &db.expander,
        }
    }

    pub fn create(&self, factory: &deploy::BlockchainFactory) -> BlockchainFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_blockchain_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(serde_json::to_value(factory.object_meta()).unwrap()),
                &(serde_json::to_value(factory.type_meta()).unwrap()),
                &(factory.get_replicas() as i16),
                &(serde_json::to_value(factory.get_resources()).unwrap()),
                &(serde_json::to_value(factory.get_metadata()).unwrap()),
                &(serde_json::to_value(factory.get_status()).unwrap()),
                &(serde_json::to_value(factory.get_secret()).unwrap()),
                &(factory.get_plan().parse::<i64>().unwrap()),
                &(serde_json::to_value(factory.get_spec()).unwrap()),
            ],
        ).map_err(Error::BlockchainFactoryCreate)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_blockchain_factory(&row, PULL_DIRECTLY)?));
            }
        }

        Ok(None)
    }

    pub fn show(&self, get_blockchain_factory: &IdGet) -> BlockchainFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_blockchain_factory_v1($1)",
            &[&(get_blockchain_factory.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::BlockchainFactoryGet)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_blockchain_factory(&row, PULL_DIRECTLY)?));
            }
        }
        Ok(None)
    }

    pub fn status_update(&self, upd: &StatusUpdate) -> BlockchainFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_blockchain_factorys_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::BlockchainFactoryUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_blockchain_factory(&row, PULL_INVALDATED)?));
            }
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> BlockchainFactoryOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_blockchains_factory_v1()", &[])
            .map_err(Error::BlockchainFactoryGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_blockchain_factory(&row, PULL_DIRECTLY)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list(&self, get_blockchain_factory: &IdGet) -> BlockchainFactoryOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_blockchain_factory_by_account_v1($1)",
            &[&(get_blockchain_factory.get_name() as String)],
        ).map_err(Error::BlockchainFactoryGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_blockchain_factory(&row, PULL_DIRECTLY)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    fn row_to_blockchain_factory(&self, row: &postgres::rows::Row, how_to: PullFromCache) -> Result<deploy::BlockchainFactory> {
        let mut blockchain_factory = deploy::BlockchainFactory::with(
            serde_json::from_value(row.get("type_meta")).unwrap(),
            serde_json::from_value(row.get("object_meta")).unwrap(),
        );

        let id: i64 = row.get("id");
        let replicas: i16 = row.get("replicas");
        let created_at = row.get::<&str, DateTime<Utc>>("created_at");
        let plan: i64 = row.get("plan");

        blockchain_factory.set_id(id.to_string());
        blockchain_factory.set_created_at(created_at.to_rfc3339());
        blockchain_factory.set_status(serde_json::from_value(row.get("status")).unwrap());
        blockchain_factory.set_resources(serde_json::from_value(row.get("resources")).unwrap());
        blockchain_factory.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
        blockchain_factory.set_secret(serde_json::from_value(row.get("secret")).unwrap());
        blockchain_factory.set_plan(plan.to_string());
        blockchain_factory.set_spec(serde_json::from_value(row.get("spec")).unwrap());
        blockchain_factory.set_replicas(replicas as u32);
        self.expander.with_plan(&mut blockchain_factory, how_to);

        //BlockchainFactory is created first, and service is created later in our process. During the creation AF sets the service to none
        //Hence the cache always return none for service. To fix this the service is pulled invalidated from cache - send a live copy always
        self.expander.with_services(
            &mut blockchain_factory,
            PULL_INVALDATED,
        );
        Ok(blockchain_factory)
    }
}
