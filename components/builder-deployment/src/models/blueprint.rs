// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].
use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::blueprint;
use protocol::api::base::{IdGet, MetaFields};

use postgres;
use db::data_store::DataStoreConn;

use serde_json;

use super::super::{PlanOutputList, PlanOutput};

pub struct DataStore;

impl DataStore {
    pub fn create(db: &DataStoreConn, plan: &blueprint::Plan) -> PlanOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_plan_factory_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
            &[
                &(serde_json::to_value(plan.type_meta()).unwrap()),
                &(serde_json::to_value(plan.object_meta()).unwrap()),
                &(plan.get_category() as String),
                &(plan.get_version() as String),
                &(serde_json::to_value(plan.get_characteristics()).unwrap()),
                &(plan.get_icon() as String),
                &(plan.get_description() as String),
                &(serde_json::to_value(plan.get_ports()).unwrap()),
                &(serde_json::to_value(plan.get_envs()).unwrap()),
                &(serde_json::to_value(plan.get_lifecycle()).unwrap()),
                &(serde_json::to_value(plan.get_status()).unwrap()),
            ],
        ).map_err(Error::PlanCreate)?;

        if rows.len() > 0 {
            return Ok(Some(row_to_plan(&rows.get(0))?));
        }
        Ok(None)
    }

    pub fn show(db: &DataStoreConn, get_plan_factory: &IdGet) -> PlanOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_plan_v1($1)",
            &[&(get_plan_factory.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::PlanGet)?;

        if rows.len() > 0 {
            for row in rows {
                return Ok(Some(row_to_plan(&row)?));
            }
        }
        Ok(None)
    }

    pub fn list_blank(db: &DataStoreConn) -> PlanOutputList {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_plans_v1()", &[])
            .map_err(Error::PlanGet)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_plan(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
}

fn row_to_plan(row: &postgres::rows::Row) -> Result<blueprint::Plan> {
    let mut plan = blueprint::Plan::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    plan.set_id(id.to_string() as String);
    plan.set_status(serde_json::from_value(row.get("status")).unwrap());
    plan.set_category(row.get("category"));
    plan.set_version(row.get("version"));
    plan.set_characteristics(serde_json::from_value(row.get("characteristics")).unwrap());
    plan.set_icon(row.get("icon"));
    plan.set_description(row.get("description"));
    plan.set_ports(serde_json::from_value(row.get("ports")).unwrap());
    plan.set_envs(serde_json::from_value(row.get("envs")).unwrap());
    plan.set_lifecycle(serde_json::from_value(row.get("lifecycle")).unwrap());
    plan.set_created_at(created_at.to_rfc3339());

    Ok(plan)
}
