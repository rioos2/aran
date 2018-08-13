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
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_MEMBER};
/// TO_DO: Should be named  (authorize::models::teams, authorize::models::permission)
use authorize::models::team;
use authorize::models::invitations;
use authorize::invites::Invites;
use protocol::api::base::MetaFields;
use protocol::api::authorize::Teams;
use protocol::api::invitations::{InvitationInputs, InvitationsList};

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
        match team::DataStore::new(&self.conn).create(&unmarshall_body) {
            Ok(create) => Ok(render_json(status::Ok, &create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    //GET: /teams/:id
    //Input id - u64 as input and returns a teams
    fn show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match team::DataStore::new(&self.conn).show(&params) {
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
        match team::DataStore::new(&self.conn).show_by_name(&params) {
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
        match team::DataStore::new(&self.conn).list() {
            Ok(Some(list)) => Ok(render_json_list(status::Ok, dispatch(req), &list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    fn list_by_origins(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_name(req)?;
        match team::DataStore::new(&self.conn).list_by_origins(&params) {
            Ok(Some(list)) => Ok(render_json_list(status::Ok, dispatch(req), &list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //POST: /teams/invitations
    //The body has the input cluster::invitation_inputs
    //Returns a mutated invitation_inputs  with
    //- id
    //- account_id
    //- origin_id
    //- team_id
    //- users
    //- created_at
    fn invite_users(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate::<InvitationInputs>(req.get::<bodyparser::Struct<InvitationInputs>>()?)?;

        debug!("{} ✓",
            format!("======= parsed {:?} ", unmarshall_body.clone()),
        );

        let converted_body: InvitationsList = unmarshall_body.clone().into();

        debug!("{} ✓",
            format!("======= converted parsed {:?} ", converted_body),
        );

        match Invites::new(&self.conn).mk_invites(&converted_body) {
            Ok(create) => Ok(render_json(status::Ok, &create)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }
}

impl Api for TeamApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : teams
        self.with_cache();
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

        let _self = self.clone();
        let invite_users =
            move |req: &mut Request| -> AranResult<Response> { _self.invite_users(req) };

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
        router.post(
            "/teams/invitations",
            XHandler::new(C { inner: invite_users }).before(basic.clone()),
            "invite_users_to_team",
        );
    }
}

use protocol::api::base::IdGet;
use serde_json;

impl ExpanderSender for TeamApi {
    fn with_cache(&mut self) {   

        let _conn = self.conn.clone();
        let member_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_MEMBER.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                debug!("» Members live load for ≈ {}", id);
                invitations::DataStore::list_by_teams(&_conn, &id)
                    .ok()
                    .and_then(|e| serde_json::to_string(&e).ok())
            }),
        ));
       
        &self.conn.expander.with(member_service);        
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

impl Validator for InvitationInputs {
    //default implementation is to check for `account`, 'team id'  and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_account_id().len() <= 0 {
            s.push("account_id".to_string());
        }

        if self.get_origin_id().len() <= 0 {
            s.push("origin_id".to_string());
        }

        if self.get_team_id().len() <= 0 {
            s.push("team_id".to_string());
        }
        
         if self.get_users().is_empty() {
            s.push("users".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
