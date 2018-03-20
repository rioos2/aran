// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the MarketPlace [marketplace].
use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::marketplace;
use protocol::api::base::{IdGet, MetaFields};
use postgres;
use db::data_store::DataStoreConn;

use serde_json;

use super::{MarketPlaceOutputList, MarketPlaceOutput};

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore { db: db }
    }
    pub fn create(&self, marketplace: &marketplace::MarketPlace) -> MarketPlaceOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_marketplace_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
            &[
                &(serde_json::to_value(marketplace.type_meta()).unwrap()),
                &(serde_json::to_value(marketplace.object_meta()).unwrap()),
                &(marketplace.get_category() as String),
                &(marketplace.get_version() as String),
                &(serde_json::to_value(marketplace.get_characteristics()).unwrap()),
                &(marketplace.get_icon() as String),
                &(marketplace.get_description() as String),
                &(serde_json::to_value(marketplace.get_ports()).unwrap()),
                &(serde_json::to_value(marketplace.get_envs()).unwrap()),
                &(serde_json::to_value(marketplace.get_lifecycle()).unwrap()),
                &(serde_json::to_value(marketplace.get_status()).unwrap()),
                &(serde_json::to_value(marketplace.get_metadata()).unwrap()),
            ],
        ).map_err(Error::MarketPlaceCreate)?;

        if rows.len() > 0 {
            return Ok(Some(self.row_to_marketplace(&rows.get(0))?));
        }
        Ok(None)
    }

    pub fn show(&self, get_marketplace: &IdGet) -> MarketPlaceOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_marketplace_v1($1)",
            &[&(get_marketplace.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::MarketPlaceGet)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_marketplace(&row)?));
            }
        }
        Ok(None)
    }

    pub fn list_blank(&self) -> MarketPlaceOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_marketplaces_v1()", &[])
            .map_err(Error::MarketPlaceGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(self.row_to_marketplace(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    fn row_to_marketplace(&self, row: &postgres::rows::Row) -> Result<marketplace::MarketPlace> {
        let mut marketplace = marketplace::MarketPlace::with(
            serde_json::from_value(row.get("type_meta")).unwrap(),
            serde_json::from_value(row.get("object_meta")).unwrap(),
        );
        let id: i64 = row.get("id");
        let created_at = row.get::<&str, DateTime<Utc>>("created_at");

        marketplace.set_id(id.to_string() as String);
        marketplace.set_status(serde_json::from_value(row.get("status")).unwrap());
        marketplace.set_category(row.get("category"));
        marketplace.set_version(row.get("version"));
        marketplace.set_characteristics(serde_json::from_value(row.get("characteristics")).unwrap());
        marketplace.set_icon(row.get("icon"));
        marketplace.set_description(row.get("description"));
        marketplace.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
        marketplace.set_ports(serde_json::from_value(row.get("ports")).unwrap());
        marketplace.set_envs(serde_json::from_value(row.get("envs")).unwrap());
        marketplace.set_lifecycle(serde_json::from_value(row.get("lifecycle")).unwrap());
        marketplace.set_created_at(created_at.to_rfc3339());

        Ok(marketplace)
    }
}
