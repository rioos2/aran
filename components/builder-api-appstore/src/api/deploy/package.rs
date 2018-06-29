// Copyright 2018 The Rio Advancement Inc

//! A collection of package [package] for the HTTP server
use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, ParmsVerifier, Validator};
use protocol::api::schema::type_meta;

use config::Config;
use error::Error;
use error::ErrorMessage::MissingParameter;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};

use marketplace::package_ds;
use protocol::api::base::MetaFields;
use protocol::api::package::Package;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;

#[derive(Clone)]
pub struct PackageApi {
    conn: Box<DataStoreConn>,
}

/// Origin api: PackageApi provides ability to create sandboxed or spaces for
/// users.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Orgins: URLs supported are.
/// POST: /packages,
impl PackageApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        PackageApi { conn: datastore }
    }

    //POST: /packages
    //The body has the input packages
    //Returns a mutated packages with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    pub fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Package>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        match package_ds::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(package)) => Ok(render_json(status::Ok, &package)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for PackageApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        //Package Api
        router.post(
            "/packages",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "package",
        );
    }
}

impl ApiValidator for PackageApi {}

impl ParmsVerifier for PackageApi {}

//Validates parsed Packages from the body of the request.
//Checks for `....` in .....
//This is a NoOp for now.
impl Validator for Package {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_name().len() <= 0 {
            s.push("name".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
