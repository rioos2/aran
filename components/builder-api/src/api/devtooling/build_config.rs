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
use protocol::api::schema::{dispatch, type_meta};

use error::Error;
use error::ErrorMessage::MissingParameter;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};


use protocol::api::devtool::BuildConfig;
use devtooling::models::build_config;

use protocol::api::base::{MetaFields, StatusUpdate};

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;


#[derive(Clone)]
pub struct BuildConfigApi {
    conn: Arc<DataStoreConn>,
}

/// BuildConfig API:
///
/// URL:
/// POST:/buildconfig
/// GET: /buildconfig/:id,
/// GET: /buildconfig/assemblyfactory/:id
/// GET: /buildconfig

impl BuildConfigApi {
    pub fn new(datastore: Arc<DataStoreConn>) -> Self {
        BuildConfigApi { conn: datastore }
    }

    //POST: /buildconfig
    //Input: Body of structure deploy::BuildConfig
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate::<BuildConfig>(
            req.get::<bodyparser::Struct<BuildConfig>>()?,
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

        match build_config::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(build_config) => Ok(render_json(status::Ok, &build_config)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    ///GET: /buildconfig/:id
    ///Input: id - u64
    ///Returns BuildConfig
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match build_config::DataStore::show(&self.conn, &params) {
            Ok(Some(build_conf)) => Ok(render_json(status::Ok, &build_conf)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /buildconfig
    //Every user will be able to list their build configs
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        match build_config::DataStore::list(&self.conn) {
            Ok(Some(buildconf)) => Ok(render_json_list(status::Ok, dispatch(req), &buildconf)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /buildconfig/assemblyfactorys/:id
    //Input assemblyfactory_id Returns show of build_config
    fn show_by_assemblyfactory(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match build_config::DataStore::show_by_assemblyfactory(&self.conn, &params) {
            Ok(Some(buildconf)) => Ok(render_json(status::Ok, &buildconf)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    ///PUT: /buildconfigs/:id
    ///Input buildconfigs id
    ///Returns updated buildconfigs
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<BuildConfig>>()?)?;

        unmarshall_body.set_id(params.get_id());
        match build_config::DataStore::update(&self.conn, &unmarshall_body) {
            Ok(Some(build_config)) => Ok(render_json(status::Ok, &build_config)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
    ///PUT: /buildconfigs/:id/status
    ///Input Status  as input
    ///Returns an BuildConfigs
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<StatusUpdate>>()?,
        )?;
        unmarshall_body.set_id(params.get_id());

        match build_config::DataStore::status_update(&self.conn, &unmarshall_body) {
            Ok(Some(build_config)) => Ok(render_json(status::Ok, &build_config)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
}

///The Api wirer for BuildConfigApi
///Add all the api needed to be supported under `/buildconfig`
///To add an api refer, comments in Api trait.
impl Api for BuildConfigApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : build config
        let mut _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let show_by_assembly_factory = move |req: &mut Request| -> AranResult<Response> { _self.show_by_assemblyfactory(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let update_status = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };


        router.post(
            "/buildconfigs",
            XHandler::new(C { inner: create })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.buildconfig.post".to_string(),&*config)),
            "build_config",
        );

        router.get(
            "/buildconfigs",
            XHandler::new(C { inner: list })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.buildconfig.get".to_string(),&*config)),
            "build_config_list",
        );
        router.get(
            "/buildconfigs/:id",
            XHandler::new(C { inner: show })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.buildconfig.get".to_string(),&*config)),
            "build_config_show",
        );
        router.get(
            "/buildconfigs/assemblyfactorys/:id",
            XHandler::new(C { inner: show_by_assembly_factory })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.buildconfig.get".to_string(),&*config)),
            "build_config_list_by_assembly_factorys",
        );

        router.put(
            "/buildconfigs/:id",
            XHandler::new(C { inner: update })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.buildconfig.put".to_string(),&*config)),
            "build_config_update",
        );

        router.put(
            "/buildconfigs/:id/status",
            XHandler::new(C { inner: update_status })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.buildconfig.put".to_string(),&*config)),
            "build_config_status_update",
        );
    }
}

///Convinient helpers to validating an api
impl ApiValidator for BuildConfigApi {}

///Convinient helpers to verify any api
impl ParmsVerifier for BuildConfigApi {}

///Called by implementing ApiValidator when validate() is invoked with the parsed body
///Checks for required parameters in the parsed struct BuildConfig
impl Validator for BuildConfig {
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
