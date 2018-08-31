use std::str;

use iron::headers::ContentType;
use iron::mime::{Mime, SubLevel, TopLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;

use protocol::api::schema::ApiSchema;
use util::errors::Bad;

use serde::Serialize;
use serde_json;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ResponseList<T> {
    api_version: String,
    kind: String,
    pub items: T,
}

/// Return an Modifier<Result> containing the body of a NetError and the appropriate HTTP response status
/// as json
/// This is used by BeforeMiddlerware and others where an error needs to be communicated as json.
pub fn render_json_error(err: &Bad, status: status::Status) -> IronError {
    let encoded = serde_json::to_string(err).unwrap();
    let headers = Header(ContentType(Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![],
    )));

    IronError::new(err.clone(), (status, encoded, headers))
}

pub fn render_json<T: Serialize>(status: status::Status, response: &T) -> Response {
    let encoded = serde_json::to_string(response).unwrap();
    let headers = Header(ContentType(Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![],
    )));

    Response::with((status, encoded, headers))
}

pub fn render_json_list<T>(status: status::Status, ident: ApiSchema, response: &T) -> Response
where
    T: Serialize,
{
    let encoded = serde_json::to_string(&ResponseList {
        api_version: ident.version,
        kind: ident.kind,
        items: response,
    }).unwrap();
    let headers = Header(ContentType(Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![],
    )));
    Response::with((status, encoded, headers))
}
