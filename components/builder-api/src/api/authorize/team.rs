// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, ParmsVerifier, Validator};
use config::Config;
use error::Error;
use protocol::api::schema::dispatch;
use protocol::api::schema::type_meta;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};

/// TO_DO: Should be named  (authorize::models::teams, authorize::models::permission)
use authorize::models::team;
use protocol::api::base::MetaFields;
use protocol::api::authorize::Teams;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;

/// team api: TeamApi provides ability to declare the teams
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Secret: URLs supported are.
/// POST: /teams,,
/// GET: /teams,
/// GET: /teams/:id,
//GET: /teams/:name
#[derive(Clone)]
pub struct TeamApi {
    conn: Box<DataStoreConn>,
}

impl TeamApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        TeamApi { conn: datastore }
    }
    //POST: /teams
    //The body has the input cluster::teams
    //Returns a mutated Teams  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate::<Teams>(req.get::<bodyparser::Struct<Teams>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);

        debug!("{} ✓",
            format!("======= parsed {:?} ", unmarshall_body),
        );
        match team::DataStore::create(&self.conn, &unmarshall_body) {
            Ok(create) => Ok(render_json(status::Ok, &create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /teams/:id
    //Input id - u64 as input and returns a teams
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match team::DataStore::show(&self.conn, &params) {
            Ok(Some(teams)) => Ok(render_json(status::Ok, &teams)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /teams/:name
    //Input as string input and returns a teams
    fn show_by_name(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;
        match team::DataStore::show_by_name(&self.conn, &params) {
            Ok(Some(teams)) => Ok(render_json(status::Ok, &teams)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }
    }

    //GET: /teams
    //Returns all the teams(irrespective of namespaces)
    fn list(&self, req: &mut Request) -> AranResult<Response> {
        match team::DataStore::list(&self.conn) {
            Ok(Some(list)) => Ok(render_json_list(status::Ok, dispatch(req), &list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    fn list_by_origins(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;
        match team::DataStore::list_by_origins(&self.conn, &params) {
            Ok(Some(list)) => Ok(render_json_list(status::Ok, dispatch(req), &list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for TeamApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : teams
        let _self = self.clone();
        let create =
            move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        let _self = self.clone();
        let list = move |req: &mut Request| -> AranResult<Response> { _self.list(req) };

        let _self = self.clone();
        let show = move |req: &mut Request| -> AranResult<Response> { _self.show(req) };

        let _self = self.clone();
        let list_by_origins = move |req: &mut Request| -> AranResult<Response> { _self.list_by_origins(req) };

        let _self = self.clone();
        let show_by_name =
            move |req: &mut Request| -> AranResult<Response> { _self.show_by_name(req) };

        //Routes:  Authorization : Teams
        router.post(
            "/teams",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "teams",
        );
        router.get(
            "/teams",
            XHandler::new(C { inner: list }).before(basic.clone()),
            "teams_list",
        );
        router.get(
            "/teams/:id",
            XHandler::new(C { inner: show }).before(basic.clone()),
            "team_show",
        );
        router.get(
            "/teams/origins/:name",
            XHandler::new(C { inner: list_by_origins }).before(basic.clone()),
            "list_by_origins",
        );

        router.get(
            "/teams/name/:name",
            XHandler::new(C {
                inner: show_by_name,
            }).before(basic.clone()),
            "show_by_name",
        );
    }
}

impl ApiValidator for TeamApi {}

impl ParmsVerifier for TeamApi {}

impl Validator for Teams {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_name().len() <= 0 {
            s.push("name".to_string());
        }

        if self.get_description().len() <= 0 {
            s.push("description".to_string());
        }

        if self.get_account().len() <= 0 {
            s.push("account".to_string());
        }

        let origin: String = match self.get_metadata().get("origin") {
                        Some(org) => org.to_string(),
                        None => "".to_string()
                    };

        if origin.len() <= 0 {
            s.push("origin".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
