// Copyright (c) 2017 RioCorp Inc.

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::Broker;

define_event_log!();

pub fn watch_show(_req: &mut Request) -> IronResult<Response> {
    // let conn = Broker::connect().unwrap();
    // // let name = {
    // //     let params = req.extensions.get::<Router>().unwrap();
    // //     match params.find("name").unwrap().parse::<u64>() {
    // //         Ok(id) => id,
    // //         Err(_) => return Ok(Response::with(status::BadRequest)),
    // //     }
    // // };
    // WorkerDS::worker_stream(&conn);
    // // match WorkerDS::worker_stream(&conn) {
    // //     Ok(hs_list) => Ok(render_json(status::Ok)),
    // //     Err(err) => Ok(render_net_error(
    // //         &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
    // //     )),
    // // }
    //
    Ok(Response::with(status::Ok))
}
