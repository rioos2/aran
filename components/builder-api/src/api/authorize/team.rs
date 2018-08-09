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
    fn team_create(&self, req: &mut Request) -> AranResult<Response> {
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
        match team::DataStore::teams_create(&self.conn, &unmarshall_body) {
            Ok(teams_create) => Ok(render_json(status::Ok, &teams_create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /teams/:id
    //Input id - u64 as input and returns a teams
    fn team_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match team::DataStore::teams_show(&self.conn, &params) {
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
    fn team_show_by_name(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;
        match team::DataStore::team_show_by_name(&self.conn, &params) {
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
    fn team_list(&self, req: &mut Request) -> AranResult<Response> {
        match team::DataStore::teams_list(&self.conn) {
            Ok(Some(teams_list)) => Ok(render_json_list(status::Ok, dispatch(req), &teams_list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    fn team_list_by_origins(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;
        match team::DataStore::team_list_by_origins(&self.conn, &params) {
            Ok(Some(teams_list)) => Ok(render_json_list(status::Ok, dispatch(req), &teams_list)),
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
        let team_create =
            move |req: &mut Request| -> AranResult<Response> { _self.team_create(req) };

        let _self = self.clone();
        let team_list = move |req: &mut Request| -> AranResult<Response> { _self.team_list(req) };

        let _self = self.clone();
        let team_show = move |req: &mut Request| -> AranResult<Response> { _self.team_show(req) };

        let _self = self.clone();
        let team_list_by_origins = move |req: &mut Request| -> AranResult<Response> { _self.team_list_by_origins(req) };

        let _self = self.clone();
        let team_show_by_name =
            move |req: &mut Request| -> AranResult<Response> { _self.team_show_by_name(req) };

        //Routes:  Authorization : Teams
        router.post(
            "/teams",
            XHandler::new(C { inner: team_create }).before(basic.clone()),
            "teams",
        );
        router.get(
            "/teams",
            XHandler::new(C { inner: team_list }).before(basic.clone()),
            "team_list",
        );
        router.get(
            "/teams/:id",
            XHandler::new(C { inner: team_show }).before(basic.clone()),
            "team_show",
        );
        router.get(
            "/teams/origins/:name",
            XHandler::new(C { inner: team_list_by_origins }).before(basic.clone()),
            "team_list_by_origins",
        );

        router.get(
            "/teams/name/:name",
            XHandler::new(C {
                inner: team_show_by_name,
            }).before(basic.clone()),
            "team_show_by_name",
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
