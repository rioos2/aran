// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use bodyparser;
use persistent;
use rio_core::event::*;
use rio_net::http::controller::*;
use rio_net::http::middleware::PrometheusCli;
use ansi_term::Colour;

use scale::scaling_ds::ScalingDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::scalesrv::{HorizontalScaling, Spec, Metrics, MetricObject, MetricResource, TimeSpec, Status};
use protocol::asmsrv::IdGet;
use router::Router;
use db::data_store::Broker;
use db;
use common::ui;
use rio_net::util::errors::AranResult;
use error::{Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HsCreateReq {
    name: String,
    description: String,
    tags: Vec<String>,
    origin: String,
    scale_type: String,
    representation_skew: String,
    state: String,
    metadata: Vec<String>,
    spec: SpecReq,
    status: StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpecReq {
    scale_target_ref: String,
    min_replicas: u64,
    max_replicas: u64,
    metrics: Vec<MetricsReq>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct MetricsReq {
    metric_type: String,
    object: MetricObjectReq,
    resource: MetricResourceReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MetricObjectReq {
    target: String,
    target_value: u64,
    metric_time_spec: ObjTimeSpecReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MetricResourceReq {
    name: String,
    min_target_value: String,
    max_target_value: String,
    metric_time_spec: ObjTimeSpecReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjTimeSpecReq {
    scale_up_by: String,
    scale_up_wait_time: String,
    scale_down_by: String,
    scale_down_wait_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusReq {
    last_scale_time: String,
    current_replicas: u64,
    desired_replicas: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HsStatusReq {
    status: StatusReq,
}

pub fn hs_create(req: &mut Request) -> AranResult<Response> {
    let mut hs_create = HorizontalScaling::new();

    {
        match req.get::<bodyparser::Struct<HsCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "name")));
                }
                if body.origin.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "origin")));
                }
                hs_create.set_name(body.name);
                hs_create.set_description(body.description);
                hs_create.set_tags(body.tags);
                hs_create.set_origin(body.origin);
                hs_create.set_scale_type(body.scale_type);
                hs_create.set_representation_skew(body.representation_skew);
                hs_create.set_metadata(body.metadata);
                hs_create.set_state(body.state);

                let mut spec = Spec::new();

                spec.set_scale_target_ref(body.spec.scale_target_ref);
                spec.set_min_replicas(body.spec.min_replicas);
                spec.set_max_replicas(body.spec.max_replicas);

                let mut metrics_collection = Vec::new();

                for data in body.spec.metrics {

                    let mut metrics = Metrics::new();

                    metrics.set_metric_type(data.metric_type);

                    let mut metrics_obj = MetricObject::new();

                    metrics_obj.set_target(data.object.target);
                    metrics_obj.set_target_value(data.object.target_value);

                    let mut obj_time_spec = TimeSpec::new();

                    obj_time_spec.set_scale_up_by(data.object.metric_time_spec.scale_up_by);
                    obj_time_spec.set_scale_up_wait_time(data.object.metric_time_spec.scale_up_wait_time);
                    obj_time_spec.set_scale_down_by(data.object.metric_time_spec.scale_down_by);
                    obj_time_spec.set_scale_down_wait_time(data.object.metric_time_spec.scale_down_wait_time);

                    metrics_obj.set_metric_time_spec(obj_time_spec);

                    metrics.set_metric_object(metrics_obj);

                    let mut metrics_res = MetricResource::new();

                    metrics_res.set_name(data.resource.name);
                    metrics_res.set_min_target_value(data.resource.min_target_value);
                    metrics_res.set_max_target_value(data.resource.max_target_value);

                    let mut res_time_spec = TimeSpec::new();

                    res_time_spec.set_scale_up_by(data.resource.metric_time_spec.scale_up_by);
                    res_time_spec.set_scale_up_wait_time(data.resource.metric_time_spec.scale_up_wait_time);
                    res_time_spec.set_scale_down_by(data.resource.metric_time_spec.scale_down_by);
                    res_time_spec.set_scale_down_wait_time(data.resource.metric_time_spec.scale_down_wait_time);

                    metrics_res.set_metric_time_spec(res_time_spec);

                    metrics.set_metric_resource(metrics_res);

                    metrics_collection.push(metrics);
                }

                spec.set_metrics(metrics_collection);
                hs_create.set_spec(spec);

                hs_create.set_status(Status::new(
                    &body.status.last_scale_time,
                    body.status.current_replicas,
                    body.status.desired_replicas,
                ));
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }
    let conn = Broker::connect().unwrap();

    match ScalingDS::hs_create(&conn, &hs_create) {
        Ok(Some(response)) => Ok(render_json(status::Ok, &response)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}
#[allow(unused_variables)]
pub fn hs_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match ScalingDS::hs_list(&conn) {
        Ok(Some(hs_list)) => Ok(render_json(status::Ok, &hs_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}


pub fn horizontal_scaling_list_by_origin(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };

    let conn = Broker::connect().unwrap();

    let mut hs_get = IdGet::new();
    hs_get.set_id(org_name);

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", hs_get),
    );
    match ScalingDS::horizontal_scaling_list_by_origin(&conn, &hs_get) {
        Ok(Some(hs)) => Ok(render_json(status::Ok, &hs)),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &hs_get.get_id()
            )))
        }
        Err(err) => Err(internal_error(&format!("{}\n", err))),
    }
}



