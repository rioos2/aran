#[warn(dead_code)]
use std::any::Any;
use std::error::Error;
use std::fmt;
use std::io::Read;

use iron::prelude::*;
use bodyparser::BodyError;
use bodyparser::BodyErrorCause::JsonError;
use reqwest;
use iron::status::Status;
use http::rendering::render_json;
use error::Error as ReqwestError;

pub const SUCCESS: &'static str = "Success";
pub const FAILURE: &'static str = "Failure";
pub const NOTFOUND: &'static str = "Not Found";
pub const INTERNALERROR: &'static str = "Must have database, blockchain running. Is it started yet ?";
pub const BADREQUEST: &'static str = "Bad Request";
pub const MALFORMED: &'static str = "MalformedBody";

#[derive(Serialize, Debug, Clone)]
pub struct Bad {
    // Status of the operation.
    // One of: "Success" or "Failure".
    status: String,

    // Suggested HTTP return code for this status, 0 if not set.
    // +optional
    code: String,
    // A human-readable description of the status of this operation.
    // +optional
    message: String,
    // A machine-readable description of why this operation is in the
    // "Failure" status. If this value is empty there
    // is no information available. A Reason clarifies an HTTP status
    reason: String,
}

impl fmt::Display for Bad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl Error for Bad {
    fn description(&self) -> &str {
        ""
    }
}

pub fn bad_err(err: &Box<AranError>) -> Bad {
    Bad {
        status: err.status().to_string(),
        code: err.code().to_string(),
        message: err.description().to_string(),
        reason: err.cause().to_string(),
    }
}

// =============================================================================
// AranError trait

pub trait AranError: Send + fmt::Display + 'static {
    fn status(&self) -> &str {
        FAILURE
    }

    fn code(&self) -> &str;

    fn http_code(&self) -> Status {
        Status::Ok
    }

    fn description(&self) -> &str;

    fn cause(&self) -> &str;

    fn response(&self) -> Option<Response> {
        Some(render_json(
            self.http_code(),
            &Bad {
                status: self.status().to_string(),
                code: self.code().to_string(),
                message: self.description().to_string(),
                reason: self.cause().to_string(),
            },
        ))
    }

    fn human(&self) -> bool {
        true
    }
}

impl fmt::Debug for Box<AranError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl AranError for Box<AranError> {
    fn status(&self) -> &str {
        (**self).status()
    }

    fn code(&self) -> &str {
        (**self).code()
    }

    fn http_code(&self) -> Status {
        (**self).http_code()
    }

    fn description(&self) -> &str {
        (**self).description()
    }

    fn cause(&self) -> &str {
        (**self).cause()
    }

    fn human(&self) -> bool {
        (**self).human()
    }
    fn response(&self) -> Option<Response> {
        (**self).response()
    }
}

impl<T: AranError> AranError for Box<T> {
    fn status(&self) -> &str {
        (**self).status()
    }

    fn code(&self) -> &str {
        (**self).code()
    }

    fn http_code(&self) -> Status {
        (**self).http_code()
    }

    fn description(&self) -> &str {
        (**self).description()
    }

    fn cause(&self) -> &str {
        (**self).cause()
    }

    fn human(&self) -> bool {
        (**self).human()
    }
    fn response(&self) -> Option<Response> {
        (**self).response()
    }
}

pub type AranResult<T> = Result<T, Box<AranError>>;

pub type AranValidResult<T> = Result<Box<T>, Box<AranError>>;


// =============================================================================
// Error impls
impl<E: Any + Error + Send + 'static> From<E> for Box<AranError> {
    fn from(err: E) -> Box<AranError> {
        if let Some(err) = Any::downcast_ref::<BodyError>(&err) {
            {
                if let JsonError(ref err1) = err.cause {
                    return Box::new(MalformedBody(
                        format!("{}", err.detail),
                        format!("{}", err1),
                    ));
                }
            }
        }
        struct Shim<E>(E);
        impl<E: Error + Send + 'static> AranError for Shim<E> {
            fn http_code(&self) -> Status {
                Status::InternalServerError
            }

            fn code(&self) -> &str {
                "500"
            }

            fn description(&self) -> &str {
                Error::description(&self.0)
            }

            fn cause(&self) -> &str {
                "unknown"
            }
        }
        impl<E: fmt::Display> fmt::Display for Shim<E> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        Box::new(Shim(err))
    }
}

// =============================================================================
// Concrete errors

