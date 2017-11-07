// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::plansrv;
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct PlanDS;

impl PlanDS {
    pub fn plan_create(datastore: &DataStoreConn, plan: &plansrv::Plan) -> Result<Option<plansrv::Plan>> {
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
