// Copyright 2018 The Rio Advancement Inc

// A module containing the common things needed to build an API
// Every API needs to implement a wire, and use the common typemeta/objectmeta

use std::sync::Arc;

use iron::prelude::*;

use router::Router;
use config::Config;
use std::collections::BTreeMap;

//The macro should be loaded first. As we want to use it `mod audit`.
//Hence  `mod events` must be loaded before `mod audit`
#[macro_use]
pub mod events;

pub mod audit;
pub mod cluster;
pub mod deploy;
pub mod security;
pub mod devtooling;
pub mod authorize;
pub mod objectstorage;

mod helpers;
use protocol::api::base::{IdGet, StatusUpdate, QueryInput};

use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, malformed_body};
use error::ErrorMessage::{MissingParameter, MissingBody, MustBeNumeric, MissingQueryParameter};
use api::helpers::extract_query_value;

// `Api` trait which defines `RESTful` API.
pub trait Api {
    /// Used to extend Api.
    fn wire(&mut self, config: Arc<Config>, router: &mut Router);
}

// `ApiValidator` trait which defines that any `RESTful` API can be validated.
// What needs to be validated is provided by the Validator trait.
pub trait ApiValidator: Send + Sized + 'static {
    fn validate<T>(&self, opt_body: Option<T>) -> AranValidResult<T>
    where
        T: Validator,
    {
        match opt_body {
            Some(body) => body.valid(),
            None => {
                return Err(malformed_body(&MissingBody, &""));
            }
        }
    }
}

//API resources that wish to expand its own using a cache can do so, by implementing
//this trait. The with_cache building the expander with the  behaviour  by defining
//what are the resources the cache needs to manage, and how does it do so.
//Every expandersender shall provide cache_closures of loading a cache to the expander.
//The expander supports multiple cache_closures.
//This is a singular expander meaning, if an id is provided it can provide the cache entry.
trait ExpanderSender: 'static + Send {
    fn with_cache(&mut self);
}

// Implement this trait when the request object (eg: Assembly, AssemblyFactory)
//  need to be validated.
pub trait Validator: Send + Sized + 'static {
    //default implementation is to bypass validation,
    fn valid(self) -> AranValidResult<Self> {
        return Ok(Box::new(self));
    }
}

// A general implementation for StatusUpdate.
// Does a check to see if there are status and conditions
impl Validator for StatusUpdate {
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.status.phase.len() <= 0 {
            s.push("phase".to_string());
        }

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

/// RequestVerifier verifies if the request sent a parameter
/// `example id`
/// If it doesn't then it sends an MissingParameter. ? TO-DO
/// If it exists and the parameter is non numeric then it sends MustBeNumeric
/// You can have as many verifiers as you want.
trait RequestVerifier {
    fn verify(req: &Request) -> AranResult<IdGet>;
}

/// Parameters verifier
/// IdParmsVerifier verifies if the request sent a parameter
/// `id`
/// If it doesn't then it sends an MissingParameter. ? TO-DO
/// If it exists and the parameter is non numeric then it sends MustBeNumeric
struct IdParmsVerifier {}

impl RequestVerifier for IdParmsVerifier {
    fn verify(req: &Request) -> AranResult<IdGet> {
        match req.extensions.get::<Router>().unwrap().find("id") {
            Some(id) => {
                match id.parse::<u64>() {
                    Ok(_name) => Ok(IdGet::with_id(id.to_string())),
                    Err(_) => return Err(bad_request(&MustBeNumeric("id".to_string()))),
                }
            }
            None => return Err(bad_request(&MissingParameter("id".to_string()))),
        }
    }
}

/// AccountParmsVerifier verifies if the request sent a parameter
/// `account`
/// If it doesn't then it sends an MissingParameter.
struct AccountParmsVerifier {}

impl RequestVerifier for AccountParmsVerifier {
    fn verify(req: &Request) -> AranResult<IdGet> {
        match req.extensions.get::<Router>().unwrap().find("account_id") {
            Some(account) => {
                match account.parse::<u64>() {
                    Ok(account) => Ok(IdGet::with_account(account.to_string())),
                    Err(_) => return Err(bad_request(&MustBeNumeric("account".to_string()))),
                }
            }
            None => return Err(bad_request(&MissingParameter("account".to_string()))),
        }
    }
}
//NameParmsVerifier verifies if the request sent a parameter
/// `name`
/// If it doesn't then it sends an MissingParameter

struct NameParmsVerifier {}

impl RequestVerifier for NameParmsVerifier {
    fn verify(req: &Request) -> AranResult<IdGet> {
        match req.extensions.get::<Router>().unwrap().find("name") {
            Some(name) => {
                Ok(IdGet::with_id(name.to_string()))
            }
            None => return Err(bad_request(&MissingParameter("name".to_string()))),
        }
    }
}

/// ParmsVerifier is a convenient method that composes any number of verifiers
/// The current verifiers are for verifying existenace of `id` `account`
///
trait ParmsVerifier {
    fn verify_id(&self, req: &Request) -> AranResult<IdGet> {
        IdParmsVerifier::verify(req)
    }

    fn verify_account(&self, req: &Request) -> AranResult<IdGet> {
        AccountParmsVerifier::verify(req)
    }

    fn verify_name(&self, req: &Request) -> AranResult<IdGet> {
        NameParmsVerifier::verify(req)
    }
}

/// QueryVerifier verifies if the request sent a query parameter
/// `example nodeid="2345678"`
/// QueryVerifier verifies if the query params has the value or not
/// If it doesn't then it sends an MissingParameter. ? TO-DO
/// If it exists then it sends QueryInput
trait QueryVerifier {
    fn validate_query(req: &mut Request) -> AranResult<QueryInput>;
}

/// OptionalQuery verifier
/// QueryVerifier verifies if the query params has the value or not
/// If it doesn't then it send the empty QueryInput
/// If it exists then it sends QueryInput
struct OptionalQuery {}

impl QueryVerifier for OptionalQuery {
    fn validate_query(req: &mut Request) -> AranResult<QueryInput> {
        if req.url.query().is_none() {
            return Ok(QueryInput::with(BTreeMap::new()));
        }
        Ok(QueryInput::with(extract_query_value(req).unwrap()))
    }
}

/// DefaultQuery verifier
/// QueryVerifier verifies if the query params has the value or not
/// If it doesn't then it sends an MissingParameter. ? TO-DO
/// If it exists then it sends QueryInput
struct DefaultQuery {}

impl QueryVerifier for DefaultQuery {
    fn validate_query(req: &mut Request) -> AranResult<QueryInput> {
        match extract_query_value(req) {
            Some(query_pairs) => Ok(QueryInput::with(query_pairs)),
            None => {
                return Err(bad_request(
                    &MissingQueryParameter("No Query Params Found".to_string()),
                ))
            }
        }
    }
}

/// QueryValidator is a convenient method that composes any number of validators
/// The current validators are for validating existenace of "query params"
///
trait QueryValidator {
    fn optional_validate(&self, req: &mut Request) -> AranResult<QueryInput> {
        OptionalQuery::validate_query(req)
    }

    fn default_validate(&self, req: &mut Request) -> AranResult<QueryInput> {
        DefaultQuery::validate_query(req)
    }
}
