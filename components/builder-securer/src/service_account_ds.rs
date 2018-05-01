// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::api::{base, service_account};
use protocol::api::base::MetaFields;
use protocol::api::base::IdGet;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

use super::{ServiceAccountOutput, ServiceAccountOutputList};

pub struct ServiceAccountDS;

impl ServiceAccountDS {
    pub fn create(datastore: &DataStoreConn, service_create: &service_account::ServiceAccount) -> ServiceAccountOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_service_account_v1($1,$2,$3,$4,$5)",
            &[
                &(serde_json::to_value(&service_create.get_secrets()).unwrap()),
                &(serde_json::to_value(&service_create.object_meta()).unwrap()),
                &(serde_json::to_value(&service_create.type_meta()).unwrap()),
                &(serde_json::to_value(&service_create.get_metadata()).unwrap()),
                &(service_create.get_roles() as Vec<String>),
            ],
        ).map_err(Error::ServiceAccountCreate)?;
        if rows.len() > 0 {
            let service_account = row_to_service_account(&rows.get(0))?;
            return Ok(Some(service_account));
        }
        Ok(None)
    }

    pub fn update(datastore: &DataStoreConn, serviceaccount: &service_account::ServiceAccount) -> ServiceAccountOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_service_account_v1($1,$2,$3)",
            &[
                &(serviceaccount.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(&serviceaccount.get_secrets()).unwrap()),
                &(serde_json::to_value(&serviceaccount.object_meta()).unwrap()),
            ],
        ).map_err(Error::ServiceAccountUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let serviceaccount = row_to_service_account(&row)?;
                return Ok(Some(serviceaccount));
            }
        }
        Ok(None)
    }

    pub fn show(datastore: &DataStoreConn, get_service: &base::IdGet) -> ServiceAccountOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_serviceaccount_by_name_v1($1)",
            &[&get_service.get_id()],
        ).map_err(Error::ServiceAccountGet)?;

        if rows.len() > 0 {
            for row in rows {
                let serv = row_to_service_account(&row)?;
                return Ok(Some(serv));
            }
        }

        Ok(None)
    }

    pub fn show_by_id(datastore: &DataStoreConn, get_service: &IdGet) -> ServiceAccountOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_service_account_by_id_v1($1)",
            &[&(get_service.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ServiceAccountGet)?;
        if rows.len() > 0 {
            let serv = row_to_service_account(&rows.get(0))?;
            return Ok(Some(serv));
        }
        Ok(None)
    }

    pub fn list_blank(datastore: &DataStoreConn) -> ServiceAccountOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_service_account_v1()", &[])
            .map_err(Error::ServiceAccountGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_service_account(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_service_account(row: &postgres::rows::Row) -> Result<service_account::ServiceAccount> {
    let mut service_account = service_account::ServiceAccount::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    service_account.set_id(id.to_string());
    service_account.set_secrets(serde_json::from_value(row.get("secrets")).unwrap());
    service_account.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    service_account.set_roles(row.get("roles"));
    service_account.set_created_at(created_at.to_rfc3339());

    Ok(service_account)
}
