// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment declaration api assembly_factory
use std::sync::Arc;

use ansi_term::Colour;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use common::ui;
use config::Config;

use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::{dispatch, type_meta};

use error::Error;
use error::ErrorMessage::MissingParameter;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};


use protocol::api::devtool::ImageReferences;
use devtooling::models::image_references;

use protocol::api::base::MetaFields;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;


#[derive(Clone)]
pub struct ImageReferencesApi {
    conn: Box<DataStoreConn>,
}

/// ImageRef API:
///
/// URL:
/// POST:/imagereferences,
/// GET:/imagereferences,
/// GET: /imagereferences/:id,
/// PUT: /imagereferences/:id,
/// PUT: /imagereferences/build_configs/:id,


impl ImageReferencesApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        ImageReferencesApi { conn: datastore }
    }

    //POST: /imagereferences
    //Input: Body of structure devtooling::ImageReferences
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate::<ImageReferences>(
            req.get::<bodyparser::Struct<ImageReferences>>()?,
        )?;

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

        match image_references::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(image) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    ///GET: /imagereferences/:id
    ///Input: id - u64
    ///Returns ImageReferences
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match image_references::DataStore::show(&self.conn, &params) {
            Ok(Some(image)) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /imagereferences
    //Every user will be able to list their imagereferences
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        match image_references::DataStore::list(&self.conn) {
            Ok(Some(image)) => Ok(render_json_list(status::Ok, dispatch(req), &image)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    ///PUT: /imagereferences/:id
    ///Input imagereferences id
    ///Returns updated imagereferences
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<ImageReferences>>()?,
        )?;

        unmarshall_body.set_id(params.get_id());
        match image_references::DataStore::update(&self.conn, &unmarshall_body) {
            Ok(Some(image)) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
    ///GET: /imagereferences/build_configs/:id
    ///Returns ImageReferences
    fn show_by_build_config(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match image_references::DataStore::show_by_build_config(&self.conn, &params) {
            Ok(Some(image)) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
}
///The Api wirer for ImageReferencesApi
///Add all the api needed to be supported under `/imageref`
///To add an api refer, comments in Api trait.
impl Api for ImageReferencesApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : ImageRef
        let mut _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show_by_build_config = move |req: &mut Request| -> AranResult<Response> { _self.show_by_build_config(req) };

        router.post(
            "/imagereferences",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "image_ref",
        );

        router.get(
            "/imagereferences/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "image_ref_show",
        );

        router.get(
            "/imagereferences",
            XHandler::new(C { inner: list }).before(basic.clone()),
            "image_references_list",
        );

        router.put(
            "/imagereferences/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "image_references_update",
        );

        router.get(
            "/imagereferences/buildconfigs/:id",
            XHandler::new(C { inner: show_by_build_config }).before(basic.clone()),
            "image_references_show_by_build_config",
        );
    }
}

///Convinient helpers to validating an api
impl ApiValidator for ImageReferencesApi {}

///Convinient helpers to verify any api
impl ParmsVerifier for ImageReferencesApi {}

///Called by implementing ApiValidator when validate() is invoked with the parsed body
///Checks for required parameters in the parsed struct ImageRef
impl Validator for ImageReferences {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        if self.object_meta().owner_references.len() < 1 {
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

        Err(bad_request(
            &MissingParameter(format!("{:?} -> {}", s, "must have => ")),
        ))
    }
}
