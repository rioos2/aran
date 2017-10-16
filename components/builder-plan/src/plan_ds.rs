// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{plansrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;

pub struct PlanDS;

impl PlanDS {
    pub fn plan_create(datastore: &DataStoreConn, plan: &plansrv::Plan) -> Result<Option<plansrv::Plan>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_plan_factory_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(plan.get_group_name() as String),
                &(plan.get_description() as String),
                &(plan.get_tags() as Vec<String>),
                &(plan.get_url() as String),
                &(plan.get_origin() as String),
                &(plan.get_artifacts() as Vec<String>),
                &(plan.get_services()),
            ],
        ).map_err(Error::PlanCreate)?;

        let plan = row_to_plan(&rows.get(0))?;
        return Ok(Some(plan.clone()));
    }

}


pub fn row_to_plan(row: &postgres::rows::Row) -> Result<plansrv::Plan> {
    let mut plan = plansrv::Plan::new();
    let id: i64 = row.get("id");
    let name: String = row.get("group_name");
    let url: String = row.get("url");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let origin: String = row.get("origin");
    let artifacts: Vec<String> = row.get("artifacts");
    let services: String = row.get("services");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    plan.set_id(id.to_string() as String);
    plan.set_group_name(name as String);
    plan.set_url(url as String);
    plan.set_description(description as String);
    plan.set_tags(tags as Vec<String>);
    plan.set_origin(origin as String);
    plan.set_artifacts(artifacts as Vec<String>);
    plan.set_services(serde_json::from_str(&services).unwrap());
    // let mut service_collection = Vec::new();
    // for data in services {
    //     let object_service: plansrv::Service = serde_json::from_str(&data).unwrap();
    //     service_collection.push(object_service);
    // }
    // plan.set_services(service_collection);
    plan.set_created_at(created_at.to_rfc3339());

    Ok(plan)
}
