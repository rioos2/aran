use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use common::ui;
use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::{dispatch, type_meta};
use config::Config;
use error::Error;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};
use protocol::api::base::MetaFields;

use deploy::models::service;
use protocol::api::linker::Services;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;
use bytes::Bytes;
use serde_json;
use protocol::api::base::IdGet;

#[derive(Clone)]
pub struct ServiceApi {
    conn: Box<DataStoreConn>,
}

/// Storage api
/// - every instance of StoragesApi needs a DataStoreConn
/// StorageConnector
/// POST: storageconnectors, GET: storageconnectors/:id, GET: storageconnectors PUT: storageconnectors/status_update
/// StoragePool
/// POST: storagepools, GET: storagepools/:id, GET: storagepools PUT: storagepools/status_update
/// Datacenters
/// POST: datacenters, GET: datacenters/:id, GET: datacenters PUT: datacenters/status_update
impl ServiceApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        ServiceApi { conn: datastore }
    }

    //POST: /services
    //The body has the input Service
    //Returns a mutated Service with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Services>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        ui::rawdumpln(
            Colour::White,
            '✓',
            format!("======= parsed {:?} ", unmarshall_body),
        );

        match service::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(services) => Ok(render_json(status::Ok, &services)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /services/:id
    //Input id - u64 as input and returns a Service
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match service::DataStore::show(&self.conn, &params) {
            Ok(Some(end)) => Ok(render_json(status::Ok, &end)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /services/:id
    //Input id - u64 as input and returns a Service
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        let res = match service::DataStore::show(&self.conn, &idget) {
            Ok(Some(end)) => {
                let data = json!({
                            "type": typ,
                            "data": end,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }

    //GET: /services
    //Blank origin: Returns all the Services (irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match service::DataStore::list_blank(&self.conn) {
            Ok(Some(linkers)) => Ok(render_json_list(status::Ok, dispatch(_req), &linkers)),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //PUT: /services/:id
    //Input services id and returns updated service
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Services>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match service::DataStore::update(&self.conn, &unmarshall_body) {
            Ok(Some(service)) => Ok(render_json(status::Ok, &service)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }
}

impl Api for ServiceApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        router.post(
            "/services",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "services",
        );

        router.get(
            "/services/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "service_show",
        );
        router.get(
            "/services",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "service_list_blank",
        );

        router.put(
            "/services/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "service_update",
        );
    }
}

impl ApiValidator for ServiceApi {}

impl ParmsVerifier for ServiceApi {}

//Validates parsed Services from the body of the request.
//Checks for `...` in .....
//This is a NoOp for now.

/*This validator applies to Service.update. The service creation is handled by api_server and scheduler.
Whoever creates a service shall send the status as well.*/
impl Validator for Services {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.object_meta().account.len() <= 0 {
            s.push("account".to_string());
        }

        if self.get_status().phase.len() <= 0 {
            s.push("phase".to_string());
        }

        if self.object_meta().owner_references.len() <= 0 {
            s.push("owner_references".to_string());
        } else {
            self.object_meta()
                .owner_references
                .iter()
                .map(|x| if x.uid.len() <= 0 {
                    s.push("uid".to_string());
                })
                .collect::<Vec<_>>();
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
