// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].
use chrono::prelude::*;

use error::{Error, Result};
use protocol::api::base::{IdGet, MetaFields};
use protocol::api::ingress::{StatusUpdate, Ingress};

use db::data_store::DataStoreConn;
use postgres;
use serde_json;

use super::super::{IngressOutput,IngressOutputList};

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore { db: db}
    }

    pub fn create(&self, ingress_create: &Ingress) -> IngressOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_ingress_v1($1,$2,$3,$4)",
            &[
                &(serde_json::to_value(ingress_create.get_status()).unwrap()),
                &(serde_json::to_value(ingress_create.object_meta()).unwrap()),
                &(serde_json::to_value(ingress_create.type_meta()).unwrap()),
                &(serde_json::to_value(ingress_create.get_spec()).unwrap()),
            ],
        ).map_err(Error::IngressCreate)?;

        if rows.len() > 0 {
            let ingress = row_to_ingress(&rows.get(0))?;
            return Ok(Some(ingress));
        }
        Ok(None)
    }

    pub fn show(&self, get_ingress: &IdGet) -> IngressOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_ingress_v1($1)", &[&(get_ingress.get_id().parse::<i64>().unwrap())])
            .map_err(Error::IngressGet)?;
        if rows.len() > 0 {
            let ingress = row_to_ingress(&rows.get(0))?;
            return Ok(Some(ingress));
        }
        Ok(None)
    }

    pub fn status_update(&self, ingress: &StatusUpdate) -> IngressOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_ingress_status_v1($1, $2)",
            &[
                &(ingress.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(ingress.get_status()).unwrap()),
            ],
        ).map_err(Error::IngressUpdate)?;
        if rows.len() > 0 {
            let ingress = row_to_ingress(&rows.get(0))?;
            return Ok(Some(ingress));
        }
        Ok(None)
    }

    pub fn show_by_assembly_factory(&self, ingress_get: &IdGet) -> IngressOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_ingress_by_assembly_factory_v1($1)", &[&(ingress_get.get_id() as String)])
            .map_err(Error::IngressGet)?;

            if rows.len() > 0 {
                let ingress = row_to_ingress(&rows.get(0))?;
                return Ok(Some(ingress));
            }
        Ok(None)
    }

    pub fn update(&self, ingress: &Ingress) -> IngressOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_ingress_v1($1,$2,$3,$4)",
            &[
                &(ingress.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(ingress.get_status()).unwrap()),
                &(serde_json::to_value(ingress.object_meta()).unwrap()),
                &(serde_json::to_value(ingress.get_spec()).unwrap()),
            ],
        ).map_err(Error::IngressUpdate)?;

        if rows.len() > 0 {
            let ingress = row_to_ingress(&rows.get(0))?;
            return Ok(Some(ingress));
        }
        Ok(None)
    }
    pub fn list_blank(&self) -> IngressOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_ingresses_v1()", &[]).map_err(Error::IngressGet)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_ingress(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_ingress(row: &postgres::rows::Row) -> Result<Ingress> {
    let mut ingress = Ingress::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    ingress.set_id(id.to_string());
    ingress.set_created_at(created_at.to_rfc3339());
    ingress.set_status(serde_json::from_value(row.get("status")).unwrap());
    ingress.set_spec(serde_json::from_value(row.get("spec")).unwrap());

    Ok(ingress)
}
