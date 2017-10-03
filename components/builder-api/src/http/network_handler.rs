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
use protocol::netsrv::Network;
use protocol::asmsrv::{Status, Condition};

use db::data_store::Broker;
use http::deployment_handler;


define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NetworkCreateReq {
    name: String,
    network_type: String,
    subnet_ip: String,
    netmask: String,
    gateway: String,
    status: deployment_handler::StatusReq,
    created_at: String,
}


pub fn network_create(req: &mut Request) -> IronResult<Response> {
    let mut net_create = Network::new();
    {
        match req.get::<bodyparser::Struct<NetworkCreateReq>>() {
            Ok(Some(body)) => {
                net_create.set_name(body.name);
                net_create.set_network_type(body.network_type);
                net_create.set_subnet_ip(body.subnet_ip);
                net_create.set_netmask(body.netmask);
                net_create.set_gateway(body.gateway);

                let mut status = Status::new();
                status.set_phase(body.status.phase);
                status.set_message(body.status.message);
                status.set_reason(body.status.reason);

                let mut condition_collection = Vec::new();

                for data in body.status.conditions {
                    let mut condition = Condition::new();
                    condition.set_message(data.message);
                    condition.set_reason(data.reason);
                    condition.set_status(data.status);
                    condition.set_last_transition_time(data.last_transition_time);
                    condition.set_last_probe_time(data.last_probe_time);
                    condition.set_condition_type(data.condition_type);
                    condition_collection.push(condition);
                }
                status.set_conditions(condition_collection);
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

#[allow(unused_variables)]
pub fn network_list(req: &mut Request) -> IronResult<Response> {
    let conn = Broker::connect().unwrap();
    match NetworkDS::network_list(&conn) {
        Ok(network_list) => Ok(render_json(status::Ok, &network_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}