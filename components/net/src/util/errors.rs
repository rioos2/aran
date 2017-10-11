use std::any::Any;
use std::error::Error;
use std::fmt;
use iron::prelude::*;
use iron::status::Status;
use super::super::http::controller::*;

// use diesel::result::Error as DieselError;

pub const SUCCESS: &'static str = "Success";
pub const FAILURE: &'static str = "Failure";

#[derive(Serialize)]
struct Bad {
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

    fn cause(&self) -> Option<&(AranError)> {
        None
    }

    fn response(&self) -> Option<Response> {
        Some(render_json(
            self.http_code(),
            &Bad {
                status: self.status().to_string(),
                code: self.code().to_string(),
                message: self.description().to_string(),
                reason: self.cause().unwrap_or(&internal_error("An unknown error", "not known")).to_string(),
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
    fn cause(&self) -> Option<&AranError> {
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
    fn cause(&self) -> Option<&AranError> {
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

// =============================================================================
// Error impls

impl<E: Any + Error + Send + 'static> From<E> for Box<AranError> {
    fn from(err: E) -> Box<AranError> {
        // if let Some(err) = Any::downcast_ref::<DieselError>(&err) {
        //     if let DieselError::NotFound = *err {
        //         return Box::new(NotFound);
        //     }
        // }

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
        }
        impl<E: fmt::Display> fmt::Display for Shim<E> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.fmt(f)
            }
        }
        Box::new(Shim(err))
    }
}

impl AranError for ::curl::Error {
    fn code(&self) -> &str {
        "500"
    }

    fn description(&self) -> &str {
        Error::description(self)
    }
}

impl AranError for ::serde_json::Error {
    fn code(&self) -> &str {
        "500"
    }

    fn description(&self) -> &str {
        Error::description(self)
    }
}

// =============================================================================
// Concrete errors

struct ConcreteAranError {
    description: String,
    detail: Option<String>,
    cause: Option<Box<AranError>>,
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
    fn cause(&self) -> Option<&AranError> {
        self.cause.as_ref().map(|c| &**c)
    }
    fn human(&self) -> bool {
        self.human
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NotFound;

impl AranError for NotFound {
    fn http_code(&self) -> Status {
        Status::NotFound
    }

    fn code(&self) -> &str {
        "404"
    }

    fn description(&self) -> &str {
        "not found"
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "Not Found".fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Unauthorized;

impl AranError for Unauthorized {
    fn http_code(&self) -> Status {
        Status::Unauthorized
    }

    fn code(&self) -> &str {
        "401"
    }

    fn description(&self) -> &str {
        "unauthorized"
    }
}

impl fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "must be logged in to perform that action".fmt(f)
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
}

impl fmt::Display for BadRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

struct MalformedBody(String);

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

}

impl fmt::Display for MalformedBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}


pub fn bad_request<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
    Box::new(BadRequest(error.to_string()))
}

pub fn malformed_body<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
    Box::new(MalformedBody(error.to_string()))
}

pub fn internal_error(error: &str, detail: &str) -> Box<AranError> {
    Box::new(ConcreteAranError {
        description: error.to_string(),
        detail: Some(detail.to_string()),
        cause: None,
        human: false,
    })
}


pub fn std_error(e: Box<AranError>) -> Box<Error + Send> {
    #[derive(Debug)]
    struct E(Box<AranError>);
    impl Error for E {
        fn description(&self) -> &str {
            self.0.description()
        }
    }
    impl fmt::Display for E {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)?;

            let mut err = &*self.0;
            while let Some(cause) = err.cause() {
                err = cause;
                write!(f, "\nCaused by: {}", err)?;
            }

            Ok(())
        }
    }
    Box::new(E(e))
}