pub fn hs_metrics(req: &mut Request) -> AranResult<Response> {
    let promcli = req.get::<persistent::Read<PrometheusCli>>().unwrap();
    let af_id = {
        let params = req.extensions.get::<Router>().unwrap();
        let id = params.find("id").unwrap().to_owned();
        id
    };
    let source = {
        let params = req.extensions.get::<Router>().unwrap();
        let source = params.find("source").unwrap().to_owned();
        source
    };
    match ScalingDS::hs_metrics(&promcli, &af_id, &source) {
        Ok(Some(hs_metrics)) => Ok(render_json(status::Ok, &hs_metrics)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &af_id
            )))
        }
    }
}

pub fn hs_status_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    let mut hs_update = HorizontalScaling::new();
    hs_update.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<HsStatusReq>>() {
            Ok(Some(body)) => {
                hs_update.set_status(Status::new(
                    &body.status.last_scale_time,
                    body.status.current_replicas,
                    body.status.desired_replicas,
                ));
            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match ScalingDS::hs_status_update(&conn, &hs_update) {
        Ok(Some(hs_update)) => Ok(render_json(status::Ok, &hs_update)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &hs_update.get_id()
            )))
        }
    }
}

pub fn hs_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(&IDMUSTNUMBER)),
        }
    };
    let mut hs_update = HorizontalScaling::new();
    {
        match req.get::<bodyparser::Struct<HsCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "name")));
                }

                hs_update.set_id(id.to_string());
                hs_update.set_name(body.name);
                hs_update.set_description(body.description);
                hs_update.set_tags(body.tags);
                hs_update.set_scale_type(body.scale_type);
                hs_update.set_representation_skew(body.representation_skew);
                hs_update.set_metadata(body.metadata);
                hs_update.set_state(body.state);

                let mut spec = Spec::new();

                spec.set_scale_target_ref(body.spec.scale_target_ref);
                spec.set_min_replicas(body.spec.min_replicas);
                spec.set_max_replicas(body.spec.max_replicas);

                let mut metrics_collection = Vec::new();

                for data in body.spec.metrics {

                    let mut metrics = Metrics::new();

                    metrics.set_metric_type(data.metric_type);

                    let mut metrics_obj = MetricObject::new();

                    metrics_obj.set_target(data.object.target);
                    metrics_obj.set_target_value(data.object.target_value);

                    let mut obj_time_spec = TimeSpec::new();

                    obj_time_spec.set_scale_up_by(data.object.metric_time_spec.scale_up_by);
                    obj_time_spec.set_scale_up_wait_time(data.object.metric_time_spec.scale_up_wait_time);
                    obj_time_spec.set_scale_down_by(data.object.metric_time_spec.scale_down_by);
                    obj_time_spec.set_scale_down_wait_time(data.object.metric_time_spec.scale_down_wait_time);

                    metrics_obj.set_metric_time_spec(obj_time_spec);

                    metrics.set_metric_object(metrics_obj);

                    let mut metrics_res = MetricResource::new();

                    metrics_res.set_name(data.resource.name);
                    metrics_res.set_min_target_value(data.resource.min_target_value);
                    metrics_res.set_max_target_value(data.resource.max_target_value);

                    let mut res_time_spec = TimeSpec::new();

                    res_time_spec.set_scale_up_by(data.resource.metric_time_spec.scale_up_by);
                    res_time_spec.set_scale_up_wait_time(data.resource.metric_time_spec.scale_up_wait_time);
                    res_time_spec.set_scale_down_by(data.resource.metric_time_spec.scale_down_by);
                    res_time_spec.set_scale_down_wait_time(data.resource.metric_time_spec.scale_down_wait_time);

                    metrics_res.set_metric_time_spec(res_time_spec);

                    metrics.set_metric_resource(metrics_res);

                    metrics_collection.push(metrics);
                }

                spec.set_metrics(metrics_collection);
                hs_update.set_spec(spec);

            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match ScalingDS::hs_update(&conn, &hs_update) {
        Ok(Some(hs)) => Ok(render_json(status::Ok, &hs)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &hs_update.get_id()
            )))
        }

    }
}
