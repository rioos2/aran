// Copyright 2018 The Rio Advancement Inc
//

use super::securer;
use api::security::config::SecurerConn;
use api::{Api, ApiValidator, ParmsVerifier, Validator};
use bodyparser;
use bytes::Bytes;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::{IdGet, MetaFields};
use protocol::api::schema::{dispatch, dispatch_url, type_meta};
use protocol::api::secret::Secret;
use router::Router;
use serde_json;
use service::models::secret;
use std::sync::Arc;

/// Securer api: SecurerApi provides ability to declare the node
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Secret: URLs supported are.
/// POST: /accounts/:account_id/secrets,,
/// GET: /accounts/:account_id/secrets,
/// GET: /secrets,
#[derive(Clone)]
pub struct SecretApi {
    conn: Box<DataStoreConn>,
    secret: Box<SecurerConn>,
}

impl SecretApi {
    pub fn new(datastore: Box<DataStoreConn>, secret: Box<SecurerConn>) -> Self {
        SecretApi {
            conn: datastore,
            secret: secret,
        }
    }
    //POST: /accounts/:account_id/secrets
    //The body has the input cluster::secrets
    //Returns a mutated Secret  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate::<Secret>(req.get::<bodyparser::Struct<Secret>>()?)?;

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            self.verify_account(req)?.get_name(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        let data = securer::from_config(&self.secret, Box::new(*self.conn.clone()))?;

        match data.secure(&securer::parse::parse_key(&unmarshall_body)?) {
            Ok(Some(secret)) => Ok(render_json(status::Ok, &secret)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //POST: /origins/:origin_id/secrets
    //The body has the input cluster::secrets
    //Returns a mutated Secret  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create_by_origin(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate::<Secret>(req.get::<bodyparser::Struct<Secret>>()?)?;

        debug!("✓ {}",
            format!("======= parsed {:?} ", unmarshall_body),
        );

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        let data = securer::from_config(&self.secret, Box::new(*self.conn.clone()))?;

        match data.secure(&securer::parse::parse_key(&unmarshall_body)?) {
            Ok(Some(secret)) => Ok(render_json(status::Ok, &secret)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /secrets/:id
    //Input id - u64 as input and returns a secrets
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match secret::DataStore::show(&self.conn, &params) {
            Ok(Some(secret)) => Ok(render_json(status::Ok, &secret)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /secrets/:id
    //Input id - u64 as input
    //Returns an secrets
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match secret::DataStore::show(&self.conn, &idget) {
            Ok(Some(secrets)) => {
                let data = json!({
                            "type": typ,
                            "data": secrets,      
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    //GET: /secrets
    //Blank origin: Returns all the secrets(irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        let data = securer::from_config(&self.secret, Box::new(*self.conn.clone()))?;

        match data.retrieve() {
            Ok(Some(service_list)) => {
                Ok(render_json_list(status::Ok, dispatch(req), &service_list))
            }
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /accounts/:account_id/secrets
    //Input origin_name Returns all the secrets (fpr that namespaces)
    //Every user will be able to list their own origin.
    //Will need roles/permission to access others origin.
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_account(req)?;

        let data = securer::from_config(&self.secret, Box::new(*self.conn.clone()))?;

        match data.retrieve_by(&params) {
            Ok(Some(secret)) => Ok(render_json_list(status::Ok, dispatch(req), &secret)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    //GET: /accounts/:account_id/secrets
    //Input origin_name Returns all the secrets (fpr that namespaces)
    //Every user will be able to list their own origin.
    //Will need roles/permission to access others origin.
    pub fn watch_list_by_account(&self, params: IdGet, dispatch: String) -> Option<String> {
        let data = match securer::from_config(&self.secret, Box::new(*self.conn.clone())) {
            Ok(result) => result,
            Err(_err) => return None,
        };
        let ident = dispatch_url(dispatch);
        match data.retrieve_by(&params) {
            Ok(Some(secret)) => {
                let res = json!({
                                "api_version": ident.version,
                                "kind": ident.kind,
                                "items": secret,
                });
                Some(serde_json::to_string(&res).unwrap())
            }
            Ok(None) => None,
            Err(_err) => None,
        }
    }

    //GET: /origins/:origin_id/secrets
    //Input origin_name Returns all the secrets (fpr that namespaces)
    //Every user will be able to list their own origin.
    //Will need roles/permission to access others origin.
    fn list_by_origin(&self, req: &mut Request) -> AranResult<Response> {
        let (org, name) = {
            let params = req.extensions.get::<Router>().unwrap();
            let org_name = params.find("origin_id").unwrap().to_owned();
            let ser_name = "".to_string();
            (org_name, ser_name)
        };

        debug!("✓ {}",
            format!("======= parsed {:?}{} ", org, name),
        );
        let mut params = IdGet::with_id(org.clone().to_string());
        params.set_name(name.clone().to_string());

        match secret::DataStore::list_by_origin(&self.conn, &params) {
            Ok(Some(secrets)) => Ok(render_json_list(status::Ok, dispatch(req), &secrets)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for account {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    //GET: /origin/:origin_name/secrets/:secrets_name
    //Input id - string as input and returns a secrets
    fn show_by_origin_and_name(&self, req: &mut Request) -> AranResult<Response> {
        let (org, name) = {
            let params = req.extensions.get::<Router>().unwrap();
            let org_name = params.find("origin").unwrap().to_owned();
            let set_name = params.find("secret_name").unwrap().to_owned();
            (org_name, set_name)
        };

        debug!("✓ {}",
            format!("======= parsed {:?}{} ", org, name),
        );
        let mut params = IdGet::with_id(name.clone().to_string());
        params.set_name(org.clone().to_string());

        match secret::DataStore::show_by_origin_and_name(&self.conn, &params) {
            Ok(Some(secrets)) => Ok(render_json(status::Ok, &secrets)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                name
            ))),
        }
    }
}

impl Api for SecretApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : secrets
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let create_by_origin =
            move |req: &mut Request| -> AranResult<Response> { _self.create_by_origin(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let list_by_origin =
            move |req: &mut Request| -> AranResult<Response> { _self.list_by_origin(req) };

        let _self = self.clone();
        let show_by_org_and_name =
            move |req: &mut Request| -> AranResult<Response> { _self.show_by_origin_and_name(req) };

        //secret API
        router.post(
            "/secrets",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "secrets",
        );

        //MEGAM
        //without authentication
        router.post(
            "/secrets/origins/:origin_id",
            XHandler::new(C {
                inner: create_by_origin,
            }),
            "secrets_by_origins",
        );
        /*router.get(
            "/secrets",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "secrets_list",
        );*/
        //TODO
        //without authentication

        router.get(
            "/secrets/all",
            XHandler::new(C { inner: list_blank }),
            "secrets_list",
        );
        router.get(
            "/secrets/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "secret_show",
        );
        router.get(
            "/secrets",
            XHandler::new(C { inner: list }).before(basic.clone()),
            "secret_show_by_account",
        );

        //MEGAM
        //without authentication
        router.get(
            "/secrets/origins/:origin_id",
            C {
                inner: list_by_origin,
            },
            "secret_show_by_origin",
        );

        //TODO enable with authentication
        /*router.get(
            "/origins/:origin/secrets/:secret_name",
            XHandler::new(C { inner: show_by_org_and_name }).before(basic.clone()),
            "secret_show_by_origin_name",
        );*/
        router.get(
            "/secrets/:secret_name/origins/:origin",
            C {
                inner: show_by_org_and_name,
            },
            "secret_show_by_origin_name",
        );
    }
}

impl ApiValidator for SecretApi {}

impl ParmsVerifier for SecretApi {}

impl Validator for Secret {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];
        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.get_secret_type().len() <= 0 {
            s.push("secret_type".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
