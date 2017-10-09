// use std::any::Any;
// use std::error::Error;
// use std::fmt;
// use iron::prelude::*;
// use super::super::http::controller::*;
//
// // use conduit::Response;
// // use diesel::result::Error as DieselError;
//
// #[derive(Serialize)]
// struct StringError {
//     detail: String,
// }
// #[derive(Serialize)]
// struct Bad {
//     errors: Vec<StringError>,
// }
//
// // =============================================================================
// // AranError trait
//
// pub trait AranError: Send + fmt::Display + 'static {
//     fn description(&self) -> &str;
//     fn cause(&self) -> Option<&(AranError)> {
//         None
//     }
//
//     fn response(&self) -> Option<Response> {
//         if self.human() {
//             Some(render_json(&Bad {
//                 errors: vec![StringError { detail: self.description().to_string() }],
//             }))
//         } else {
//             self.cause().and_then(|cause| cause.response())
//         }
//     }
//     fn human(&self) -> bool {
//         false
//     }
// }
//
// impl fmt::Debug for Box<AranError> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         fmt::Display::fmt(self, f)
//     }
// }
//
// impl AranError for Box<AranError> {
//     fn description(&self) -> &str {
//         (**self).description()
//     }
//     fn cause(&self) -> Option<&AranError> {
//         (**self).cause()
//     }
//     fn human(&self) -> bool {
//         (**self).human()
//     }
//     fn response(&self) -> Option<Response> {
//         (**self).response()
//     }
// }
// impl<T: AranError> AranError for Box<T> {
//     fn description(&self) -> &str {
//         (**self).description()
//     }
//     fn cause(&self) -> Option<&AranError> {
//         (**self).cause()
//     }
//     fn human(&self) -> bool {
//         (**self).human()
//     }
//     fn response(&self) -> Option<Response> {
//         (**self).response()
//     }
// }
//
// pub type AranResult<T> = Result<T, Box<AranError>>;
//
// // =============================================================================
// // Chaining errors
//
// pub trait ChainError<T> {
//     fn chain_error<E, F>(self, callback: F) -> AranResult<T>
//     where
//         E: AranError,
//         F: FnOnce() -> E;
// }
//
// struct ChainedError<E> {
//     error: E,
//     cause: Box<AranError>,
// }
//
// impl<T, F> ChainError<T> for F
// where
//     F: FnOnce() -> AranResult<T>,
// {
//     fn chain_error<E, C>(self, callback: C) -> AranResult<T>
//     where
//         E: AranError,
//         C: FnOnce() -> E,
//     {
//         self().chain_error(callback)
//     }
// }
//
// impl<T, E: AranError> ChainError<T> for Result<T, E> {
//     fn chain_error<E2, C>(self, callback: C) -> AranResult<T>
//     where
//         E2: AranError,
//         C: FnOnce() -> E2,
//     {
//         self.map_err(move |err| {
//             Box::new(ChainedError {
//                 error: callback(),
//                 cause: Box::new(err),
//             }) as Box<AranError>
//         })
//     }
// }
//
// impl<T> ChainError<T> for Option<T> {
//     fn chain_error<E, C>(self, callback: C) -> AranResult<T>
//     where
//         E: AranError,
//         C: FnOnce() -> E,
//     {
//         match self {
//             Some(t) => Ok(t),
//             None => Err(Box::new(callback())),
//         }
//     }
// }
//
// impl<E: AranError> AranError for ChainedError<E> {
//     fn description(&self) -> &str {
//         self.error.description()
//     }
//     fn cause(&self) -> Option<&AranError> {
//         Some(&*self.cause)
//     }
//     fn response(&self) -> Option<Response> {
//         self.error.response()
//     }
//     fn human(&self) -> bool {
//         self.error.human()
//     }
// }
//
// impl<E: AranError> fmt::Display for ChainedError<E> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} caused by {}", self.error, self.cause)
//     }
// }
//
// // =============================================================================
// // Error impls
//
// impl<E: Any + Error + Send + 'static> From<E> for Box<AranError> {
//     fn from(err: E) -> Box<AranError> {
//         // if let Some(err) = Any::downcast_ref::<DieselError>(&err) {
//         //     if let DieselError::NotFound = *err {
//         //         return Box::new(NotFound);
//         //     }
//         // }
//
//         struct Shim<E>(E);
//         impl<E: Error + Send + 'static> AranError for Shim<E> {
//             fn description(&self) -> &str {
//                 Error::description(&self.0)
//             }
//         }
//         impl<E: fmt::Display> fmt::Display for Shim<E> {
//             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                 self.0.fmt(f)
//             }
//         }
//         Box::new(Shim(err))
//     }
// }
//
// impl AranError for ::curl::Error {
//     fn description(&self) -> &str {
//         Error::description(self)
//     }
// }
//
// impl AranError for ::serde_json::Error {
//     fn description(&self) -> &str {
//         Error::description(self)
//     }
// }
//
// // =============================================================================
// // Concrete errors
//
// struct ConcreteAranError {
//     description: String,
//     detail: Option<String>,
//     cause: Option<Box<AranError>>,
//     human: bool,
// }
//
// impl fmt::Display for ConcreteAranError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.description)?;
//         if let Some(ref s) = self.detail {
//             write!(f, " ({})", s)?;
//         }
//         Ok(())
//     }
// }
//
// impl AranError for ConcreteAranError {
//     fn description(&self) -> &str {
//         &self.description
//     }
//     fn cause(&self) -> Option<&AranError> {
//         self.cause.as_ref().map(|c| &**c)
//     }
//     fn human(&self) -> bool {
//         self.human
//     }
// }
//
// #[derive(Debug, Clone, Copy)]
// pub struct NotFound;
//
// impl AranError for NotFound {
//     fn description(&self) -> &str {
//         "not found"
//     }
//
//     fn response(&self) -> Option<Response> {
//         let mut response = render_json(&Bad {
//             errors: vec![StringError { detail: "Not Found".to_string() }],
//         });
//         response.status = (404, "Not Found");
//         Some(response)
//     }
// }
//
// impl fmt::Display for NotFound {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         "Not Found".fmt(f)
//     }
// }
//
// #[derive(Debug, Clone, Copy)]
// pub struct Unauthorized;
//
// impl AranError for Unauthorized {
//     fn description(&self) -> &str {
//         "unauthorized"
//     }
//
//     fn response(&self) -> Option<Response> {
//         let mut response = render_json(&Bad {
//             errors: vec![
//                 StringError { detail: "must be logged in to perform that action".to_string() },
//             ],
//         });
//         response.status = (403, "Forbidden");
//         Some(response)
//     }
// }
//
// impl fmt::Display for Unauthorized {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         "must be logged in to perform that action".fmt(f)
//     }
// }
//
// struct BadRequest(String);
//
// impl AranError for BadRequest {
//     fn description(&self) -> &str {
//         self.0.as_ref()
//     }
//
//     fn response(&self) -> Option<Response> {
//         let mut response = render_json(&Bad {
//             errors: vec![StringError { detail: self.0.clone() }],
//         });
//         response.status = (400, "Bad Request");
//         Some(response)
//     }
// }
//
// impl fmt::Display for BadRequest {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.0.fmt(f)
//     }
// }
//
// pub fn internal_error(error: &str, detail: &str) -> Box<AranError> {
//     Box::new(ConcreteAranError {
//         description: error.to_string(),
//         detail: Some(detail.to_string()),
//         cause: None,
//         human: false,
//     })
// }
//
// pub fn internal<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
//     Box::new(ConcreteAranError {
//         description: error.to_string(),
//         detail: None,
//         cause: None,
//         human: false,
//     })
// }
//
// pub fn human<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
//     Box::new(ConcreteAranError {
//         description: error.to_string(),
//         detail: None,
//         cause: None,
//         human: true,
//     })
// }
//
// /// This is intended to be used for errors being sent back to the Ember
// /// frontend, not to cargo as cargo does not handle non-200 response codes well
// /// (see https://github.com/rust-lang/cargo/issues/3995), but Ember requires
// /// non-200 response codes for its stores to work properly.
// ///
// /// Since this is going back to the UI these errors are treated the same as
// /// `human` errors, other than the HTTP status code.
// pub fn bad_request<S: ToString + ?Sized>(error: &S) -> Box<AranError> {
//     Box::new(BadRequest(error.to_string()))
// }
//
// pub fn std_error(e: Box<AranError>) -> Box<Error + Send> {
//     #[derive(Debug)]
//     struct E(Box<AranError>);
//     impl Error for E {
//         fn description(&self) -> &str {
//             self.0.description()
//         }
//     }
//     impl fmt::Display for E {
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             write!(f, "{}", self.0)?;
//
//             let mut err = &*self.0;
//             while let Some(cause) = err.cause() {
//                 err = cause;
//                 write!(f, "\nCaused by: {}", err)?;
//             }
//
//             Ok(())
//         }
//     }
//     Box::new(E(e))
// }
