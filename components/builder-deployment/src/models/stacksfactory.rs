// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the StacksFactory.
use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::base::{IdGet, MetaFields, StatusUpdate};
use protocol::api::deploy;
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_DIRECTLY, PULL_INVALDATED};

use db::data_store::DataStoreConn;
use postgres;

use serde_json;

use super::super::{StacksFactoryOutput, StacksFactoryOutputList};

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

    pub fn create(&self, factory: &deploy::StacksFactory) -> StacksFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_stacks_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
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
        ).map_err(Error::StacksFactoryCreate)?;

        if rows.len() > 0 {
            for row in rows {
                let stacks = self.collect_spec(&row, PULL_DIRECTLY)?;
                return Ok(Some(stacks));
            }
        }

        Ok(None)
    }

    pub fn show(&self, get_stacks_factory: &IdGet) -> StacksFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_stacks_factory_v1($1)",
            &[&(get_stacks_factory.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::StacksFactoryGet)?;

        if rows.len() > 0 {
            for row in rows {
                let stacks = self.collect_spec(&row, PULL_DIRECTLY)?;
                return Ok(Some(stacks));
            }
        }
        Ok(None)
    }

    pub fn status_update(&self, upd: &StatusUpdate) -> StacksFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_stacks_factorys_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::StacksFactoryUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let stacks = self.collect_spec(&row, PULL_INVALDATED)?;
                return Ok(Some(stacks));
            }
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> StacksFactoryOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_stacks_factorys_v1()", &[])
            .map_err(Error::StacksFactoryGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.collect_spec(&row, PULL_DIRECTLY)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list(&self, get_stacks_factory: &IdGet) -> StacksFactoryOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_stacks_factory_by_account_v1($1)",
            &[&(get_stacks_factory.get_name() as String)],
        ).map_err(Error::StacksFactoryGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.collect_spec(&row, PULL_DIRECTLY)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    /// Expands the assembly by sticking in Spec
    ///         1. AssemblyFactory (parent information)
    ///         2. Plan
    fn collect_spec(
        &self,
        row: &postgres::rows::Row,
        how_to: PullFromCache,
    ) -> Result<deploy::StacksFactory> {
        let mut stacks = self.row_to_stacks_factory(&row)?;
        self.expander.with_plan(&mut stacks, how_to);
        Ok(stacks)
    }

    fn row_to_stacks_factory(&self, row: &postgres::rows::Row) -> Result<deploy::StacksFactory> {
        let mut stacks_factory = deploy::StacksFactory::with(
            serde_json::from_value(row.get("type_meta")).unwrap(),
            serde_json::from_value(row.get("object_meta")).unwrap(),
        );

        let id: i64 = row.get("id");
        let replicas: i16 = row.get("replicas");
        let created_at = row.get::<&str, DateTime<Utc>>("created_at");
        let plan: i64 = row.get("plan");

        stacks_factory.set_id(id.to_string());
        stacks_factory.set_created_at(created_at.to_rfc3339());
        stacks_factory.set_status(serde_json::from_value(row.get("status")).unwrap());
        stacks_factory.set_resources(serde_json::from_value(row.get("resources")).unwrap());
        stacks_factory.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
        stacks_factory.set_secret(serde_json::from_value(row.get("secret")).unwrap());
        stacks_factory.set_plan(plan.to_string());
        stacks_factory.set_replicas(replicas as u32);
        stacks_factory.set_spec(serde_json::from_value(row.get("spec")).unwrap());

        Ok(stacks_factory)
    }
}
