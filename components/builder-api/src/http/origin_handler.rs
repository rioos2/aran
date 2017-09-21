// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [origin] for the HTTP server

use std::env;

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use session::session_ds::SessionDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::originsrv::{Origin, OriginGet};
use protocol::net::{self, ErrCode};
use router::Router;
use protocol::servicesrv::ObjectMetaData;
use protocol::asmsrv::TypeMeta;
use db::data_store::Broker;
use std::collections::BTreeMap;


define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OriginCreateReq {
    type_meta: TypeMetaReq,
    object_meta: ObjectMetaReq,
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
}

pub fn origin_create(req: &mut Request) -> IronResult<Response> {
    let mut org_create = Origin::new();
    {
        match req.get::<bodyparser::Struct<OriginCreateReq>>() {
            Ok(Some(body)) => {
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                org_create.set_object_meta(object_meta);
                let mut type_meta = TypeMeta::new();
                type_meta.set_kind(body.type_meta.kind);
                type_meta.set_api_version(body.type_meta.api_version);
                org_create.set_type_meta(type_meta);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::origin_create(&conn, &org_create) {
        Ok(origin) => Ok(render_json(status::Ok, &origin)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn origin_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match SessionDS::origin_list(&conn) {
        Ok(org_list) => Ok(render_json(status::Ok, &org_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}


pub fn origin_show(req: &mut Request) -> IronResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };
    let conn = Broker::connect().unwrap();

    let mut org_get = OriginGet::new();
    org_get.set_name(org_name);
    match SessionDS::origin_show(&conn, &org_get) {
        Ok(origin) => Ok(render_json(status::Ok, &origin)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}
