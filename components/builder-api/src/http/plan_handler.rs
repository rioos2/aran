use std::collections::BTreeMap;
use bodyparser;
use iron::prelude::*;
use iron::status;
use persistent;
use router::Router;
use ansi_term::Colour;

use protocol::net::{self, ErrCode};
use rio_net::http::controller::*;
use rio_net::http::middleware::PrometheusCli;
use plan::plan_ds::PlanDS;
use db::data_store::Broker;
use rio_net::util::errors::AranResult;
use rio_net::util::errors::{bad_request, internal_error, malformed_body};

use protocol::plansrv::{Plan, Service};
// use protocol::asmsrv::Condition;
// use http::deployment_handler;
use common::ui;

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

pub fn plan_factory_create(req: &mut Request) -> IronResult<Response> {
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
                    let mut services = Service::new();
                    services.set_name(service.name);
                    services.set_description(service.description);
                    services.set_href(service.href);
                    services.set_characteristics(service.characteristics);
                    service_collection.push(services);
                }
                plan_create.set_services(service_collection);
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

    match PlanDS::plan_create(&conn, &plan_create) {
        Ok(plan) => Ok(render_json(status::Ok, &plan)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
