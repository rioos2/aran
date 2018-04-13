// Copyright 2018 The Rio Advancement Inc

//! A collection of marketplaces for deployment
//! MarketPlaceApi produces marketplaces which are blueprint for deployment.
//These are pre built recipes that a customer can use (ready to cook).

use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::path::Path;
use rio_core::fs::rioconfig_package_path;

use common::ui;
use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::{dispatch, type_meta};
use rio_net::http::static_handler::Static;

use config::Config;
use error::Error;
use error::ErrorMessage::MissingParameter;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

use marketplace::{marketplace_ds, package_attacher};

use protocol::api::marketplace::MarketPlace;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use protocol::api::base::MetaFields;

#[derive(Clone)]
pub struct MarketPlaceApi {
    conn: Box<DataStoreConn>,
}

/// MarketPlaceApi Api:  provides ability to manage prebuilt custom blueprint suites.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
// This isn't under any origin yet.
//
/// MarketPlaceApi
/// POST: /marketplace
/// GET: /marketplace/:id
/// GET: storagepools PUT: storagepools/status_update
impl MarketPlaceApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        MarketPlaceApi { conn: datastore }
    }

    //POST: /marketplace
    //The body has the input Plan
    //Returns a mutated Plan with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<MarketPlace>>()?)?;

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
        match marketplace_ds::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(Some(marketplace)) => Ok(render_json(status::Ok, &marketplace)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /marketplaces
    //Blank origin: Returns all the MarketPlaces (irrespective of namespaces)
    //Will need roles/permission to access this.
    fn list_blank(&self, _req: &mut Request) -> AranResult<Response> {
        match marketplace_ds::DataStore::new(&self.conn).list_blank() {
            Ok(Some(marketplaces)) => Ok(render_json_list(status::Ok, dispatch(_req), &marketplaces)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /marketplace/:id
    //Input id - u64 as input and returns a MarketPlaces
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match marketplace_ds::DataStore::new(&self.conn).show(&params) {
            Ok(Some(marketplace)) => Ok(render_json(status::Ok, &marketplace)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /marketplace/:id/download
    //Input id - u64 as input and returns a dowload url
    fn download(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        match package_attacher::PackageAttacher::new(&self.conn, &params).get_package() {
            Ok(Some(package)) => {
                match Static::new(Path::new(
                    &rioconfig_package_path(None).join(package.get_url()),
                )).get(req) {
                    Ok(path) => Ok(path),
                    Err(err) => Err(internal_error(&format!("{}\n", err))),
                }
            }
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
}

impl Api for MarketPlaceApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures:marketplace
        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list_blank = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let download = move |req: &mut Request| -> AranResult<Response> { _self.download(req) };

        router.post(
            "/marketplaces",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "markets",
        );

        router.get(
            "/marketplaces",
            XHandler::new(C { inner: list_blank }).before(basic.clone()),
            "market_list",
        );

        router.get(
            "/marketplaces/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "market_show",
        );

        router.get(
            "/marketplaces/:id/download",
            XHandler::new(C { inner: download }).before(basic.clone()),
            "market_download",
        );
    }
}

impl ApiValidator for MarketPlaceApi {}

impl ParmsVerifier for MarketPlaceApi {}

//Validates parsed MarketPlace from the body of the request.
//Checks for `....` in ...
//This is NoOp for now.
impl Validator for MarketPlace {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_category().len() <= 0 {
            s.push("category".to_string());
        }
        if self.get_version().len() <= 0 {
            s.push("version".to_string());
        }
        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
