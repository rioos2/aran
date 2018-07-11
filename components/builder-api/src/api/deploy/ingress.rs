// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [origin] for the HTTP server
use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
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
use deploy::models::ingress::DataStore;
use protocol::api::base::{IdGet, MetaFields};
use protocol::api::ingress::{Ingress,StatusUpdate};
use protocol::api::schema::{dispatch, type_meta};
use router::Router;
use serde_json;
use std::sync::Arc;

#[derive(Clone)]
pub struct IngressApi {
    conn: Box<DataStoreConn>,
}

/// Ingress api
/// - every instance of IngressApi needs a DataStoreConn
impl IngressApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        IngressApi { conn: datastore }
    }

    //POST: /ingress
    //The body has the input ingress
    //Returns a mutated Ingress with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    //Will need roles/permission to access others ingress
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Ingress>>()?)?;
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

        match DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(ingress)) => Ok(render_json(status::Ok, &ingress)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //PUT: /ingress/:id/status
    //Input status  as input and returns an updated Ingress
    //Will need roles/permission to access others ingress
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<StatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match DataStore::new(&self.conn).status_update(&unmarshall_body) {
            Ok(Some(ingress)) => Ok(render_json(status::Ok, &ingress)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: /assemblyfactorys/:id/ingress
    //Input assembly factory id Returns ingress
    //Will need roles/permission to access others ingress
    fn show_by_assembly_factory(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match DataStore::new(&self.conn).show_by_assembly_factory(&params) {
            Ok(Some(ingress)) => Ok(render_json(status::Ok, &ingress)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //PUT: /ingress/:id
    //Input ingress id and returns updated ingress
    //Will need roles/permission to access others ingress
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body =
            self.validate(req.get::<bodyparser::Struct<Ingress>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match DataStore::new(&self.conn).update(&unmarshall_body) {
            Ok(Some(ingress)) => Ok(render_json(status::Ok, &ingress)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: /ingress/:id
    //Input id - u64 as input
    //Returns an ingress
    pub fn watch(&mut self, idget: IdGet, typ: String) -> Bytes {
        let res = match DataStore::new(&self.conn).show(&idget) {
            Ok(Some(ingress)) => {
                let data = json!({
                            "type": typ,
                            "data": ingress,
                            });
                serde_json::to_string(&data).unwrap()
            }
            _ => "".to_string(),
        };
        Bytes::from(res)
    }
}

impl Api for IngressApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let show_by_assembly_factory =
            move |req: &mut Request| -> AranResult<Response> { _self.show_by_assembly_factory(req) };

        router.post(
            "/ingress",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "ingress_create",
        );

        router.put(
            "ingress/:id",
            XHandler::new(C {
                inner: update,
            }).before(basic.clone()),
            "ingress_update",
        );
        router.put(
            "/ingress/:id/status",
            XHandler::new(C {
                inner: status_update,
            }).before(basic.clone()),
            "ingress_status_update",
        );
        router.get(
            "/assemblyfactorys/:id/ingress",
            XHandler::new(C {
                inner: show_by_assembly_factory,
            }).before(basic.clone()),
            "ingress_show_by_assembly_factory",
        );

    }
}

impl ApiValidator for IngressApi {}

impl ParmsVerifier for IngressApi {}

impl QueryValidator for IngressApi {}

//Validates parsed ingress from the body of the request.
//Checks for ....
impl Validator for Ingress {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.object_meta().account.len() <= 0 {
            s.push("account".to_string());
        }

        if self.object_meta().owner_references.len() < 2 {
            s.push("owner_references".to_string());
        } else {
            self.object_meta()
            .owner_references
            .iter()
            .map(|x| {
                if x.uid.len() <= 0 {
                    s.push("uid".to_string());
                        }
                    })
            .collect::<Vec<_>>();
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for StatusUpdate {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
