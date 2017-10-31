use std::collections::BTreeMap;
use bodyparser;
use iron::prelude::*;
use iron::status;
use persistent;
use router::Router;
use ansi_term::Colour;

use protocol::net::{self, ErrCode};
use rio_net::http::controller::*;
use plan::plan_ds::PlanDS;
use db::data_store::Broker;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body,not_found_error};
use error::{Result, Error, MISSING_FIELD, BODYNOTFOUND, IDMUSTNUMBER};

use protocol::plansrv::{Plan, Service};
use common::ui;
use db;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PlanCreateReq {
    group_name: String,
    url: String,
    description: String,
    tags: Vec<String>,
    origin: String,
    artifacts: Vec<String>,
    services: Vec<ServiceReq>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ServiceReq {
    name: String,
    description: String,
    href: String,
    characteristics: BTreeMap<String, String>,
}

pub fn plan_factory_create(req: &mut Request) -> AranResult<Response> {
    let mut plan_create = Plan::new();
    {
        match req.get::<bodyparser::Struct<PlanCreateReq>>() {
            Ok(Some(body)) => {

                plan_create.set_group_name(body.group_name);
                plan_create.set_url(body.url);
                plan_create.set_description(body.description);
                plan_create.set_tags(body.tags);
                plan_create.set_origin(body.origin);
                plan_create.set_artifacts(body.artifacts);

                let mut service_collection = Vec::new();

                for service in body.services {
                    let mut serv = Service::new();
                    serv.set_name(service.name);
                    serv.set_description(service.description);
                    serv.set_href(service.href);
                    serv.set_characteristics(service.characteristics);
                    service_collection.push(serv);
                }
                plan_create.set_services(service_collection);
            }
            Err(err) => {
                return Err(malformed_body(&format!("{}, {:?}\n", err.detail, err.cause),));
            }
            _ => return Err(malformed_body(&BODYNOTFOUND)),
        }
    }

    let conn = Broker::connect().unwrap();

    match PlanDS::plan_create(&conn, &plan_create) {
        Ok(Some(plan)) => Ok(render_json(status::Ok, &plan)),
        Err(err) => {
            Err(internal_error(&format!("{}\n", err)))
        }
        Ok(None) => {
            Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(db::error::Error::RecordsNotFound),
                &plan_create.get_id()
            )))
    }
}
}
