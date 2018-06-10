// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment declaration api assembly_factory
use ansi_term::Colour;
use api::{Api, ApiValidator, ParmsVerifier, Validator};
use bodyparser;
use common::ui;
use config::Config;
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use devtooling::models::image_marks;
use error::Error;
use error::ErrorMessage::MissingParameter;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
use iron::prelude::*;
use iron::status;
use protocol::api::base::MetaFields;
use protocol::api::devtool::ImageMarks;
use protocol::api::schema::{dispatch, type_meta};
use router::Router;
use std::sync::Arc;

#[derive(Clone)]
pub struct ImageMarksApi {
    conn: Box<DataStoreConn>,
}

/// ImageMarks API:
///
/// URL:
/// POST:/imagemarks
/// GET:/imagemarks
/// GET: /imagemarks/:id,
/// PUT: /imagemarks/:id,
//GET: /imagemarks/builds/:id

impl ImageMarksApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        ImageMarksApi { conn: datastore }
    }

    //POST: /imagemarks
    //Input: Body of structure devtooling::ImageMarksApi
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body =
            self.validate::<ImageMarks>(req.get::<bodyparser::Struct<ImageMarks>>()?)?;

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

        match image_marks::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(image) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    ///GET: /imagemarks/:id
    ///Input: id - u64
    ///Returns ImageMarks
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match image_marks::DataStore::show(&self.conn, &params) {
            Ok(Some(image)) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
    //GET: /imagemarks
    //Every user will be able to list their imagemarks
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        match image_marks::DataStore::list(&self.conn) {
            Ok(Some(image)) => Ok(render_json_list(status::Ok, dispatch(req), &image)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    ///PUT: /imagemarks/:id
    ///Input imagemarks id
    ///Returns updated imagemarks
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<ImageMarks>>()?)?;

        unmarshall_body.set_id(params.get_id());
        match image_marks::DataStore::update(&self.conn, &unmarshall_body) {
            Ok(Some(image)) => Ok(render_json(status::Ok, &image)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /imagemarks/builds/:id
    //Input build id Returns list of imagemarks
    fn list_by_build(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match image_marks::DataStore::list_by_build(&self.conn, &params) {
            Ok(Some(image)) => Ok(render_json_list(status::Ok, dispatch(req), &image)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }
}
///The Api wirer for ImageMarksApi
///Add all the api needed to be supported under `/imagemarks`
///To add an api refer, comments in Api trait.
impl Api for ImageMarksApi {
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
        let list_by_build =
            move |req: &mut Request| -> AranResult<Response> { _self.list_by_build(req) };

        router.post(
            "/imagemarks",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "image_marks",
        );

        router.get(
            "/imagemarks/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "image_marks_show",
        );
        router.get(
            "/imagemarks",
            XHandler::new(C { inner: list }).before(basic.clone()),
            "image_marks_list",
        );

        router.put(
            "/imagemarks/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "image_marks_update",
        );

        router.get(
            "/imagemarks/builds/:id",
            XHandler::new(C {
                inner: list_by_build,
            }).before(basic.clone()),
            "image_marks_list_by_build",
        );
    }
}

///Convinient helpers to validating an api
impl ApiValidator for ImageMarksApi {}

///Convinient helpers to verify any api
impl ParmsVerifier for ImageMarksApi {}

///Called by implementing ApiValidator when validate() is invoked with the parsed body
///Checks for required parameters in the parsed struct image marks
impl Validator for ImageMarks {
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

        Err(bad_request(&MissingParameter(format!(
            "{:?} -> {}",
            s, "must have => "
        ))))
    }
}
