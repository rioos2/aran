// Copyright 2018 The Rio Advancement Inc

// A module containing the common things needed to build an API
// Every API needs to implement a wire, and use the common typemeta/objectmeta

use iron::prelude::*;
use std::sync::Arc;

use config::Config;
use router::Router;

pub mod deploy;
pub mod security;
use protocol::api::base::{IdGet, StatusUpdate};

use http_gateway::util::errors::{bad_request, malformed_body};
use http_gateway::util::errors::{AranResult, AranValidResult};

use error::ErrorMessage::{MissingBody, MissingParameter, MustBeNumeric};

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

// Implement this trait when the request object (eg: Assembly, AssemblyFactory)
//  need to be validated.
pub trait Validator: Send + Sized + 'static {
    fn results(self) {
        println!("default results is getting called");
    }
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
            Some(id) => match id.parse::<u64>() {
                Ok(_) => Ok(IdGet::with_id(id.to_string())),
                Err(_) => return Err(bad_request(&MustBeNumeric("id".to_string()))),
            },
            None => return Err(bad_request(&MissingParameter("id".to_string()))),
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
            Some(name) => Ok(IdGet::with_id(name.to_string())),
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

    fn verify_name(&self, req: &Request) -> AranResult<IdGet> {
        NameParmsVerifier::verify(req)
    }
}
