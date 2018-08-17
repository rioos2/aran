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
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_MEMBER, CACHE_PREFIX_TEAM};
/// TO_DO: Should be named  (authorize::models::teams, authorize::models::permission)
use authorize::models::{team, team_members};
use authorize::models::invitations;
use authorize::invites::Invites;
use protocol::api::base::MetaFields;
use protocol::api::authorize::Teams;
use protocol::api::invitations::{InvitationInputs, InvitationsList};
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;
use typemap;
use api::audit::blockchain_api::EventLog;
use protocol::api::session;
use session::models::{session as sessions};

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
        let account_id = self.verify_account(req)?;

        match team_members::DataStore::new(&self.conn).list_by_origins(&params, &account_id) {
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
            format!("======= converted body parsed {:?} ", converted_body),
        );

        let params = IdGet::with_id(unmarshall_body.get_team_id());
        let account_id = unmarshall_body.get_account_id();

        match team::DataStore::new(&self.conn).show(&params) {
            Ok(Some(teams)) => self.push_invite_notifications(req, teams, converted_body, account_id),
            Err(err) => Err(internal_error(&format!("Requested Team not found: {}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
        }        
    }

    /// this is private helper function
    /// first build audit data from team and invitation datas
    /// then push event using generated audit data
    /// the audit data has message and sender email details
    fn push_invite_notifications(&self, req: &mut Request, team: Teams, invite_list: InvitationsList, account_id: String) -> AranResult<Response> {
       
        debug!("{} ✓",
            format!("======= Invited team {:?} ", team.clone()),
        );        

        let mut account_get = session::AccountGetId::new();
        account_get.set_id(account_id.clone());
        let account = match sessions::DataStore::get_account_by_id(&self.conn, &account_get) {
            Ok(Some(opt_account)) => opt_account,
            Err(err) => return Err(internal_error(&format!("Requester account not found: {}", err))),
            Ok(None) => return Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                account_id.clone()
            ))),
        };

        let originated_url = format!("https://{}:{}", req.url.host().to_owned(), req.url.port().to_string());

        let invites = Invites::new(&self.conn);

        match invites.mk_invites(&invite_list) {
            Ok(converted) => {
                match converted {
                    Some(ini) => {
                        for x in ini.iter() {
                            let eve = invites.build_event(originated_url.clone(), &x, &team.clone(), account.get_first_name());
                            debug!("{} ✓",
                                        format!("===== parsed audit {:?} ", eve.clone()),
                            );     
                            push_notification!(req, eve.clone());
                        }
                        Ok(render_json(status::Ok, &ini))
                    },
                    None => Err(internal_error(&format!("{}\n", "Invitations build error."))),
                }
            },
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }
    }

    fn accept_invite(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;        
        let invites = Invites::new(&self.conn);

        match invitations::DataStore::show(&self.conn, &params) {
            Ok(Some(invite)) => Ok(render_json(status::Ok, &invites.mk_member(&invite, &params)?)),
            Err(err) => Err(internal_error(&format!("Accepted Team not found: {}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                params.get_id()
            ))),
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

        let _self = self.clone();
        let accept_invite =
            move |req: &mut Request| -> AranResult<Response> { _self.accept_invite(req) };

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
        router.put(
            "/invitations/:id/accept",
            XHandler::new(C { inner: accept_invite }).before(basic.clone()),
            "accept_invite_user_to_team",
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

        let _conn = self.conn.clone();
        let team_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_TEAM.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                debug!("» Team live load for ≈ {}", id);
                team::DataStore::new(&_conn).show(&id)
                    .ok()
                    .and_then(|e| serde_json::to_string(&e).ok())
            }),
        ));
       
        &self.conn.expander.with(member_service);  
        &self.conn.expander.with(team_service);       
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
