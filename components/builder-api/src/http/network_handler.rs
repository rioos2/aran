// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [origin] for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use network::network_ds::NetworkDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::net::{self, ErrCode};
// use router::Router;
use protocol::servicesrv::ObjectMetaData;
use protocol::asmsrv::TypeMeta;
use protocol::netsrv::{Network, Status};

use db::data_store::Broker;
use std::collections::BTreeMap;
use http::{service_account_handler, deployment_handler};


define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NetworkCreateReq {
    type_meta: deployment_handler::TypeMetaReq,
    object_meta: service_account_handler::ObjectMetaReq,
    name: String,
    host_ip: String,
    storage_type: String,
    parameters: BTreeMap<String, String>,
    status: StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StatusReq {
    health_status: String,
    message: String,
    reason: String,
}

pub fn network_create(req: &mut Request) -> IronResult<Response> {
    let mut net_create = Network::new();
    {
        match req.get::<bodyparser::Struct<NetworkCreateReq>>() {
            Ok(Some(body)) => {
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                net_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                net_create.set_type_meta(type_meta);
                net_create.set_name(body.name);
                net_create.set_host_ip(body.host_ip);
                net_create.set_storage_type(body.storage_type);
                net_create.set_paramaters(body.parameters);
                let mut status = Status::new();
                status.set_health_status(body.status.health_status);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);
                net_create.set_status(status);
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

    match NetworkDS::network_create(&conn, &net_create) {
        Ok(network) => Ok(render_json(status::Ok, &network)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
