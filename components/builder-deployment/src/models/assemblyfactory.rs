// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Deployment - AssemblyFactory
use super::super::{AssemblyFactoryOutput, AssemblyFactoryOutputList};
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::{Error, Result};
use postgres;
use protocol::api::base::{IdGet, MetaFields, StatusUpdate};
use protocol::api::deploy;
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_DIRECTLY, PULL_INVALDATED};
use serde_json;

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

    pub fn create(&self, factory: &deploy::AssemblyFactory) -> AssemblyFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_assembly_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
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
        ).map_err(Error::AssemblyFactoryCreate)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_assembly_factory(&row, PULL_DIRECTLY)?));
            }
        }

        Ok(None)
    }

    pub fn show(&self, get_assembly_factory: &IdGet) -> AssemblyFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assembly_factory_v1($1)",
            &[&(get_assembly_factory.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyFactoryGet)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_assembly_factory(&row, PULL_DIRECTLY)?));
            }
        }
        Ok(None)
    }

    pub fn show_by_stacksfactory(&self, id: &IdGet) -> AssemblyFactoryOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblyfactorys_by_parentid_v1($1)",
            &[&(id.get_id() as String)],
        ).map_err(Error::AssemblyFactoryGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_assembly_factory(&row, PULL_DIRECTLY)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn status_update(&self, upd: &StatusUpdate) -> AssemblyFactoryOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM set_assembly_factorys_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::AssemblyFactoryUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_assembly_factory(&row, PULL_INVALDATED)?));
            }
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> AssemblyFactoryOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_assemblys_factory_v1()", &[])
            .map_err(Error::AssemblyFactoryGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_assembly_factory(&row, PULL_DIRECTLY)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list(&self, get_assembly_factory: &IdGet) -> AssemblyFactoryOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assembly_factory_by_account_v1($1)",
            &[&(get_assembly_factory.get_name() as String)],
        ).map_err(Error::AssemblyFactoryGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_assembly_factory(&row, PULL_DIRECTLY)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    /// A private convertor of postgres Row to the required structure.
    /// In this case AssemblyFactory.
    fn row_to_assembly_factory(
        &self,
        row: &postgres::rows::Row,
        how_to: PullFromCache,
    ) -> Result<deploy::AssemblyFactory> {
        let mut assembly_factory = deploy::AssemblyFactory::with(
            serde_json::from_value(row.get("type_meta")).unwrap(),
            serde_json::from_value(row.get("object_meta")).unwrap(),
        );

        let id: i64 = row.get("id");
        let replicas: i16 = row.get("replicas");
        let created_at = row.get::<&str, DateTime<Utc>>("created_at");
        let plan: i64 = row.get("plan");

        assembly_factory.set_id(id.to_string());
        assembly_factory.set_created_at(created_at.to_rfc3339());
        assembly_factory.set_status(serde_json::from_value(row.get("status")).unwrap());
        assembly_factory.set_resources(serde_json::from_value(row.get("resources")).unwrap());
        assembly_factory.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
        assembly_factory.set_secret(serde_json::from_value(row.get("secret")).unwrap());
        assembly_factory.set_plan(plan.to_string());
        assembly_factory.set_spec(serde_json::from_value(row.get("spec")).unwrap());
        assembly_factory.set_replicas(replicas as u32);

        // Pull the parent information - stacks
        self.expander.with_stacks(&mut assembly_factory, how_to);
        self.expander.with_plan(&mut assembly_factory, how_to);

        // AssemblyFactory is created first, and service is created later in our process.
        // During the creation AF sets the service to none
        // Hence the cache always return none for service.
        // HACK: To fix this the service is pulled invalidated from cache - send a LIVE copy always.
        self.expander
            .with_services(&mut assembly_factory, PULL_INVALDATED);
        Ok(assembly_factory)
    }
}
