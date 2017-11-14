// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [origin] for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use router::Router;
use job::job_ds::JobDS;
use protocol::jobsrv::{SpecData, Jobs};
use protocol::servicesrv::ObjectMetaData;
use protocol::asmsrv::{TypeMeta, Status, Condition, IdGet};
use db::data_store::Broker;
use common::ui;
use ansi_term::Colour;
use std::collections::BTreeMap;
use db;
use http::{service_account_handler, deployment_handler};
use rio_net::util::errors::AranResult;
use error::{Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER, INVALIDQUERY};
use rio_net::util::errors::{bad_request, internal_error, malformed_body, not_found_error};
const JOB: &'static str = "Job";
use extract_query_value;


define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JobsReq {
    node_id: String,
    type_meta: deployment_handler::TypeMetaReq,
    object_meta: service_account_handler::ObjectMetaReq,
    spec: SpecDataReq,
    status: deployment_handler::StatusReq,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SpecDataReq {
    node_id: String,
    target_ref: String,
    selector: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct JobStatusReq {
    status: deployment_handler::StatusReq,
}


pub fn jobs_create(req: &mut Request) -> AranResult<Response> {
    let mut jobs_create = Jobs::new();
    {
        match req.get::<bodyparser::Struct<JobsReq>>() {
            Ok(Some(body)) => {
                if body.node_id.len() <= 0 {
                    return Err(bad_request(&format!("{} {}", MISSING_FIELD, "node_id")));
                }
                jobs_create.set_node_id(body.node_id);
                let mut spec = SpecData::new();
                spec.set_node_id(body.spec.node_id);
                spec.set_target_ref(body.spec.target_ref);
                spec.set_selector(body.spec.selector);
                jobs_create.set_spec(spec);

                jobs_create.set_status(Status::with_conditions(
                    &body.status.phase,
                    &body.status.message,
                    &body.status.reason,
                    body.status
                        .conditions
                        .iter()
                        .map(|x| {
                            Condition::with_type(
                                &x.message,
                                &x.reason,
                                &x.status,
                                &x.last_transition_time,
                                &x.last_probe_time,
                                &x.condition_type,
                            )
                        })
                        .collect::<Vec<_>>(),
                ));
                let mut object_meta = ObjectMetaData::new();
                object_meta.set_name(body.object_meta.name);
                object_meta.set_origin(body.object_meta.origin);
                object_meta.set_uid(body.object_meta.uid);
                object_meta.set_created_at(body.object_meta.created_at);
                object_meta.set_cluster_name(body.object_meta.cluster_name);
                object_meta.set_labels(body.object_meta.labels);
                object_meta.set_annotations(body.object_meta.annotations);
                jobs_create.set_object_meta(object_meta);
                jobs_create.set_type_meta(TypeMeta::new(JOB));

            }
            Err(err) => {
                return Err(malformed_body(
                    &format!("{}, {:?}\n", err.detail, err.cause),
                ));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    ui::rawdumpln(
        Colour::White,
        '✓',
        format!("======= parsed {:?} ", jobs_create),
    );

    let conn = Broker::connect().unwrap();

    match JobDS::jobs_create(&conn, &jobs_create) {
        Ok(Some(jobs)) => Ok(render_json(status::Ok, &jobs)),
        Err(err) => Err(internal_error(&format!("{}\n", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }

    }
}
#[allow(unused_variables)]
pub fn jobs_get(req: &mut Request) -> AranResult<Response> {
    let conn = Broker::connect().unwrap();
    match JobDS::jobs_get(&conn) {
        Ok(Some(jobs_get)) => Ok(render_json(status::Ok, &jobs_get)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}


pub fn jobs_get_by_node(req: &mut Request) -> AranResult<Response> {
    let node_id = {
        match extract_query_value("node_id", req) {
            Some(id) => id,
            None => return Err(bad_request(&INVALIDQUERY)),
        }
    };
    let conn = Broker::connect().unwrap();

    let mut nodeid_get = IdGet::new();
    nodeid_get.set_id(node_id.to_string());
    match JobDS::jobs_get_by_node(&conn, &nodeid_get) {
        Ok(Some(jobs_get)) => Ok(render_json(status::Ok, &jobs_get)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(
                &format!("{}", Error::Db(db::error::Error::RecordsNotFound)),
            ))
        }
    }
}
pub fn jobs_status_update(req: &mut Request) -> AranResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("jobid").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Err(bad_request(IDMUSTNUMBER)),
        }
    };
    let mut jobs = Jobs::new();
    jobs.set_id(id.to_string());
    {
        match req.get::<bodyparser::Struct<JobStatusReq>>() {
            Ok(Some(body)) => {
                jobs.set_status(Status::with_conditions(
                    &body.status.phase,
                    &body.status.message,
                    &body.status.reason,
                    body.status
                        .conditions
                        .iter()
                        .map(|x| {
                            Condition::with_type(
                                &x.message,
                                &x.reason,
                                &x.status,
                                &x.last_transition_time,
                                &x.last_probe_time,
                                &x.condition_type,
                            )
                        })
                        .collect::<Vec<_>>(),
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

    match JobDS::jobs_status_update(&conn, &jobs) {
        Ok(Some(jobs)) => Ok(render_json(status::Ok, &jobs)),
        Err(err) => Err(internal_error(&format!("{}", err))),
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &jobs.get_id()
            )))
        }

    }
}
