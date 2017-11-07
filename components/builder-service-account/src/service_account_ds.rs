// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{servicesrv, asmsrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use rio_net::util::errors::*;
use protocol::constants::*;


pub struct ServiceAccountDS;

impl ServiceAccountDS {
    pub fn secret_create(datastore: &DataStoreConn, secret_create: &servicesrv::Secret) -> Result<Option<servicesrv::Secret>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_secret_v1($1,$2,$3,$4,$5)",
            &[
                &(secret_create.get_secret_type() as String),
                &(secret_create.get_object_meta().get_origin() as String),
                &(serde_json::to_string(secret_create.get_data()).unwrap()),
                &(serde_json::to_string(secret_create.get_object_meta()).unwrap()),
                &(serde_json::to_string(secret_create.get_type_meta()).unwrap()),
            ],
        ).map_err(Error::SecretCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let secret = row_to_secret(&rows.get(0));
                return Ok(Some(secret));
            }
        }
        Ok(None)

    }
    pub fn secret_show(datastore: &DataStoreConn, get_secret: &asmsrv::IdGet) -> Result<Option<servicesrv::Secret>> {
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

    pub fn secret_show_by_origin(datastore: &DataStoreConn, get_secret: &asmsrv::IdGet) -> Result<Option<servicesrv::SecretGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_secrets_by_origin_v1($1)",
            &[&(get_secret.get_id() as String)],
        ).map_err(Error::SecretGetResponse)?;

        let mut response = servicesrv::SecretGetResponse::new();

        let mut secret_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                secret_collection.push(row_to_secret(&row))
            }
            response.set_secret_collection(secret_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn secret_list(datastore: &DataStoreConn) -> Result<Option<servicesrv::SecretGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_secrets_v1()", &[]).map_err(
            Error::SecretGetResponse,
        )?;

        let mut response = servicesrv::SecretGetResponse::new();

        let mut secret_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                secret_collection.push(row_to_secret(&row))
            }
            response.set_secret_collection(secret_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn service_account_create(datastore: &DataStoreConn, service_create: &servicesrv::ServiceAccount) -> Result<Option<servicesrv::ServiceAccount>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_service_account_v1($1,$2,$3,$4,$5)",
            &[
                &(service_create.get_object_meta().get_origin() as String),
                &(service_create.get_object_meta().get_name() as String),
                &(serde_json::to_string(service_create.get_secrets()).unwrap()),
                &(serde_json::to_string(service_create.get_object_meta()).unwrap()),
                &(serde_json::to_string(service_create.get_type_meta()).unwrap()),
            ],
        ).map_err(Error::ServiceAccountCreate)?;
        if rows.len() > 0 {
            let service_account = row_to_service_account(&rows.get(0))?;
            return Ok(Some(service_account));
        }
        Ok(None)
    }

    pub fn service_account_show(datastore: &DataStoreConn, get_service: &asmsrv::IdGet) -> Result<Option<servicesrv::ServiceAccount>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_service_account_by_origin_v1($1,$2)",
            &[&get_service.get_id(), &get_service.get_name()],
        ).map_err(Error::ServiceAccountGet)?;
        if rows.len() > 0 {
            for row in rows {
                let serv = row_to_service_account(&row)?;
                return Ok(Some(serv));
            }
        }
        Ok(None)
    }

    pub fn service_account_list(datastore: &DataStoreConn) -> Result<Option<servicesrv::ServiceAccountGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_service_account_v1()", &[])
            .map_err(Error::ServiceAccountGetResponse)?;

        let mut response = servicesrv::ServiceAccountGetResponse::new();

        let mut service_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                service_collection.push(row_to_service_account(&row)?)
            }
            response.set_service_collection(service_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn endpoints_create(datastore: &DataStoreConn, endpoints_create: &servicesrv::EndPoints) -> Result<Option<servicesrv::EndPoints>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_endpoints_v1($1,$2,$3,$4,$5)",
            &[
                &(endpoints_create.get_target_ref().parse::<i64>().unwrap()),
                &(endpoints_create.get_object_meta().get_origin() as String),
                &(serde_json::to_string(endpoints_create.get_subsets()).unwrap()),
                &(serde_json::to_string(endpoints_create.get_object_meta()).unwrap()),
                &(serde_json::to_string(endpoints_create.get_type_meta()).unwrap()),
            ],
        ).map_err(Error::EndPointsCreate)?;
        if rows.len() > 0 {
            let end = row_to_endpoints(&rows.get(0))?;
            return Ok(Some(end));
        }
        Ok(None)
    }

    pub fn endpoints_list(datastore: &DataStoreConn) -> Result<Option<servicesrv::EndpointsGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_endpoints_v1()", &[])
            .map_err(Error::EndpointsGetResponse)?;

        let mut response = servicesrv::EndpointsGetResponse::new();

        let mut end_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                end_collection.push(row_to_endpoints(&row)?)
            }
            response.set_end_collection(end_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn endpoints_show(datastore: &DataStoreConn, endpoints_get: &asmsrv::IdGet) -> Result<Option<servicesrv::EndPoints>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_endpoint_v1($1)",
            &[&(endpoints_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::EndPointsGet)?;
        if rows.len() > 0 {
            for row in rows {
                let end = row_to_endpoints(&row)?;
                return Ok(Some(end));
            }
        }
        Ok(None)
    }

    pub fn endpoints_list_by_origin(datastore: &DataStoreConn, endpoints_get: &asmsrv::IdGet) -> Result<Option<servicesrv::EndpointsGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_endpoints_by_origin_v1($1)",
            &[&(endpoints_get.get_id() as String)],
        ).map_err(Error::EndPointsGet)?;

        let mut response = servicesrv::EndpointsGetResponse::new();

        let mut end_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                end_collection.push(row_to_endpoints(&row)?)
            }
            response.set_end_collection(end_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn endpoints_get_by_assembly(datastore: &DataStoreConn, endpoints_get: &asmsrv::IdGet) -> Result<Option<servicesrv::EndPoints>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_endpoints_by_assebmly_v1($1)",
            &[&(endpoints_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::EndPointsGet)?;

        let mut response = servicesrv::EndpointsGetResponse::new();

        if rows.len() > 0 {
            for row in rows {
                let response = row_to_endpoints(&row)?;
                return Ok(Some(response));
            }

        }
        Ok(None)
    }
    pub fn services_create(datastore: &DataStoreConn, services_create: &servicesrv::Services) -> Result<Option<servicesrv::Services>> {
        let conn = datastore.pool.get_shard(0)?;
        let asmid = services_create.get_spec().get_selector().get(
            &RIO_ASM_FAC_ID
                .to_string(),
        );
        let rows = &conn.query(
            "SELECT * FROM insert_services_v1($1,$2,$3,$4,$5,$6)",
            &[
                &(services_create.get_object_meta().get_origin() as String),
                &(asmid.unwrap().parse::<i64>().unwrap()),
                &(serde_json::to_string(services_create.get_spec()).unwrap()),
                &(serde_json::to_string(services_create.get_status()).unwrap()),
                &(serde_json::to_string(services_create.get_object_meta()).unwrap()),
                &(serde_json::to_string(services_create.get_type_meta()).unwrap()),
            ],
        ).map_err(Error::ServicesCreate)?;
        if rows.len() > 0 {
            for row in rows {
                let end = row_to_services(&rows.get(0));
                return Ok(Some(end));
            }
        }
        Ok(None)

    }
    pub fn services_show(datastore: &DataStoreConn, services_get: &asmsrv::IdGet) -> Result<Option<servicesrv::Services>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_services_v1($1)",
            &[&(services_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ServicesGet)?;
        if rows.len() > 0 {
            for row in rows {
                let end = row_to_services(&row);
                return Ok(Some(end));
            }
        }
        Ok(None)
    }
    pub fn services_list(datastore: &DataStoreConn) -> Result<Option<servicesrv::ServicesGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_services_list_v1()", &[])
            .map_err(Error::ServicesGetResponse)?;

        let mut response = servicesrv::ServicesGetResponse::new();

        let mut services_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                services_collection.push(row_to_services(&row))
            }
            response.set_services_collection(services_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn services_list_by_origin(datastore: &DataStoreConn, services_get: &asmsrv::IdGet) -> Result<Option<servicesrv::ServicesGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_services_by_origin_v1($1)",
            &[&(services_get.get_id() as String)],
        ).map_err(Error::ServicesGetResponse)?;

        let mut response = servicesrv::ServicesGetResponse::new();

        let mut services_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                services_collection.push(row_to_services(&row))
            }
            response.set_services_collection(services_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn services_list_by_assembly(datastore: &DataStoreConn, services_get: &asmsrv::IdGet) -> Result<Option<servicesrv::ServicesGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_services_by_assebmly_v1($1)",
            &[&(services_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::ServicesGetResponse)?;

        let mut response = servicesrv::ServicesGetResponse::new();

        let mut services_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                services_collection.push(row_to_services(&row))
            }
            response.set_services_collection(services_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
}


fn row_to_secret(row: &postgres::rows::Row) -> servicesrv::Secret {
    let mut secret = servicesrv::Secret::new();
    let id: i64 = row.get("id");
    let secret_type = row.get("secret_type");
    let data: String = row.get("data");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    secret.set_id(id.to_string());
    secret.set_data(serde_json::from_str(&data).unwrap());
    secret.set_object_meta(serde_json::from_str(&object_meta).unwrap());
    secret.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    secret.set_secret_type(secret_type);
    secret.set_created_at(created_at.to_rfc3339());

    secret
}

fn row_to_endpoints(row: &postgres::rows::Row) -> Result<servicesrv::EndPoints> {
    let mut endpoints = servicesrv::EndPoints::new();
    let id: i64 = row.get("id");
    let target_ref: i64 = row.get("target_ref");
    let subsets: String = row.get("subsets");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    endpoints.set_id(id.to_string());
    endpoints.set_target_ref(target_ref.to_string());
    endpoints.set_subsets(serde_json::from_str(&subsets).unwrap());
    endpoints.set_object_meta(serde_json::from_str(&object_meta).unwrap());
    endpoints.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    endpoints.set_created_at(created_at.to_rfc3339());

    Ok(endpoints)
}
fn row_to_services(row: &postgres::rows::Row) -> servicesrv::Services {
    let mut services = servicesrv::Services::new();
    let id: i64 = row.get("id");
    let spec: String = row.get("spec");
    let status: String = row.get("status");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    services.set_id(id.to_string());
    services.set_spec(serde_json::from_str(&spec).unwrap());
    services.set_status(serde_json::from_str(&status).unwrap());
    services.set_object_meta(serde_json::from_str(&object_meta).unwrap());
    services.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    services.set_created_at(created_at.to_rfc3339());

    services
}


fn row_to_service_account(row: &postgres::rows::Row) -> Result<servicesrv::ServiceAccount> {
    let mut service_account = servicesrv::ServiceAccount::new();
    let id: i64 = row.get("id");
    let secrets: String = row.get("secrets");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    service_account.set_id(id.to_string());
    service_account.set_secrets(serde_json::from_str(&secrets).unwrap());
    service_account.set_object_meta(serde_json::from_str(&object_meta).unwrap());
    service_account.set_type_meta(serde_json::from_str(&type_meta).unwrap());
    service_account.set_created_at(created_at.to_rfc3339());

    Ok(service_account)
}
