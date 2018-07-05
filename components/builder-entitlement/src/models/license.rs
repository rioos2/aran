// Copyright 2018 The Rio Advancement Inc

use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::licenses::Licenses;
use protocol::api::base::{IdGet,MetaFields};
use protocol::cache::PULL_DIRECTLY;
use protocol::cache::InMemoryExpander;

use postgres;
use db::data_store::DataStoreConn;
use super::super::{LicenseOutput,LicenseOutputList};
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

    pub fn create_or_update(&self, license: &Licenses) -> LicenseOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_or_update_license_v1 ($1,$2,$3,$4,$5,$6)",
            &[
                &(serde_json::to_value(license.object_meta()).unwrap()),
                &(serde_json::to_value(license.type_meta()).unwrap()),
                &(license.get_status() as String),
                &(license.get_product() as String),
                &(license.get_activation_code() as String),
                &(license.get_expired() as String),
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

    pub fn list_blank(&self) -> LicenseOutputList {
        let conn = self.db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_license_list_by_v1()", &[])
            .map_err(Error::LicenseGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_licenses(&row)?)
            }
            return Ok(Some(response));
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

    let mut licenses = Licenses::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    licenses.set_id(id.to_string() as String);
    licenses.set_status(row.get("status"));
    licenses.set_product(row.get("product"));
    licenses.set_expired(row.get("expired"));
    licenses.set_created_at(created_at.to_rfc3339());

    Ok(licenses)
}
