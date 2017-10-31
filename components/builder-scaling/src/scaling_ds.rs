// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Scaling [horizonalscaler].

use chrono::prelude::*;
use error::{Result, Error};
use protocol::{scalesrv, asmsrv, nodesrv};
use postgres;
use db::data_store::DataStoreConn;
use serde_json;
use rio_net::metrics::prometheus::PrometheusClient;
use rio_net::metrics::collector::{Collector, CollectorScope};

const METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID: &'static str = "rioos_assemblyfactory_id";
const METRIC_LBL_RIOOS_SOURCENAME: &'static str = "rioos_source";
const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";


pub struct ScalingDS;

impl ScalingDS {
    pub fn hs_create(datastore: &DataStoreConn, hs: &scalesrv::HorizontalScaling) -> Result<Option<scalesrv::HorizontalScaling>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(hs.get_spec()).unwrap();
        let status_str = serde_json::to_string(hs.get_status()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_hs_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(hs.get_name() as String),
                &(hs.get_description() as String),
                &(hs.get_tags() as Vec<String>),
                &(hs.get_origin() as String),
                &(hs.get_scale_type() as String),
                &(hs.get_representation_skew() as String),
                &(hs.get_state() as String),
                &(hs.get_metadata() as Vec<String>),
                &(spec_str as String),
                &(status_str as String),
            ],
        ).map_err(Error::HSCreate)?;
if rows.len() > 0 {
        let hs = row_to_hs(&rows.get(0))?;
        return Ok(Some(hs));
    }
    Ok(None)
    }

    pub fn hs_list(datastore: &DataStoreConn) -> Result<Option<scalesrv::HorizontalScalingGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_hs_v1()", &[]).map_err(
            Error::HSGet,
        )?;

        let mut response = scalesrv::HorizontalScalingGetResponse::new();

        let mut hs_collection = Vec::new();
    if rows.len() > 0 {
        for row in rows {
            hs_collection.push(row_to_hs(&row)?)
        }
        response.set_hs_collection(
            hs_collection,
            "HorizontalPodAutoscalerList".to_string(),
            "v1".to_string(),
        );
        return Ok(Some(response));
    }
    Ok(None)
    }
    pub fn horizontal_scaling_list_by_origin(datastore: &DataStoreConn, hs_get: &asmsrv::IdGet) -> Result<Option<scalesrv::HorizontalScalingGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query(
            "SELECT * FROM get_hs_by_origin_v1($1)",
            &[&(hs_get.get_id() as String)],
        ).map_err(Error::HSGet)?;

        let mut response = scalesrv::HorizontalScalingGetResponse::new();

        let mut hs_collection = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                hs_collection.push(row_to_hs(&row)?)
            }
            response.set_hs_collection(
                hs_collection,
                "HorizontalPodAutoscalerList".to_string(),
                "v1".to_string(),
            );
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn hs_status_update(datastore: &DataStoreConn, hs: &scalesrv::HorizontalScaling) -> Result<Option<scalesrv::HorizontalScaling>> {
        let conn = datastore.pool.get_shard(0)?;
        let id = hs.get_id().parse::<i64>().unwrap();
        let status_str = serde_json::to_string(hs.get_status()).unwrap();
        let rows = &conn.query(
            "SELECT set_hs_status_v1($1, $2)",
            &[&id, &(status_str as String)],
        ).map_err(Error::HSSetStatus)?;
        if rows.len() > 0 {
            let hs = row_to_hs(&rows.get(0))?;
            return Ok(Some(hs));
        }
        Ok(None)

    }

    pub fn hs_update(datastore: &DataStoreConn, hs: &scalesrv::HorizontalScaling) -> Result<Option<scalesrv::HorizontalScaling>> {
        let conn = datastore.pool.get_shard(0)?;
        let spec_str = serde_json::to_string(hs.get_spec()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM update_hs_v1($1,$2,$3,$4,$5,$6,$7,$8,$9)",
            &[
                &(hs.get_id().parse::<i64>().unwrap()),
                &(hs.get_name() as String),
                &(hs.get_description() as String),
                &(hs.get_tags() as Vec<String>),
                &(hs.get_scale_type() as String),
                &(hs.get_representation_skew() as String),
                &(hs.get_state() as String),
                &(hs.get_metadata() as Vec<String>),
                &(spec_str as String),
            ],
        ).map_err(Error::HSUpdate)?;
            if rows.len() > 0 {
        let hscale = row_to_hs(&rows.get(0))?;
        return Ok(Some(hscale));
    }
    Ok(None)
    }

    pub fn hs_metrics(client: &PrometheusClient, af_id: &str, metric_source_name: &str) -> Result<Option<scalesrv::ScalingGetResponse>> {
        let label_name = format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID, af_id);
        let metric_scope = vec![metric_source_name.to_string()];
        let group_scope: Vec<String> = vec![label_name.to_string()];

        let scope = CollectorScope {
            metric_names: metric_scope,
            labels: group_scope,
            last_x_minutes: Some(METRIC_DEFAULT_LAST_X_MINUTE.to_string()),
        };

        let mut metric_collector = Collector::new(client, scope);
        let metric_response = metric_collector.metric_by().unwrap();

        let mut metrics = nodesrv::Osusages::new();

        let all_items = metric_response
            .into_iter()
            .map(|p| {
                let p1: nodesrv::Osusages = p.into();
                p1.get_items()
            })
            .collect::<Vec<_>>();

        metrics.set_items(all_items.iter().flat_map(|s| (*s).clone()).collect());

        let mut response = scalesrv::ScalingGet::new();
        response.set_title("Scaling metrics ".to_owned() + af_id);
        /*res.set_from_date(from_date);
        res.set_to_date(to_date);*/
        response.set_metrics(metrics);

        let response: scalesrv::ScalingGetResponse = response.into();

        Ok(Some(response))
    }
}

