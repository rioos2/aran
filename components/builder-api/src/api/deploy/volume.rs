// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [origin] for the HTTP server
use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::{dispatch, type_meta};

use config::Config;
use error::Error;
use error::ErrorMessage::MissingParameter;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

use deploy::models::volume;
use protocol::api::volume::Volumes;
use protocol::api::base::{MetaFields, StatusUpdate};

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;

#[derive(Clone)]
pub struct VolumeApi {
    conn: Box<DataStoreConn>,
}

/// volume api: VolumeApi provides ability to create sandboxed or spaces for
/// users.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Volumes: URLs supported are.
/// POST: /volumes,
/// GET: /assembly/:id/volumes
/// GET: /volumes/:id
/// PUT: /volumes/:id
/// PUT: /volumes/:id/status
impl VolumeApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        VolumeApi { conn: datastore }
    }

    //POST: /volumes
    //The body has the input volumes
    //Returns a mutated volumes with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    pub fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Volumes>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        match volume::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(Some(volume)) => Ok(render_json(status::Ok, &volume)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /assembly/:id/volumes
    //Returns all the volumes for a particular assembly
    fn show_by_assembly(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match volume::DataStore::show_by_assembly(&self.conn, &params) {
            Ok(Some(volume)) => Ok(render_json_list(status::Ok, dispatch(req), &volume)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: /volumes/:id
    pub fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match volume::DataStore::show(&self.conn, &params) {
            Ok(Some(volumes)) => Ok(render_json(status::Ok, &volumes)),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    //PUT: /volumes/:id/status
    //Input status  as input and returns an updated volumes
    fn status_update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<StatusUpdate>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match volume::DataStore::status_update(&self.conn, &unmarshall_body) {
            Ok(Some(volumes)) => Ok(render_json(status::Ok, &volumes)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    ///PUT: /volumes/:id
    ///Input volumes id
    ///Returns updated volumes
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Volumes>>()?)?;
        unmarshall_body.set_id(params.get_id());

        match volume::DataStore::update(&self.conn, &unmarshall_body) {
            Ok(Some(volumes)) => Ok(render_json(status::Ok, &volumes)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }
}

impl Api for VolumeApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let status_update = move |req: &mut Request| -> AranResult<Response> { _self.status_update(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let show_by_assembly = move |req: &mut Request| -> AranResult<Response> { _self.show_by_assembly(req) };

        //volumes
        router.post(
            "/volumes",
            XHandler::new(C { inner: create })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.volume.post".to_string())),
            "volumes",
        );
        router.get(
            "/volumes/:id",
            XHandler::new(C { inner: show })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.volume.get".to_string())),
            "volumes_show",
        );
        router.put(
            "/volumes/:id",
            XHandler::new(C { inner: update })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.volume.put".to_string())),
            "volumes_update",
        );
        router.put(
            "/volumes/:id/status",
            XHandler::new(C {
                inner: status_update,
            })
            .before(basic.clone())
            .before(TrustAccessed::new("rioos.volume.put".to_string())),
            "volumes_status_update",
        );
        router.get(
            "/assemblys/:id/volumes",
            XHandler::new(C {
                inner: show_by_assembly,
            }).before(basic.clone())
            .before(TrustAccessed::new("rioos.volume.get".to_string())),
            "volumes_show_by_assembly",
        );
    }
}

impl ApiValidator for VolumeApi {}

impl ParmsVerifier for VolumeApi {}

//Validates parsed Origin from the body of the request.
//Checks for `....` in .....
//This is a NoOp for now.
impl Validator for Volumes {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];
        if self.object_meta().account.len() <= 0 {
            s.push("account".to_string());
        }
        if self.get_mount_path().len() <= 0 {
            s.push("mount_path".to_string());
        }
        if self.get_allocated().len() <= 0 {
            s.push("allocated".to_string());
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
