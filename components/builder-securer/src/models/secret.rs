// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::Error;
use protocol::api::base::MetaFields;
use protocol::api::{base, secret};

use db::data_store::DataStoreConn;
use postgres;
use serde_json;
use protocol::cache::InMemoryExpander;

use super::super::{SecretOutput, SecretOutputList};

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
    pub fn create(datastore: &DataStoreConn, secret_create: &secret::Secret) -> SecretOutput {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_secret_v1($1,$2,$3,$4,$5)",
            &[
                &(secret_create.get_secret_type() as String),
                &(serde_json::to_value(secret_create.get_data()).unwrap()),
                &(serde_json::to_value(secret_create.get_metadata()).unwrap()),
                &(serde_json::to_value(&secret_create.object_meta()).unwrap()),
                &(serde_json::to_value(&secret_create.type_meta()).unwrap()),
            ],
        ).map_err(Error::SecretCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let secret = row_to_secret(&row);
                return Ok(Some(secret));
            }
        }
        Ok(None)
    }
    pub fn show(datastore: &DataStoreConn, get_secret: &base::IdGet) -> SecretOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_secret_v1($1)",
            &[&(get_secret.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::SecretGet)?;
        if rows.len() > 0 {
            for row in rows {
                let secret = row_to_secret(&row);
                return Ok(Some(secret));
            }
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn, get_secret: &base::IdGet) -> SecretOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_secrets_by_account_v1($1)",
            &[&(get_secret.get_name() as String)],
        ).map_err(Error::SecretGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_secret(&row))
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn list_by_origin(datastore: &DataStoreConn, get_secret: &base::IdGet) -> SecretOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_secrets_by_origin_id_v1($1)",
            &[&(get_secret.get_id() as String)],
        ).map_err(Error::SecretGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_secret(&row))
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn update(datastore: &DataStoreConn, secret: &secret::Secret) -> SecretOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM update_secret_v1($1,$2,$3,$4,$5)",
            &[
                &(secret.get_id().parse::<i64>().unwrap()),
                &(secret.get_secret_type() as String),
                &(serde_json::to_value(secret.get_data()).unwrap()),
                &(serde_json::to_value(secret.get_metadata()).unwrap()),
                &(serde_json::to_value(&secret.object_meta()).unwrap()),
            ],
        ).map_err(Error::SecretUpdate)?;

        if rows.len() > 0 {
            for row in rows {
                let secret = row_to_secret(&row);
                return Ok(Some(secret));
            }
        }
        Ok(None)
    }

    pub fn list_blank(datastore: &DataStoreConn) -> SecretOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_secrets_v1()", &[])
            .map_err(Error::SecretGetResponse)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_secret(&row))
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn show_by_origin_and_name(
        datastore: &DataStoreConn,
        get_secret: &base::IdGet,
    ) -> SecretOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_secrets_by_origin_v1($1,$2)",
            &[&(get_secret.get_name()), &(get_secret.get_id())],
        ).map_err(Error::SecretGetResponse)?;
        if rows.len() > 0 {
            for row in rows {
                let secret = row_to_secret(&row);
                return Ok(Some(secret));
            }
        }
        Ok(None)
    }
}

fn row_to_secret(row: &postgres::rows::Row) -> secret::Secret {
    let mut secret = secret::Secret::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    secret.set_id(id.to_string());
    secret.set_data(serde_json::from_value(row.get("data")).unwrap());
    secret.set_metadata(serde_json::from_value(row.get("metadata")).unwrap());
    secret.set_secret_type(row.get("secret_type"));
    secret.set_created_at(created_at.to_rfc3339());

    secret
}
