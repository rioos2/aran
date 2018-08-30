// Copyright 2018 The Rio Advancement Inc

use super::super::{LicenseOutput, LicenseOutputList};
use chrono::prelude::*;
use db::data_store::DataStoreConn;
use error::{Error, Result};

use postgres;
use protocol::api::base::{IdGet, MetaFields, WhoAmITypeMeta};
use protocol::api::licenses::Licenses;
use protocol::api::schema::type_meta_url;
use protocol::cache::{PULL_DIRECTLY, PULL_INVALDATED};
use protocol::cache::InMemoryExpander;
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
            "SELECT * FROM insert_or_update_license_v1 ($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(serde_json::to_value(license.object_meta()).unwrap()),
                &(serde_json::to_value(license.type_meta()).unwrap()),
                &(license.get_status() as String),
                &(license.get_expired() as String),
                &(serde_json::to_value(license.get_activation()).unwrap()),
                &(license.get_activation_completed() as bool),
                &(license.get_provider_name() as String),
                &(license.get_error() as String),
                &(license.get_product() as String),
            ],
        ).map_err(Error::LicenseCreate)?;
        if rows.len() > 0 {
            let mut licenses_create = row_to_licenses(&rows.get(0))?;
            self.expander.with_license(
                &mut licenses_create,
                PULL_INVALDATED,
            );
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

    pub fn update(&self, license: &Licenses) -> LicenseOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_license_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(license.get_provider_name() as String),
                &(serde_json::to_value(license.get_activation()).unwrap()),
                &(license.get_license_id() as String),
                &(license.get_password() as String),
                &(license.get_expired() as String),
                &(license.get_status() as String),
                &(license.get_error() as String),
            ],
        ).map_err(Error::LicenseUpdate)?;

        if rows.len() > 0 {
            let license = row_to_licenses(&rows.get(0))?;
            return Ok(Some(license));
        }
        Ok(None)
    }


    pub fn update_activation(&self, license: &Licenses) -> LicenseOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_activation_complete_v1($1,$2)",
            &[
                &(license.get_id().parse::<i64>().unwrap()),
                &(license.get_activation_completed() as bool),
            ],
        ).map_err(Error::LicenseUpdate)?;

        if rows.len() > 0 {
            let license_data = row_to_licenses(&rows.get(0))?;
            return Ok(Some(license_data));
        }
        Ok(None)
    }

    pub fn persist_error(&self, license: &Licenses) -> LicenseOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_error_v1($1,$2)",
            &[
                &(license.get_provider_name() as String),
                &(license.get_error() as String),
            ],
        ).map_err(Error::LicenseUpdate)?;

        if rows.len() > 0 {
            let license_data = row_to_licenses(&rows.get(0))?;
            return Ok(Some(license_data));
        }
        Ok(None)
    }


    pub fn update_status(&self, license: &Licenses) -> LicenseOutput {
        let conn = self.db.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_status_v1($1,$2,$3)",
            &[
                &(license.get_provider_name() as String),
                &(license.get_status() as String),
                &(license.get_expired() as String),
            ],
        ).map_err(Error::LicenseUpdate)?;

        if rows.len() > 0 {
            let license_data = row_to_licenses(&rows.get(0))?;
            return Ok(Some(license_data));
        }
        Ok(None)
    }

    //This is a fascade method to get_by_name.
    pub fn show(&self, name: IdGet) -> Licenses {
        let mut license = Licenses::new();

        let m = license.mut_meta(license.object_meta(), name.get_id(), license.get_account());

        let jackie = license.who_am_i();

        license.set_meta(type_meta_url(jackie), m);

        self.expander.with_license(&mut license, PULL_DIRECTLY);
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
    let activation_completed: bool = row.get("user_activation");

    licenses.set_id(id.to_string() as String);
    licenses.set_status(row.get("status"));
    licenses.set_expired(row.get("expired"));
    licenses.set_error(row.get("error"));
    licenses.set_product(row.get("product"));
    licenses.set_activation_completed(activation_completed);
    licenses.set_activation(serde_json::from_value(row.get("activation")).unwrap());
    licenses.set_created_at(created_at.to_rfc3339());

    Ok(licenses)
}
