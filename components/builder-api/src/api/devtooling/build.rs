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


use protocol::api::devtool::{Build, BuildStatusUpdate};
use devtooling::models::build;

use protocol::api::base::MetaFields;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;


#[derive(Clone)]
pub struct BuildApi {
    conn: Box<DataStoreConn>,
}

/// Build API:
///
/// URL:
/// POST:/builds
/// GET: /builds/:id,
/// GET: /builds/buildconfig/:id
/// GET: /builds

impl BuildApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        BuildApi { conn: datastore }
    }

    //POST: /builds
    //Input: Body of structure deploy::Build
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate::<Build>(
            req.get::<bodyparser::Struct<Build>>()?,
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

        match build::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(build) => Ok(render_json(status::Ok, &build)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    ///GET: /builds/:id
    ///Input: id - u64
    ///Returns Build
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match build::DataStore::show(&self.conn, &params) {
            Ok(Some(build)) => Ok(render_json(status::Ok, &build)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /builds
    //Every user will be able to list their builds
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        match build::DataStore::list(&self.conn) {
            Ok(Some(build)) => Ok(render_json_list(status::Ok, dispatch(req), &build)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /builds/buildconfig/:id
    //Input build config id Returns show of build
    fn show_by_build_config(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match build::DataStore::show_by_build_config(&self.conn, &params) {
            Ok(Some(build)) => Ok(render_json(status::Ok, &build)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    ///PUT: /builds/:id
    ///Input builds id
    ///Returns updated builds
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Build>>()?)?;

        unmarshall_body.set_id(params.get_id());
        match build::DataStore::update(&self.conn, &unmarshall_body) {
            Ok(Some(build)) => Ok(render_json(status::Ok, &build)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //PUT: /builds/id/status
    //Input status  as input and returns an updated builds
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<BuildStatusUpdate>>()?,
        )?;
        unmarshall_body.set_id(params.get_id());

        match build::DataStore::status_update(&self.conn, &unmarshall_body) {
            Ok(Some(jobs)) => Ok(render_json(status::Ok, &jobs)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }
}

///The Api wirer for BuildApi
///Add all the api needed to be supported under `/builds`
///To add an api refer, comments in Api trait.
impl Api for BuildApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : builds
        let mut _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let show_by_build_config = move |req: &mut Request| -> AranResult<Response> { _self.show_by_build_config(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        router.post(
            "/builds",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "build",
        );

        router.get(
            "/builds",
            XHandler::new(C { inner: list }).before(basic.clone()),
            "build_list",
        );
        router.get(
            "/builds/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "build_show",
        );
        router.get(
            "/builds/buildconfig/:id",
            XHandler::new(C { inner: show_by_build_config }).before(basic.clone()),
            "build_list_by_buildconfig",
        );
        router.put(
            "/builds/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "builds_update",
        );
        router.put(
            "/builds/:id/status",
            XHandler::new(C { inner: status_update }).before(basic.clone()),
            "builds_status_update",
        );
    }
}

///Convinient helpers to validating an api
impl ApiValidator for BuildApi {}

///Convinient helpers to verify any api
impl ParmsVerifier for BuildApi {}

///Called by implementing ApiValidator when validate() is invoked with the parsed body
///Checks for required parameters in the parsed struct Build
impl Validator for Build {
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

impl Validator for BuildStatusUpdate {
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
