// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [origin] for the HTTP server
use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, Validator, ParmsVerifier};
use rio_net::http::schema::type_meta;

use config::Config;
use error::Error;
use error::ErrorMessage::MissingParameter;

use rio_net::http::controller::*;
use rio_net::util::errors::{AranResult, AranValidResult};
use rio_net::util::errors::{bad_request, internal_error, not_found_error};

use session::team_ds::TeamDS;
use protocol::api::team::Team;
use protocol::api::base::MetaFields;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;

#[derive(Clone)]
pub struct TeamApi {
    conn: Box<DataStoreConn>,
}

/// Origin api: OriginApi provides ability to create sandboxed or spaces for
/// users.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
/// Orgins: URLs supported are.
/// POST: /origin,
/// GET: /origin/:id
impl TeamApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        TeamApi { conn: datastore }
    }

    //POST: /origins
    //The body has the input origins
    //Returns a mutated Origins with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    pub fn create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(req.get::<bodyparser::Struct<Team>>()?)?;

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_name(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        match TeamDS::create(&self.conn, &unmarshall_body) {
            Ok(Some(team)) => Ok(render_json(status::Ok, &team)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for TeamApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        let _self = self.clone();
        let create = move |req: &mut Request| -> AranResult<Response> { _self.create(req) };

        //Team API
        router.post(
            "/teams",
            XHandler::new(C { inner: create }).before(basic.clone()),
            "teams",
        );
    }
}

impl ApiValidator for TeamApi {}

impl ParmsVerifier for TeamApi {}

//Validates parsed Origin from the body of the request.
//Checks for `....` in .....
//This is a NoOp for now.
impl Validator for Team {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_name().len() <= 0 {
            s.push("name".to_string());
        }

        if self.get_metadata()
            .get("origin")
            .unwrap_or(&"".to_string())
            .len() <= 0
        {
            s.push("origin".to_string());
        }

        if self.get_account().len() <= 0 {
            s.push("account".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
