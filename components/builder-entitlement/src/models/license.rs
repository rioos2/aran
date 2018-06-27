// Copyright 2018 The Rio Advancement Inc

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::licenses::Licenses;
use protocol::api::base::IdGet;
use protocol::cache::PULL_DIRECTLY;
use protocol::cache::InMemoryExpander;

use postgres;
use db::data_store::DataStoreConn;
use super::super::LicenseOutput;

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

    pub fn license_create_or_update(&self, license: &Licenses) -> LicenseOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_or_update_license_v1 ($1,$2)",
            &[
                &(license.get_name() as String),
                &(license.get_status() as String),
            ],
        ).map_err(Error::LicenseCreate)?;
        if rows.len() > 0 {
            let licenses_create = row_to_licenses(&rows.get(0))?;
            return Ok(Some(licenses_create));
        }
        Ok(None)
    }   

    pub fn license_show_by_name(&self, get_license: &IdGet) -> LicenseOutput {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_license_v1($1)",
            &[&(get_license.get_id() as String)],
        ).map_err(Error::LicenseGet)?;

        if rows.len() > 0 {
            let license = row_to_licenses(&rows.get(0))?;
            return Ok(Some(license));
        }
        Ok(None)
    }

    //This is a fascade method to get_by_name.
    pub fn get_by_name_fascade(&self, name: IdGet) -> Licenses {
        let mut license = Licenses::new();
        license.set_name(name.get_id());
        self.expander
            .with_license(&mut license, PULL_DIRECTLY);
        license
    }

}

fn row_to_licenses(row: &postgres::rows::Row) -> Result<Licenses> {
    let mut licenses = Licenses::new();

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let status: String = row.get("status");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    let updated_at = row.get::<&str, DateTime<Utc>>("updated_at");

    licenses.set_id(id.to_string() as String);
    licenses.set_name(name as String);
    licenses.set_status(status as String);
    licenses.set_created_at(created_at.to_rfc3339());
    licenses.set_updated_at(updated_at.to_rfc3339());

    Ok(licenses)
}
