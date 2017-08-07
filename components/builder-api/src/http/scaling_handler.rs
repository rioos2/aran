// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use std::env;

use bodyparser;
use hab_core::event::*;
use hab_net::http::controller::*;
use scale::scaling_ds::ScalingDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;
use protocol::scalesrv::{HorizontalScaling, Spec};
use protocol::sessionsrv;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::DataStoreBroker;
use serde_json;

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HsCreateReq {
    name: String,
    description: String,
    tags: Vec<String>,
    hs_type: String,
    representation_skew: String,
    target_resource: String,
    metadata: Vec<String>,
    spec: SpecReq,
    status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpecReq {
    scale_target_ref: String,
    min_replicas: u64,
    max_replicas: u64,
    // metrics: Vec<Metrics>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Metrics {
    metric_type: String,
    object: MetricObject,
    resource: MetricResource,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MetricObject {
    target: String,
    target_value: u64,
    metric_time_spec: TimeSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MetricResource {
    name: String,
    min_target_value: String,
    max_target_value: String,
    metric_time_spec: TimeSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TimeSpec {
    scale_up_by: String,
    scale_up_wait_time: String,
    scale_down_by: String,
    scale_down_wait_time: String,
}


pub fn hs_create(req: &mut Request) -> IronResult<Response> {
    let mut hs_create = HorizontalScaling::new();
    let mut spec = Spec::new();
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
                hs_create.set_hs_type(body.hs_type);
                hs_create.set_representation_skew(body.representation_skew);
                hs_create.set_metadata(body.metadata);
                hs_create.set_status(body.status);
                spec.set_scale_target_ref(body.spec.scale_target_ref);
                spec.set_min_replicas(body.spec.min_replicas);
                spec.set_max_replicas(body.spec.max_replicas);
                let encoded = serde_json::to_string(&spec).unwrap();
                hs_create.set_spec_as_string(encoded);
                hs_create.set_target_resource(body.target_resource);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();
    //This is needed as you'll need the email/token if any
    // let session = req.extensions.get::<Authenticated>().unwrap().clone();
    match ScalingDS::hs_create(&conn, &hs_create) {
        Ok(response) => Ok(render_json(status::Ok, &response)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
