// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [accounts, login, roles, permissions,] for the HTTP server

use std::env;

use bodyparser;
use hab_core::event::*;
use hab_net::http::controller::*;
use session::auth_ds::AuthenticateDS;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;

use protocol::sessionsrv;
use protocol::net::{self, ErrCode};
use router::Router;
use db::data_store::DataStoreBroker;

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyCreateReq {
    name: String,
    uri: String,
    tags: Vec<String>,
    parent_id: u64,
    description: String,
    node: String,
    status: String,
    ip: String,
    urls: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyFacCreateReq {
    name: String,
    uri: String,
    description: String,
    tags: Vec<String>,
    properties: String,
    plan: String,
    external_management_resource: Vec<String>,
    component_collection: String,
    status: String,
    opssettings: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AssemblyUpdateReq {
    name: String,
    uri: String,
    description: String,
    parent_id: u64,
    tags: Vec<String>,
    node: String,
    ip: String,
    urls: String,
    status: String,
}

pub fn default_authenticate(req: &mut Request) -> IronResult<Response> {
    let code = {
        let params = req.extensions.get::<Router>().unwrap();
        params.find("code").unwrap().to_string()
    };

    let authcli = req.get::<persistent::Read<DefaultAuthCli>>().unwrap();

    if env::var_os("RIOOS_FUNC_TEST").is_some() {
        let session = try!(session_create(&authcli, &code));

        log_event!(
            req,
            Event::DefaultAuthenticate {
                user: session.get_name().to_string(),
                account: session.get_id().to_string(),
            }
        );

        return Ok(render_json(status::Ok, &session));
    }

    match authcli.authenticate(&code) {
        Ok(token) => {
            let session = try!(session_create(&authcli, &token));

            log_event!(
                req,
                Event::DefaultAuthenticate {
                    user: session.get_name().to_string(),
                    account: session.get_id().to_string(),
                }
            );

            Ok(render_json(status::Ok, &session))
        }
        Err(hab_net::Error::Net(err)) => Ok(render_net_error(&err)),
        Err(e) => {
            error!("unhandled default authentication, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:0");
            Ok(render_net_error(&err))
        }
    }
}
