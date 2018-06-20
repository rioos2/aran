// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Deployment - Assembly
use super::super::{AssemblyOutput, AssemblyOutputList};
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::{Error, Result};
use postgres;
use protocol::api::base::{IdGet, MetaFields, StatusUpdate};
use protocol::api::{deploy, node};
use protocol::cache::{InMemoryExpander, PullFromCache, PULL_DIRECTLY, PULL_INVALDATED};
use serde_json;
use std::collections::BTreeMap;
use telemetry::metrics::collector::{Collector, CollectorScope};
use telemetry::metrics::prometheus::PrometheusClient;

const METRIC_LBL_RIOOS_ASSEMBLY_ID: &'static str = "rioos_assembly_id";
const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";

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

    pub fn create(&self, assembly: &deploy::Assembly) -> AssemblyOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_assembly_v1($1,$2,$3,$4,$5)",
            &[
                &(serde_json::to_value(assembly.type_meta()).unwrap()),
                &(serde_json::to_value(assembly.object_meta()).unwrap()),
                &(assembly.get_selector() as Vec<String>),
                &(serde_json::to_value(assembly.get_status()).unwrap()),
                &(serde_json::to_value(assembly.get_metadata()).unwrap()),
            ],
        ).map_err(Error::AssemblyCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = self.collect_spec(&row, PULL_DIRECTLY)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn update(&self, assembly: &deploy::Assembly) -> AssemblyOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_assembly_v1($1,$2,$3,$4,$5)",
            &[
                &(assembly.get_id().parse::<i64>().unwrap()),
                &(assembly.get_selector() as Vec<String>),
                &(serde_json::to_value(assembly.get_status()).unwrap()),
                &(serde_json::to_value(assembly.object_meta()).unwrap()),
                &(serde_json::to_value(assembly.get_metadata()).unwrap()),
            ],
        ).map_err(Error::AssemblyUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let assembly = self.collect_spec(&row, PULL_INVALDATED)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn show(&self, get_assembly: &IdGet) -> AssemblyOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_assembly_v1($1)",
            &[&(get_assembly.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::AssemblyGet)?;
        if rows.len() > 0 {
            for row in rows {
                let assembly = self.collect_spec(&row, PULL_DIRECTLY)?;
                return Ok(Some(assembly));
            }
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> AssemblyOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_assemblys_v1()", &[])
            .map_err(Error::AssemblyGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.collect_spec(&row, PULL_DIRECTLY)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list(&self, assemblys_get: &IdGet) -> AssemblyOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblys_by_account_v1($1)",
            &[&(assemblys_get.get_name() as String)],
        ).map_err(Error::AssemblyGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(self.collect_spec(&row, PULL_DIRECTLY)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn status_update(&self, upd: &StatusUpdate) -> AssemblyOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_assembly_status_v1($1, $2)",
            &[
                &(upd.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(upd.get_status()).unwrap()),
            ],
        ).map_err(Error::AssemblyUpdate)?;
        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.collect_spec(&row, PULL_INVALDATED)?));
            }
        }
        Ok(None)
    }

    pub fn show_by_assemblyfactory(&self, id: &IdGet) -> AssemblyOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_assemblys_by_parentid_v1($1)",
            &[&(id.get_id() as String)],
        ).map_err(Error::AssemblyGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.collect_spec(&row, PULL_DIRECTLY)?);
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    //Get the metrics as a map of assembly_id and its metric
    pub fn show_metrics(
        &self,
        id: &IdGet,
        prom: &PrometheusClient,
    ) -> Result<BTreeMap<String, String>> {
        match &id.get_name()[..] {
            "machine" => {
                let label_collection = vec![
                    format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLY_ID, id.get_id()).to_string(),
                    node::ASSEMBLY_JOBS.to_string(),
                    node::IDLEMODE.to_string(),
                ];
                let nodes_metric_scope: Vec<String> = vec!["node_cpu".to_string()];

                let scope = CollectorScope {
                    metric_names: nodes_metric_scope,
                    labels: label_collection,
                    last_x_minutes: METRIC_DEFAULT_LAST_X_MINUTE.to_string(),
                    avg_by_name: "rioos_assembly_id".to_string(),
                };
                Ok(Collector::new(prom, scope).metric_by_avg_for_machines()?)
            }
            "container" => {
                let label_collection: Vec<String> =
                    vec![format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLY_ID, id.get_id()).to_string()];

                let container_metric_scope: Vec<String> =
                    vec!["container_cpu_usage_seconds_total".to_string()];

                let scope = CollectorScope {
                    metric_names: container_metric_scope,
                    labels: label_collection,
                    last_x_minutes: METRIC_DEFAULT_LAST_X_MINUTE.to_string(),
                    avg_by_name: "rioos_assembly_id".to_string(),
                };
                Ok(Collector::new(prom, scope).metric_by_avg_for_containers("cpu")?)
            }
            _ => Ok(BTreeMap::new()),
        }
    }
    /// Expands the assembly by sticking in Spec
    ///         1. AssemblyFactory (parent information)
    ///         2. endpoints for this assembly.
    ///         3. volumes
    ///         4. metrics
    fn collect_spec(
        &self,
        row: &postgres::rows::Row,
        how_to: PullFromCache,
    ) -> Result<deploy::Assembly> {
        let mut assembly = row_to_assembly(&row)?;
        self.expander.with_factory(&mut assembly, how_to);
        self.expander.with_endpoints(&mut assembly, how_to);
        self.expander.with_volumes(&mut assembly, how_to);
        self.expander.with_metrics(&mut assembly, PULL_INVALDATED);
        Ok(assembly)
    }
}

/// A private convertor of postgres Row to the required structure.
/// In this case Assembly.
fn row_to_assembly(row: &postgres::rows::Row) -> Result<deploy::Assembly> {
    let mut assembly = deploy::Assembly::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    assembly.set_id(id.to_string());
    assembly.set_selector(row.get("selector"));
    assembly.set_status(serde_json::from_value(row.get("status")).unwrap());
    assembly.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    assembly.set_created_at(created_at.to_rfc3339());
    Ok(assembly)
}
