// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [origin] for the HTTP server

use std::env;

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use session::auth_ds::SessionDS;

use iron::prelude::*;
use iron::status;
use iron::typemap;

use protocol::sessionsrv;
use router::Router;

use db::data_store::Broker;

define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SessionCreateReq {
    token: String,
    extern_id: String,
    email: Vec<String>,
    login: u64,
    provider: String,
}

pub fn account_origin_invitation_create(req: &mut Request) -> IronResult<Response> {
    let code = {
        let params = req.extensions.get::<Router>().unwrap();
        params.find("code").unwrap().to_string()
    };

    let authcli = req.get::<persistent::Read<PasswordAuthCli>>().unwrap();

    if env::var_os("RIOOS_FUNC_TEST").is_some() {
        let session = try!(session_create(&authcli, &code));

        log_event!(
            req,
            Event::PasswordAuthenticate {
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
                Event::PAsswordAuthenticate {
                    user: session.get_name().to_string(),
                    account: session.get_id().to_string(),
                }
            );

            Ok(render_json(status::Ok, &session))
        }
        Err(e) => {
            error!("unhandled default authentication, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:0");
            Ok(render_net_error(&err))
        }
    }
}

pub fn account_origin_invitation_accept(req: &mut Request) -> IronResult<Response> {
    let mut account_create = AccountCreate::new();
    {

        match req.get::<bodyparser::Struct<AccountCreateReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                account_create.set_token(token.to_string());
                account_create.set_extern_id(user.id);
                account_create.set_email(email);
                account_create.set_name(user.login);
                account_create.set_provider(OAuthProvider::PasswordAuth);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::account_create(&conn, &account_create) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn account_origin_create(req: &mut Request) -> IronResult<Response> {
    let mut account_get_by_id = AccountGet::new();
    {

        match req.get::<bodyparser::Struct<AccountGetReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `id`",
                    )));
                }
                account_get_by_id.set_id(id.to_string());
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::get_account_by_id(&conn, &account_get_by_id) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn account_origin_list_request(req: &mut Request) -> IronResult<Response> {
    let mut account_get = AccountGet::new();
    {

        match req.get::<bodyparser::Struct<AccountGetReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                account_create.set_id(id.to_string());
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::account_get(&conn, &account_get) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn account_invitation_list(req: &mut Request) -> IronResult<Response> {
    let mut session_get = SessionGet::new();
    {

        match req.get::<bodyparser::Struct<AccountGetReq>>() {
            Ok(Some(body)) => {
                if body.name.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `name`",
                    )));
                }
                session_get.set_token(token.to_string());
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::get_session(&conn, &session_get) {
        Ok(session) => Ok(render_json(status::Ok, &session)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
