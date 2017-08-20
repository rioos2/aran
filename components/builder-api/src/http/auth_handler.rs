// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [accounts, login, roles, permissions,] for the HTTP server

use bodyparser;
use rio_core::event::*;
use rio_net::http::controller::*;
use session::session_ds::SessionDS;

use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;
use protocol::net::{self, ErrCode};
use protocol::sessionsrv::*;

use db::data_store::DataStoreBroker;


define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SessionCreateReq {
    name: String,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
    api_key: String,
    password: String,
    states: String,
    approval: String,
    suspend: String,
    registration_ip_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AccountGetReq {
    id: String,
    name: String,
    email: String,
    token: String,
}

//Default password authentication.
//The body contains email, password, authenticate and if all is well return a token.
pub fn default_authenticate(req: &mut Request) -> IronResult<Response> {
    let mut session_data = SessionCreate::new();
    {

        match req.get::<bodyparser::Struct<SessionCreateReq>>() {
            Ok(Some(body)) => {
                if body.email.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `email`",
                    )));
                }

                if body.password.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `password`",
                    )));
                }

                session_data.set_email(body.email);
                session_data.set_password(body.password);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let authcli = req.get::<persistent::Read<PasswordAuthCli>>().unwrap();
    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();

    //make sure authenticate returns an account.
    let mut account_get: AccountGet = AccountGet::new();

    account_get.set_email(session_data.get_email().to_string());
    account_get.set_password(session_data.get_password().to_string());

    match authcli.authenticate(&conn, &account_get) {
        Ok(account) => {
            session_data.set_token(account.get_token());

            let session = try!(session_create(&conn, &session_data));

            log_event!(
                req,
                Event::PasswordAuthenticate {
                    user: session.get_name().to_string(),
                    account: session.get_id().to_string(),
                }
            );

            Ok(render_json(status::Ok, &session))
        }
        Err(e) => {
            error!("unhandled password authentication, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:0");
            Ok(render_net_error(&err))
        }
    }
}

pub fn account_create(req: &mut Request) -> IronResult<Response> {
    let mut account_create = SessionCreate::new();
    {

        match req.get::<bodyparser::Struct<SessionCreateReq>>() {
            Ok(Some(body)) => {
                if body.email.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `email`",
                    )));
                }

                if body.api_key.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `api_key`",
                    )));
                }

                //Don't know if this a good way to do so as why should PasswordAuthCli
                //act as token generator
                let authcli = req.get::<persistent::Read<PasswordAuthCli>>().unwrap();
                let email = body.email.to_string();

                account_create.set_token(authcli.token().unwrap());
                account_create.set_name(body.name.to_string());
                account_create.set_email(email.clone());
                account_create.set_first_name(body.first_name);
                account_create.set_last_name(body.last_name);
                account_create.set_phone(body.phone);
                account_create.set_apikey(body.api_key);
                account_create.set_password(authcli.encrypt(email.clone(),body.password).unwrap());
                account_create.set_states(body.states);
                account_create.set_approval(body.approval);
                account_create.set_suspend(body.suspend);
                account_create.set_registration_ip_address(body.registration_ip_address);
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();

    match SessionDS::account_create(&conn, &account_create) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn account_get_by_id(req: &mut Request) -> IronResult<Response> {
    let mut account_get_by_id = AccountGetId::new();
    {

        match req.get::<bodyparser::Struct<AccountGetReq>>() {
            Ok(Some(body)) => {
                if body.id.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `id`",
                    )));
                }
                account_get_by_id.set_id(body.id.to_string());
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();
    //This is needed as you'll need the email/token if any

    match SessionDS::get_account_by_id(&conn, &account_get_by_id) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn account_get(req: &mut Request) -> IronResult<Response> {
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
                account_get.set_name(body.name.to_string());
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();
    //This is needed as you'll need the email/token if any

    match SessionDS::get_account(&conn, &account_get) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn session_get(req: &mut Request) -> IronResult<Response> {
    let mut session_get = SessionGet::new();
    {

        match req.get::<bodyparser::Struct<AccountGetReq>>() {
            Ok(Some(body)) => {
                if body.email.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `email`",
                    )));
                }
                if body.token.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `token`",
                    )));
                }
                session_get.set_email(body.email.to_string());
                session_get.set_token(body.token.to_string());
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = req.get::<persistent::Read<DataStoreBroker>>().unwrap();
    //This is needed as you'll need the email/token if any

    match SessionDS::get_session(&conn, &session_get) {
        Ok(session) => Ok(render_json(status::Ok, &session)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}
