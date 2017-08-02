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
use protocol::scalesrv::HorizontalScaling;
use protocol::sessionsrv;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::DataStoreBroker;


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
    rules: Vec<String>,
    properties: Vec<String>,
    status: String,
}

pub fn hs_create(req: &mut Request) -> IronResult<Response> {
    println!("-----------------------------------------------------");
    let mut hs_create = HorizontalScaling::new();
    {
        match req.get::<bodyparser::Struct<HsCreateReq>>() {
            Ok(Some(body)) => {
                println!("----------------------{:?}", body.name);
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
                hs_create.set_target_resource(body.target_resource);
                hs_create.set_rules(body.rules);
                hs_create.set_properties(body.properties);
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