fn row_to_hs(row: &postgres::rows::Row) -> Result<scalesrv::HorizontalScaling> {
    let mut hs = scalesrv::HorizontalScaling::new();

    let id: i64 = row.get("id");
    let name: String = row.get("name");
    let description: String = row.get("description");
    let tags: Vec<String> = row.get("tags");
    let origin: i64 = row.get("origin_id");
    let scale_type: String = row.get("scale_type");
    let representation_skew: String = row.get("representation_skew");
    let state: String = row.get("state");
    let metadata: Vec<String> = row.get("metadata");
    let status: String = row.get("status");
    let spec: String = row.get("spec");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");


    hs.set_id(id.to_string() as String);
    hs.set_name(name as String);
    hs.set_description(description as String);
    hs.set_tags(tags as Vec<String>);
    hs.set_origin(origin.to_string());
    hs.set_scale_type(scale_type as String);
    hs.set_representation_skew(representation_skew as String);
    hs.set_state(state as String);
    hs.set_metadata(metadata as Vec<String>);
    let spec_obj: scalesrv::Spec = serde_json::from_str(&spec).unwrap();
    let status_obj: scalesrv::Status = serde_json::from_str(&status).unwrap();
    hs.set_spec(spec_obj);
    hs.set_status(status_obj);

    let mut obj_meta = asmsrv::ObjectMeta::new();
    let mut owner_collection = Vec::new();
    let owner = asmsrv::OwnerReferences::new();
    owner_collection.push(owner);
    obj_meta.set_name(id.to_string());
    obj_meta.set_owner_references(owner_collection);
    hs.set_object_meta(obj_meta);
    let mut type_meta = asmsrv::TypeMeta::new();
    type_meta.set_kind("HorizontalPodAutoscaler".to_string());
    type_meta.set_api_version("v1".to_string());
    hs.set_type_meta(type_meta);

    hs.set_created_at(created_at.to_rfc3339());
    Ok(hs)
}
