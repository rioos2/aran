// Copyright 2018 The Rio Advancement Inc

//! A collection of deployment [assembly, assembly_factory, for the HTTP server

use std::sync::Arc;
use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;

use api::{Api, ApiValidator, ParmsVerifier, Validator, QueryValidator};

use config::Config;
use error::Error;
use http_gateway::http::controller::*;
use http_gateway::util::errors::{bad_request, internal_error, not_found_error};
use http_gateway::util::errors::{AranResult, AranValidResult};
/// TO_DO: Should be named  (authorize::models::teams, authorize::models::permission)
use authorize::models::policy;
use protocol::api::authorize::PolicyMemberInputs;
use authorize::policies::PolicyFactory;
use protocol::api::schema::{dispatch};
use protocol::api::base::{MetaFields,IdGet};
use protocol::api::authorize::{Policies, PolicyMembersList};
use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::ErrorMessage::MissingParameter;

/// team api: PolicyApi provides ability to declare the teams
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
#[derive(Clone)]
pub struct PolicyApi {
    conn: Box<DataStoreConn>,
}

impl PolicyApi {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        PolicyApi { conn: datastore }
    }

    //POST: /policies/teams/:id
    //The body has the input policy members
    //Returns a mutated policy_members  with
    //- id
    //- ObjectMeta: has updated created_at
    //- created_at
    fn apply(&self, req: &mut Request) -> AranResult<Response> {
        self.verify_id(req)?;

        let unmarshall_body = self.validate::<PolicyMemberInputs>(req.get::<bodyparser::Struct<PolicyMemberInputs>>()?)?;

        debug!("{} ✓",
            format!("======= policy members parsed {:?} ", unmarshall_body.clone()),
        );        

        match PolicyFactory::new(&self.conn).mk_policy_members(&unmarshall_body) {
            Ok(mem) => Ok(render_json(status::Ok, &mem)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }       
    }

    /// PUT: /policies/teams/:id
    /// update policy to particular team
    fn update(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;
        let unmarshall_body = self.validate::<PolicyMembersList>(req.get::<bodyparser::Struct<PolicyMembersList>>()?)?;

        debug!("{} ✓",
            format!("======= policy members update parsed {:?} ", unmarshall_body.clone()),
        );        

        match PolicyFactory::new(&self.conn).update_policy_members(&unmarshall_body) {
            Ok(mem) => Ok(render_json(status::Ok, &mem)),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
        }       
    }


    //GET: /policies?level="user"
    //Input as string input and returns a teams
    fn list_by_level(&self, req: &mut Request) -> AranResult<Response> {
        let query_pairs = self.default_validate(req)?;
        match policy::DataStore::new(&self.conn).list_by_level(&IdGet::with_id(query_pairs.get("level"))) {
            Ok(Some(policies)) => Ok(render_json_list(status::Ok, dispatch(req), &policies)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                query_pairs.get("level")
            ))),
        }
    }

    //GET: /policies/all
    //Returns all the policies(irrespective of namespaces)
    fn list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match policy::DataStore::new(&self.conn).list_blank() {
            Ok(Some(list)) => Ok(render_json_list(status::Ok, dispatch(req), &list)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

}

impl Api for PolicyApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : teams
        let _self = self.clone();
        let list_all = move |req: &mut Request| -> AranResult<Response> { _self.list_blank(req) };

        let _self = self.clone();
        let list_by_level = move |req: &mut Request| -> AranResult<Response> { _self.list_by_level(req) };

        let _self = self.clone();
        let apply = move |req: &mut Request| -> AranResult<Response> { _self.apply(req) };

        let _self = self.clone();
        let update = move |req: &mut Request| -> AranResult<Response> { _self.update(req) };

        router.get(
            "/policies/all",
            XHandler::new(C { inner: list_all }).before(basic.clone()),
            "list_all",
        );
        router.get(
            "/policies",
            XHandler::new(C { inner: list_by_level }).before(basic.clone()),
            "list_by_level",
        );
        router.post(
            "/policies/teams/:id",
            XHandler::new(C { inner: apply }).before(basic.clone()),
            "apply",
        );
        router.put(
            "/policies/teams/:id",
            XHandler::new(C { inner: update }).before(basic.clone()),
            "update",
        );

    }
}
impl ApiValidator for PolicyApi {}

impl QueryValidator for PolicyApi {}

impl ParmsVerifier for PolicyApi {}

impl Validator for Policies {
    //default implementation is to check for `name` and 'level'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.object_meta().name.len() <= 0 {
            s.push("name".to_string());
        }

        let level: String = match self.get_metadata().get("level") {
                        Some(level) => level.to_string(),
                        None => "".to_string()
                    };

        if level.len() <= 0 {
            s.push("level".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for PolicyMemberInputs {
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
        
        if self.get_allowed_policies().is_empty() {
            s.push("allowed_policies".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for PolicyMembersList {
    //default implementation is to check for `policies` vec
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];
        
        if self.get_policies().is_empty() {
            s.push("policies".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
