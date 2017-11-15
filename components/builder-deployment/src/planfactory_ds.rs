// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::plansrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct PlanFactoryDS;

impl PlanFactoryDS {
    pub fn create(datastore: &DataStoreConn, plan: &plansrv::Plan) -> Result<Option<plansrv::Plan>> {
        let conn = datastore.pool.get_shard(0)?;
        let data: Vec<String> = plan.get_services()
            .into_iter()
            .map(|plan| {
                let d = serde_json::to_string(plan).unwrap();
                d
            })
            .collect();
        let rows = &conn.query(
            "SELECT * FROM insert_plan_factory_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(plan.get_group_name() as String),
                &(plan.get_description() as String),
                &(plan.get_tags() as Vec<String>),
                &(plan.get_url() as String),
                &(plan.get_origin() as String),
                &(plan.get_artifacts() as Vec<String>),
                &(data as Vec<String>),
            ],
        ).map_err(Error::PlanCreate)?;
        if rows.len() > 0 {
            let plan = row_to_plan(&rows.get(0))?;
            return Ok(Some(plan));
        }
        Ok(None)

    }

    pub fn show(datastore: &DataStoreConn, plan_url: String) -> Result<Option<plansrv::Plan>> {
        let url = plan_url.to_string();
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_plan_v1($1)", &[&url])
            .map_err(Error::PlanGet)?;
        if rows.len() > 0 {
            for row in rows {
                let plan = row_to_plan(&row)?;
                return Ok(Some(plan));
            }
        }
        Ok(None)
    }

    pub fn list(datastore: &DataStoreConn) -> Result<Option<plansrv::PlanGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_plans_v1()", &[]).map_err(
            Error::PlanGetResponse,
        )?;

        let mut response = plansrv::PlanGetResponse::new();

        let mut plan_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                plan_collection.push(row_to_plan(&row)?)
            }
            response.set_plan_collection(plan_collection);
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_plan(row: &postgres::rows::Row) -> Result<plansrv::Plan> {
    let mut plan = plansrv::Plan::new();
    let id: i64 = row.get("id");
    let services: Vec<String> = row.get("services");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    plan.set_id(id.to_string() as String);
    plan.set_group_name(row.get("group_name"));
    plan.set_url(row.get("url"));
    plan.set_description(row.get("description"));
    plan.set_tags(row.get("tags"));
    plan.set_origin(row.get("origin"));
    plan.set_artifacts(row.get("artifacts"));
    let mut service_collection = Vec::new();
    for data in services {
        let object_service: plansrv::Service = serde_json::from_str(&data).unwrap();
        service_collection.push(object_service);
    }
    plan.set_services(service_collection);
    plan.set_created_at(created_at.to_rfc3339());

    Ok(plan)
}