struct ConcreteAranError {
    description: String,
    detail: Option<String>,
    cause: String,
    human: bool,
}

impl fmt::Display for ConcreteAranError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)?;
        if let Some(ref s) = self.detail {
            write!(f, " ({})", s)?;
        }
        Ok(())
    }
}

impl AranError for ConcreteAranError {
    fn http_code(&self) -> Status {
        Status::InternalServerError
    }

    fn code(&self) -> &str {
        "500"
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> &str {
        &self.cause
    }
    fn human(&self) -> bool {
        self.human
    }
}

pub struct NotFound(String);

impl AranError for NotFound {
    fn http_code(&self) -> Status {
        Status::NotFound
    }

    fn code(&self) -> &str {
        "404"
    }

    fn description(&self) -> &str {
        self.0.as_ref()
    }

    fn cause(&self) -> &str {
        NOTFOUND
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct Unauthorized(String);

impl AranError for Unauthorized {
    fn http_code(&self) -> Status {
        Status::Unauthorized
    }

    fn code(&self) -> &str {
        "401"
    }

    fn description(&self) -> &str {
        self.0.as_ref()
    }
    fn cause(&self) -> &str {
        "unauthorized"
    }
}

impl fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Refused since authentication is required and has failed or has not yet been provided. Refer  https://bit.ly/rioosauthetication for the supported authentication.".fmt(f)
    }
}

pub struct NotAcceptable(String);

impl AranError for NotAcceptable {
    fn http_code(&self) -> Status {
        Status::NotAcceptable
    }

    fn code(&self) -> &str {
        "406"
    }

    fn description(&self) -> &str {
        self.0.as_ref()
    }
    fn cause(&self) -> &str {
        "NotAcceptable"
    }
}

impl fmt::Display for NotAcceptable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Refused since the requested resource is must have headers for the supported authentication. Refer  https://bit.ly/rioosauthetication for the supported authentication.".fmt(f)
    }
}

struct BadRequest(String);

impl AranError for BadRequest {
    fn http_code(&self) -> Status {
        Status::BadRequest
    }

    fn code(&self) -> &str {
        "400"
    }

    fn description(&self) -> &str {
        self.0.as_ref()
    }

    fn cause(&self) -> &str {
        BADREQUEST
    }
}

impl fmt::Display for BadRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

struct MalformedBody(String, String);

impl AranError for MalformedBody {
    fn http_code(&self) -> Status {
        Status::BadRequest
    }

    fn code(&self) -> &str {
        "400"
    }

    fn description(&self) -> &str {
        self.0.as_ref()
    }

    fn cause(&self) -> &str {
        self.1.as_ref()
    }
}

impl fmt::Display for MalformedBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


pub struct Entitlement(String);

impl AranError for Entitlement {
    fn http_code(&self) -> Status {
        Status::NotFound
    }

    fn code(&self) -> &str {
        "402"
    }

    fn description(&self) -> &str {
        self.0.as_ref()
    }

    fn cause(&self) -> &str {
        NOTFOUND
    }
}

impl fmt::Display for Entitlement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


pub fn bad_request<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
    Box::new(BadRequest(error.to_string()))
}

pub fn malformed_body<S: ToString + ?Sized>(error: &S, cause: &str) -> Box<AranError> {
    Box::new(MalformedBody(error.to_string(), cause.to_string()))
}

pub fn internal_error(error: &str) -> Box<AranError> {
    Box::new(ConcreteAranError {
        description: error.to_string(),
        detail: None,
        cause: INTERNALERROR.to_string(),
        human: false,
    })
}

pub fn not_found_error<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
    Box::new(NotFound(error.to_string()))
}

pub fn unauthorized_error<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
    Box::new(Unauthorized(error.to_string()))
}

pub fn not_acceptable_error<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
    Box::new(NotAcceptable(error.to_string()))
}

pub fn entitlement_error<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
    Box::new(Entitlement(error.to_string()))
}

pub fn err_from_response(mut response: reqwest::Response) -> ReqwestError {
    if response.status() == reqwest::StatusCode::Unauthorized {
        return ReqwestError::APIError(
            response.status(),
            "Your token mismatch and requires permissions.".to_string(),
        );
    }

    let mut buff = String::new();
    match response.read_to_string(&mut buff) {
        Ok(_) => {
            ReqwestError::APIError(response.status(), buff)
        }
        Err(_) => {
            buff.truncate(0);
            ReqwestError::APIError(response.status(), buff)
        }
    }
}
