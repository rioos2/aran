// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the Authorization [assembly, assemblyfactory].
use chrono::prelude::*;
use error::{Result, Error};

use protocol::api::blueprint;
use protocol::api::base::{IdGet, MetaFields, StatusUpdate};

use postgres;
use db::data_store::DataStoreConn;

use serde_json;

use super::super::{PlanOutputList, PlanOutput};

pub struct DataStore;

impl DataStore {
    pub fn create(db: &DataStoreConn, plan: &blueprint::Plan) -> PlanOutput {
        let conn = db.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM insert_plan_factory_v1($1,$2,$3,$4,$5,$6,$7,$8)",
            &[
            &(serde_json::to_value(plan.type_meta()).unwrap()),
            &(serde_json::to_value(plan.object_meta()).unwrap()),
            &(serde_json::to_value(plan.get_plan()).unwrap()),
            &(plan.get_category() as String),
            &(plan.get_version() as String),
            &(plan.get_icon() as String),
            &(plan.get_description() as String),
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

        let rows = &conn.query("SELECT * FROM get_plans_v1()", &[]).map_err(
            Error::PlanGet,
        )?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_plan(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }
    pub fn status_update(datastore: &DataStoreConn, plan: &StatusUpdate) -> PlanOutput {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM set_plan_status_v1($1, $2)",
            &[
                &(plan.get_id().parse::<i64>().unwrap()),
                &(serde_json::to_value(plan.get_status()).unwrap()),
            ],
        ).map_err(Error::PlanSetStatus)?;
        if rows.len() > 0 {
            let plan = row_to_plan(&rows.get(0))?;
            return Ok(Some(plan));
        }
        Ok(None)
    }
}

fn row_to_plan(row: &postgres::rows::Row) -> Result<blueprint::Plan> {
    let mut planfactory = blueprint::Plan::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    planfactory.set_status(serde_json::from_value(row.get("status")).unwrap());
    planfactory.set_category(row.get("category"));
    planfactory.set_version(row.get("version"));
    planfactory.set_icon(row.get("icon"));
    planfactory.set_description(row.get("description"));
    planfactory.set_id(id.to_string() as String);
    planfactory.set_created_at(created_at.to_string() as String);
    planfactory.set_plan(serde_json::from_value(row.get("plans")).unwrap());
    Ok(planfactory)
}
