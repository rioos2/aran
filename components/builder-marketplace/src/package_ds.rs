// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Packages [package].
use chrono::prelude::*;

use error::{Result, Error};
use protocol::api::package;
use protocol::api::base::{IdGet, MetaFields};

use postgres;
use db::data_store::DataStoreConn;
use serde_json;

use super::PackageOutput;

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore { db: db }
    }
    pub fn create(&self, package: &package::Package) -> PackageOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_package_v1($1,$2,$3,$4)",
            &[
                &(serde_json::to_value(package.type_meta()).unwrap()),
                &(serde_json::to_value(package.object_meta()).unwrap()),
                &(package.get_version_number()),
                &(package.get_extension()),
            ],
        ).map_err(Error::PackageCreate)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_package(&row)?));
            }
        }

        Ok(None)
    }

    pub fn show(&self, get_package: &IdGet) -> PackageOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_package_v1($1)",
            &[&(get_package.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::PackageGet)?;
        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(self.row_to_package(&row)?));
            }
        }
        Ok(None)
    }

    fn row_to_package(&self, row: &postgres::rows::Row) -> Result<package::Package> {
        let mut package_create = package::Package::with(
            serde_json::from_value(row.get("type_meta")).unwrap(),
            serde_json::from_value(row.get("object_meta")).unwrap(),
        );

        let id: i64 = row.get("id");
        let created_at = row.get::<&str, DateTime<Utc>>("created_at");
        package_create.set_id(id.to_string());
        package_create.set_created_at(created_at.to_rfc3339());
        package_create.set_version_number(row.get("version_number"));
        package_create.set_extension(row.get("extension"));

        Ok(package_create)
    }
}
