// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use std::collections::BTreeMap;


pub struct ServiceAccountDS;

impl ServiceAccountDS {
    pub fn secret_create(datastore: &DataStoreConn, secret_create: &servicesrv::Secret) -> Result<Option<servicesrv::Secret>> {
        let conn = datastore.pool.get_shard(0)?;
        let data_str = serde_json::to_string(secret_create.get_data()).unwrap();
        let object_meta = serde_json::to_string(secret_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(secret_create.get_type_meta()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_secret_v1($1,$2,$3,$4,$5)",
            &[
                &(secret_create.get_secret_type() as String),
                &(secret_create.get_object_meta().get_origin() as String),
                &(data_str as String),
                &(object_meta as String),
                &(type_meta as String),
            ],
        ).map_err(Error::SecretCreate)?;
        let secret = row_to_secret(&rows.get(0))?;
        return Ok(Some(secret.clone()));
    }
    pub fn secret_show(datastore: &DataStoreConn, get_secret: &asmsrv::IdGet) -> Result<Option<servicesrv::Secret>> {
        let conn = datastore.pool.get_shard(0)?;
        let secret_id = get_secret.get_id().parse::<i64>().unwrap();
        let rows = &conn.query("SELECT * FROM get_secret_v1($1)", &[&secret_id])
            .map_err(Error::SecretGet)?;
        for row in rows {
            let secret = row_to_secret(&row)?;
            return Ok(Some(secret));
        }
        Ok(None)
    }

    pub fn secret_show_by_origin(datastore: &DataStoreConn, get_secret: &asmsrv::IdGet) -> Result<Option<servicesrv::SecretGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_secrets_by_origin_v1($1)",
            &[&(get_secret.get_id() as String)],
        ).map_err(Error::SecretGetResponse)?;

        let mut response = servicesrv::SecretGetResponse::new();

        let mut secret_collection = Vec::new();
        for row in rows {
            secret_collection.push(row_to_secret(&row)?)
        }
        response.set_secret_collection(
            secret_collection,
            "SecretList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }

    pub fn secret_list(datastore: &DataStoreConn) -> Result<Option<servicesrv::SecretGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_secrets_v1()", &[]).map_err(
            Error::SecretGetResponse,
        )?;

        let mut response = servicesrv::SecretGetResponse::new();

        let mut secret_collection = Vec::new();
        for row in rows {
            secret_collection.push(row_to_secret(&row)?)
        }
        response.set_secret_collection(
            secret_collection,
            "SecretList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }

    pub fn service_account_create(datastore: &DataStoreConn, service_create: &servicesrv::ServiceAccount) -> Result<Option<servicesrv::ServiceAccount>> {
        let conn = datastore.pool.get_shard(0)?;
        let secret_str = serde_json::to_string(service_create.get_secrets()).unwrap();
        let object_meta = serde_json::to_string(service_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(service_create.get_type_meta()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_service_account_v1($1,$2,$3,$4,$5)",
            &[
                &(service_create.get_object_meta().get_origin() as String),
                &(service_create.get_object_meta().get_name() as String),
                &(secret_str as String),
                &(object_meta as String),
                &(type_meta as String),
            ],
        ).map_err(Error::ServiceAccountCreate)?;
        let service_account = row_to_service_account(&rows.get(0))?;
        return Ok(Some(service_account.clone()));
    }

    pub fn service_account_show(datastore: &DataStoreConn, get_service: &asmsrv::IdGet) -> Result<Option<servicesrv::ServiceAccount>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_service_account_by_origin_v1($1,$2)",
            &[&get_service.get_id(), &get_service.get_name()],
        ).map_err(Error::ServiceAccountGet)?;

        for row in rows {
            let serv = row_to_service_account(&row)?;
            return Ok(Some(serv));
        }
        Ok(None)
    }

    pub fn service_account_list(datastore: &DataStoreConn) -> Result<Option<servicesrv::ServiceAccountGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_service_account_v1()", &[])
            .map_err(Error::ServiceAccountGetResponse)?;

        let mut response = servicesrv::ServiceAccountGetResponse::new();

        let mut service_collection = Vec::new();
        for row in rows {
            service_collection.push(row_to_service_account(&row)?)
        }
        response.set_service_collection(
            service_collection,
            "ServiceAccountList".to_string(),
            "v1".to_string(),
        );
        Ok(Some(response))
    }
}


fn row_to_secret(row: &postgres::rows::Row) -> Result<servicesrv::Secret> {
    let mut secret = servicesrv::Secret::new();
    let id: i64 = row.get("id");
    let secret_type = row.get("secret_type");
    let data: String = row.get("data");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    secret.set_id(id.to_string() as String);
    let data_obj: BTreeMap<String, String> = serde_json::from_str(&data).unwrap();
    secret.set_data(data_obj);
    let object_meta_obj: servicesrv::ObjectMetaData = serde_json::from_str(&object_meta).unwrap();
    secret.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    secret.set_type_meta(type_meta_obj);
    secret.set_secret_type(secret_type);
    secret.set_created_at(created_at.to_rfc3339());

    Ok(secret)
}


fn row_to_service_account(row: &postgres::rows::Row) -> Result<servicesrv::ServiceAccount> {
    let mut service_account = servicesrv::ServiceAccount::new();
    let id: i64 = row.get("id");
    let secrets: String = row.get("secrets");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    service_account.set_id(id.to_string() as String);
    let secret_obj: servicesrv::ObjectReference = serde_json::from_str(&secrets).unwrap();
    service_account.set_secrets(secret_obj);
    let object_meta_obj: servicesrv::ObjectMetaData = serde_json::from_str(&object_meta).unwrap();
    service_account.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    service_account.set_type_meta(type_meta_obj);
    service_account.set_created_at(created_at.to_rfc3339());

    Ok(service_account)
}
