use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, Validator};
use bodyparser;
use bytes::Bytes;
use common::ui;
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
use protocol::api::schema::{dispatch, type_meta};
use protocol::api::service_account::ServiceAccount;
use router::Router;
use serde_json;
use service::service_account_ds::ServiceAccountDS;
use std::sync::Arc;

const SERVICEACCOUNTDEFAULT: &'static str = "rioos:universalsoldier";

/// Securer api: SecurerApi provides ability to declare the node
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Secret: URLs supported are.
/// POST: /accounts/:account_id/secrets,,
/// GET: /accounts/:account_id/secrets,
/// GET: /secrets,
#[derive(Clone)]
pub struct SeriveAccountApi {
    conn: Box<DataStoreConn>,
}

impl SeriveAccountApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        SeriveAccountApi { conn: datastore }
    }

    //POST: /origins/:origin_id/serviceaccount

    //The body has the input cluster::serviceaccount
    //Returns a mutated ServiceAccount  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<ServiceAccount>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        unmarshall_body.set_roles(vec![SERVICEACCOUNTDEFAULT.to_string()]);

        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match ServiceAccountDS::create(&self.conn, &unmarshall_body) {
            Ok(Some(service)) => Ok(render_json(status::Ok, &service)),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }
    //GET: /serviceaccount
    //Blank origin: Returns all the serviceaccount(irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match ServiceAccountDS::list_blank(&self.conn) {
            Ok(Some(service_list)) => {
                Ok(render_json_list(status::Ok, dispatch(req), &service_list))
            }
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /serviceaccount/:id
    //Input id - u64 as input and returns a serviceaccount
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let name = {
            let params = req.extensions.get::<Router>().unwrap();
            let ser_name = params.find("serviceaccount").unwrap().to_owned();
            ser_name
        };

        ui::rawdumpln(Colour::White, '✓', format!("======= parsed {:?} ", name));
        match ServiceAccountDS::show(&self.conn, &IdGet::with_id(name.clone().to_string())) {
            Ok(Some(origin)) => Ok(render_json(status::Ok, &origin)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                name
            ))),
        }
    }

    ///PUT: /origins/:origin_id/serviceaccount/:serviceaccount
    ///Input: new updated secret
    fn secret_update(&self, req: &mut Request) -> AranResult<Response> {
        let (_org, name) = {
            let params = req.extensions.get::<Router>().unwrap();
            let org_name = params.find("origin_id").unwrap().to_owned();
            let ser_name = params.find("serviceaccount").unwrap().to_owned();
            (org_name, ser_name)
        };

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<ServiceAccount>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        match ServiceAccountDS::update(&self.conn, &unmarshall_body) {
            Ok(Some(serviceaccount)) => Ok(render_json(status::Ok, &serviceaccount)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                name.clone()
            ))),
        }
    }

    //GET: /serviceaccount/:id
    //Input id - u64 as input
    //Returns an serviceaccount
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        //self.with_cache();
        let res = match ServiceAccountDS::show_by_id(&self.conn, &idget) {
            Ok(Some(sa)) => {
                let data = json!({
                            "type": typ,
                            "data": sa,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for SeriveAccountApi {
    fn wire(&mut self, _config: Arc<Config>, router: &mut Router) {
        let _basic = Authenticated::new(&*_config);

        //closures : secrets
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let show_by_origin = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let secret_update =
            move |req: &mut Request| -> AranResult<Response> { _self.secret_update(req) };

        //serviceAccount API
        router.post(
            "/origins/:origin_id/serviceaccounts",
            XHandler::new(C { inner: create }),
            "service_accounts",
        );
        router.get(
            "/serviceaccounts",
            C { inner: list_blank },
            "service_account_list",
        );
        router.get(
            "/origins/:origin_id/serviceaccounts/:serviceaccount",
            C {
                inner: show_by_origin,
            },
            "service_account_get_by_origin",
        );

        router.put(
            "/origins/:origin_id/serviceaccounts/:serviceaccount",
            C {
                inner: secret_update,
            },
            "service_account_secret_update",
        );

        router.get(
            "/serviceaccounts/:serviceaccount",
            C { inner: show },
            "service_account_get",
        );
    }
}

impl ApiValidator for SeriveAccountApi {}

impl ParmsVerifier for SeriveAccountApi {}

impl Validator for ServiceAccount {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
