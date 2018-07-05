// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use super::super::{SenseiOutput, SenseiOutputList};
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::{Error, Result};
use postgres;
use protocol::api::base::{IdGet, MetaFields, WhoAmITypeMeta};
use protocol::api::schema::type_meta_url;
use protocol::api::senseis::Senseis;
use serde_json;
use std::collections::BTreeMap;

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore { db: db }
    }

    pub fn create(&self, sensei_create: &Senseis) -> SenseiOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_or_update_senseis_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(sensei_create.get_node_ip() as String),
                &(serde_json::to_value(sensei_create.get_spec()).unwrap()),
                &(serde_json::to_value(sensei_create.get_status()).unwrap()),
                &(serde_json::to_value(sensei_create.object_meta()).unwrap()),
                &(serde_json::to_value(sensei_create.type_meta()).unwrap()),
                &(serde_json::to_value(sensei_create.get_metadata()).unwrap()),
            ],
        ).map_err(Error::SenseiCreate)?;

        if rows.len() > 0 {
            let sensei = row_to_sensei(&rows.get(0))?;

            return Ok(Some(sensei));
        }
        Ok(None)
    }

    pub fn show(&self, sensei_get: &IdGet) -> SenseiOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * from get_sensei_v1($1)", &[&(sensei_get.get_id().parse::<i64>().unwrap())])
            .map_err(Error::SenseiGet)?;

        if rows.len() > 0 {
            let sensei = row_to_sensei(&rows.get(0))?;
            return Ok(Some(sensei));
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> SenseiOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_senseis_v1()", &[]).map_err(Error::SenseiGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_sensei(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_sensei(row: &postgres::rows::Row) -> Result<Senseis> {
    let mut sensei = Senseis::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    sensei.set_id(id.to_string());
    sensei.set_node_ip(row.get("node_ip"));
    sensei.set_spec(serde_json::from_value(row.get("spec")).unwrap());
    sensei.set_status(serde_json::from_value(row.get("status")).unwrap());
    sensei.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    sensei.set_created_at(created_at.to_rfc3339());
    Ok(sensei)
}
