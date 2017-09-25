// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use scale::scaling_ds::ScalingDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::scalesrv::{HorizontalScaling, Spec, Metrics, MetricObject, MetricResource, TimeSpec, Status};
use protocol::asmsrv::{ObjectMeta, OwnerReferences, TypeMeta};
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;
use std::collections::BTreeMap;
define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HsCreateReq {
    name: String,
    description: String,
    tags: Vec<String>,
    scale_type: String,
    representation_skew: String,
    state: String,
    metadata: Vec<String>,
    spec: SpecReq,
    status: StatusReq,
    object_meta: ObjectMetaReq,
    type_meta: TypeMetaReq,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TypeMetaReq {
    kind: String,
    api_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ObjectMetaReq {
    name: String,
    origin: String,
    uid: String,
    created_at: String,
    cluster_name: String,
    labels: BTreeMap<String, String>,
    annotations: BTreeMap<String, String>,
    owner_references: Vec<OwnerReferencesReq>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct OwnerReferencesReq {
    kind: String,
    api_version: String,
    name: String,
    uid: String,
    block_owner_deletion: bool,
}

pub fn hs_create(req: &mut Request) -> IronResult<Response> {
    let mut hs_create = HorizontalScaling::new();

    {
        match req.get::<bodyparser::Struct<HsCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                hs_create.set_name(body.name);
                hs_create.set_description(body.description);
                hs_create.set_tags(body.tags);
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

                let mut status = Status::new();

                status.set_last_scale_time(body.status.last_scale_time);
                status.set_current_replicas(body.status.current_replicas);
                status.set_desired_replicas(body.status.desired_replicas);
                hs_create.set_status(status);

                let mut object_meta = ObjectMeta::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                let mut owner_references_collection = Vec::new();
                for data in body.object_meta.owner_references {
                    let mut owner_references = OwnerReferences::new();
                    owner_references.set_kind(data.kind);
                    owner_references.set_api_version(data.api_version);
                    owner_references.set_name(data.name);
                    owner_references.set_uid(data.uid);
                    owner_references.set_block_owner_deletion(data.block_owner_deletion);
                    owner_references_collection.push(owner_references);
                }
                object_meta.set_owner_references(owner_references_collection);
                hs_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                hs_create.set_type_meta(type_meta);

            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = Broker::connect().unwrap();

    match ScalingDS::hs_create(&conn, &hs_create) {
        Ok(response) => Ok(render_json(status::Ok, &response)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn hs_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match ScalingDS::hs_list(&conn) {
        Ok(hs_list) => Ok(render_json(status::Ok, &hs_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}


pub fn hs_status_update(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };
    let mut hs_update = HorizontalScaling::new();
    hs_update.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<HsStatusReq>>() {
            Ok(Some(body)) => {
                let mut status = Status::new();
                status.set_last_scale_time(body.status.last_scale_time);
                status.set_current_replicas(body.status.current_replicas);
                status.set_desired_replicas(body.status.desired_replicas);
                hs_update.set_status(status);
            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match ScalingDS::hs_status_update(&conn, &hs_update) {
        Ok(hs_update) => Ok(render_json(status::Ok, &hs_update)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
