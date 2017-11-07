// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [origin] for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use session::session_ds::SessionDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::originsrv::Origin;
use protocol::net::{self, ErrCode};
use router::Router;
use protocol::servicesrv::ObjectMetaData;
use protocol::asmsrv::{TypeMeta, IdGet};
use db::data_store::Broker;
use db;
use http::{service_account_handler, deployment_handler};
use rio_net::util::errors::AranResult;
use error::{Result, Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OriginCreateReq {
    type_meta: deployment_handler::TypeMetaReq,
    object_meta: service_account_handler::ObjectMetaReq,
}

pub fn origin_create(req: &mut Request) -> AranResult<Response> {
    let mut org_create = Origin::new();
    {
        match req.get::<bodyparser::Struct<OriginCreateReq>>() {
            Ok(Some(body)) => {
                if body.object_meta.uid.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "uid")));
                }
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
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::origin_create(&conn, &org_create) {
        Ok(Some(origin)) => Ok(render_json(status::Ok, &origin)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}

#[allow(unused_variables)]
pub fn origin_list(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match SessionDS::origin_list(&conn) {
        Ok(Some(org_list)) => Ok(render_json(status::Ok, &org_list)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}


pub fn origin_show(req: &mut Request) -> AranResult<Response> {
    let org_name = {
        let params = req.extensions.get::<Router>().unwrap();
        let org_name = params.find("origin").unwrap().to_owned();
        org_name
    };
    let conn = Broker::connect().unwrap();

    let mut org_get = IdGet::new();
    org_get.set_id(org_name);
    match SessionDS::origin_show(&conn, &org_get) {
        Ok(Some(origin)) => Ok(render_json(status::Ok, &origin)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &org_get.get_id()
            )))
        }
    }
}
