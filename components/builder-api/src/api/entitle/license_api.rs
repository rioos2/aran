// Copyright 2018 The Rio Advancement Inc

use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, QueryValidator, Validator};
use bodyparser;
use entitle::models::license::DataStore;
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
use protocol::api::base::MetaFields;
use protocol::api::licenses::Licenses;
use protocol::api::schema::{type_meta,dispatch};
use router::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct LicenseApi {
    conn: Box<DataStoreConn>,
}

//
/// license: URLs supported are.
/// GET: /license/:name,

impl LicenseApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        LicenseApi {
            conn: datastore
        }
    }
    //POST: /license/activate
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate::<Licenses>(req.get::<bodyparser::Struct<Licenses>>()?)?;

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

        match DataStore::new(&self.conn).create_or_update(&unmarshall_body) {
            Ok(license) => Ok(render_json(status::Ok, &license)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /license/:name
    //Input id - u64 as input and returns a License
    fn show_by_name(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;

        match DataStore::new(&self.conn).license_show_by_name(&params) {
            Ok(Some(license)) => Ok(render_json(status::Ok, &license)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_name()
            ))),
        }
    }

    //Global: Returns all the license
    //GET: /license
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match DataStore::new(&self.conn).list_blank() {
            Ok(Some(licenses)) => Ok(render_json_list(status::Ok, dispatch(req), &licenses)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

}
impl Api for LicenseApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create_or_update = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show_by_name(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };


        router.get(
            "/license/:name",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "license_show_by_name",
        );

        router.post(
            "/license/activate",
            XHandler::new(C { inner: create_or_update }).before(basic.clone()),
            "license_create_or_update",
        );

        router.get(
            "/license",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "license_list",
        );

    }
}

impl ApiValidator for LicenseApi {}

impl ParmsVerifier for LicenseApi {}

impl QueryValidator for LicenseApi {}

impl Validator for Licenses {
    //default implementation is to check for `name` and 'node_ip'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }
        if self.get_status().len() <= 0 {
            s.push("status".to_string());
        }

        if self.get_activation_code().len() <= 0 {
            s.push("activation_code".to_string());
        }

        if self.get_product().len() <= 0 {
            s.push("product".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}