// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [accounts, login, teams, permissions,] for the HTTP server


use api::{Api, ApiValidator, ParmsVerifier, Validator};
use auth::rioos::AuthenticateDelegate;
use auth::rioos::user_account::UserAccountAuthenticate;
use auth::util::authenticatable::Authenticatable;
use bodyparser;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, conflict_error, internal_error, not_found_error, unauthorized_error};
use iron::prelude::*;
use iron::status;
use protocol::api::base::MetaFields;
use protocol::api::schema::type_meta;
use protocol::api::session::*;
use rand;
use router::Router;
use session::models::session as sessions;
use std::sync::Arc;

const DEFAULTTEAM: &'static str = "rioos:loneranger";

#[derive(Clone)]
pub struct AuthenticateApi {
    conn: Box<DataStoreConn>,
}

/// Authenticate api: AuthenticateyApi provides ability to authenticate the user.
/// Needs a DataStore mapper, hence a DataStoreConn needs to be sent in.
//
/// Authentication: URLs supported are.
/// POST: /authenticate,
/// GET: /account/id
/// Record the other apis here.

impl AuthenticateApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        AuthenticateApi { conn: datastore }
    }

    //POST: /authenticate
    //Input the user credentials - (default password authentication)
    //The body contains email, password, authenticate and if all is well return a token.
    fn default_authenticate(&self, req: &mut Request) -> AranResult<Response> {
        let account = self.validate(req.get::<bodyparser::Struct<AccountGet>>()?)?;
        let delegate = AuthenticateDelegate::new(Arc::new(*self.conn.clone()));
        let auth = Authenticatable::UserAndPass {
            username: account.get_email(),
            password: account.get_password(),
        };

        match delegate.authenticate(&auth) {
            Ok(_validate) => {
                let mut account_data = Account::new();
                account_data.set_email(account.get_email());
                account_data.set_password(account.get_password());

                let mut session_data = Session::new();
                session_data.set_token(UserAccountAuthenticate::token().unwrap());

                let mut device: Device = user_agent(req).into();
                device.set_ip(format!("{}", req.remote_addr.ip()));

                session_data.set_device(device);

                match sessions::DataStore::find_account(&self.conn, &account_data, &session_data) {
                    Ok(session) => Ok(render_json(status::Ok, &session)),
                    Err(err) => Err(internal_error(&format!("{}", err))),
                }
            }
            Err(e) => Err(unauthorized_error(&format!("{}", e))),
        }
    }

    //POST: accounts",
    //Input account and creates an user, by returning the Account information of an user
    fn account_create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Account>>()?)?;

        if unmarshall_body.get_apikey().len() <= 0 {
            unmarshall_body.set_apikey(rand::random::<u64>().to_string());
        }

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_email(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        let en = unmarshall_body.get_password();
        unmarshall_body.set_password(UserAccountAuthenticate::encrypt(en).unwrap());

        let mut account_get = AccountGet::new();
        account_get.set_email(unmarshall_body.get_email());

        let mut session_data = Session::new();
        session_data.set_token(UserAccountAuthenticate::token().unwrap());

        let mut device: Device = user_agent(req).into();
        device.set_ip(format!("{}", req.remote_addr.ip()));

        session_data.set_device(device);

        match sessions::DataStore::get_account(&self.conn, &account_get) {
            Ok(Some(_account)) => Err(conflict_error(
                &format!("alreay exists {}", account_get.get_email()),
            )),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => {
                match sessions::DataStore::account_create(&self.conn, &unmarshall_body, &session_data) {
                    Ok(account) => Ok(render_json(status::Ok, &account)),
                    Err(err) => Err(internal_error(&format!("{}", err))),
                }
            }
        }
    }

    //GET: accounts/:id",
    //Input id, and returns the Account information of an user
    fn account_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match sessions::DataStore::get_account_by_id(&self.conn, &params) {
            Ok(Some(account)) => Ok(render_json(status::Ok, &account)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: accounts/:name",
    //Input name, and returns the Account information of an user
    //Change it by building a NameParmsVerifier
    fn account_show_by_name(&self, req: &mut Request) -> AranResult<Response> {
        let name = {
            let params = req.extensions.get::<Router>().unwrap();
            let name = params.find("name").unwrap().to_owned();
            name
        };
        let mut account_get = AccountGet::new();
        account_get.set_email(name);

        match sessions::DataStore::get_account(&self.conn, &account_get) {
            Ok(Some(account)) => Ok(render_json(status::Ok, &account)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &account_get.get_email()
            ))),
        }
    }

    //POST: /logout
    //Global: Logout the current user
    fn account_logout(&self, req: &mut Request) -> AranResult<Response> {
        let account = self.validate(
            req.get::<bodyparser::Struct<AccountTokenGet>>()?,
        )?;

        let mut device: Device = user_agent(req).into();
        device.set_ip(format!("{}", req.remote_addr.ip()));

        match sessions::DataStore::account_logout(&self.conn, &account, &device) {
            Ok(Some(account)) => Ok(render_json(status::Ok, &account)),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }
}

impl Api for AuthenticateApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : scaling
        let _self = self.clone();
        let account_create = move |req: &mut Request| -> AranResult<Response> { _self.account_create(req) };

        let _self = self.clone();
        let account_show = move |req: &mut Request| -> AranResult<Response> { _self.account_show(req) };

        let _self = self.clone();
        let account_show_by_name = move |req: &mut Request| -> AranResult<Response> { _self.account_show_by_name(req) };

        let _self = self.clone();
        let authenticate = move |req: &mut Request| -> AranResult<Response> { _self.default_authenticate(req) };

        let _self = self.clone();
        let account_logout = move |req: &mut Request| -> AranResult<Response> { _self.account_logout(req) };

        router.post(
            "/accounts",
            XHandler::new(C { inner: account_create }),
            "account_create:signup",
        );
        router.get(
            "/accounts/:id",
            XHandler::new(C { inner: account_show }).before(basic.clone()),
            "account_show",
        );

        router.get(
            "/accounts/name/:name",
            XHandler::new(C { inner: account_show_by_name }).before(basic.clone()),
            "account_show_by_name",
        );

        router.post(
            "/authenticate",
            XHandler::new(C { inner: authenticate }),
            "authenticate",
        );
        router.post(
            "/logout",
            XHandler::new(C { inner: account_logout }).before(basic.clone()),
            "account_logout",
        );
    }
}

impl ApiValidator for AuthenticateApi {}

impl ParmsVerifier for AuthenticateApi {}

impl Validator for AccountGet {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
impl Validator for AccountTokenGet {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_email().len() <= 0 {
            s.push("email".to_string());
        }
        if self.get_token().len() <= 0 {
            s.push("token".to_string());
        }
        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for Account {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_email().len() <= 0 {
            s.push("email".to_string());
        }
        if self.get_password().len() <= 0 {
            s.push("password".to_string());
        }
        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for SessionGet {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
