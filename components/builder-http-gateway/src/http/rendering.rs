// Copyright (c) 2018 Rio Advancement Inc
//

use hab_net::NetError;
use iron::headers::ContentType;
use iron::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use serde::Serialize;
use serde_json;

use super::net_err_to_http;

pub fn render_json<T>(status: status::Status, response: &T) -> Response
where
    T: Serialize,
{
    let encoded = serde_json::to_string(response).unwrap();
    let headers = Header(ContentType(Mime(
        TopLevel::Application,
        SubLevel::Json,
        vec![(Attr::Charset, Value::Utf8)],
    )));
    Response::with((status, encoded, headers))
}

/// Return an IronResult containing the body of a NetError and the appropriate HTTP response status
/// for the corresponding NetError.
///
/// For example, a NetError::ENTITY_NOT_FOUND will result in an HTTP response containing the body
/// of the NetError with an HTTP status of 404.
///
/// # Panics
///
/// * The given encoded message was not a NetError
/// * The given message could not be decoded
/// * The NetError could not be encoded to JSON
pub fn render_net_error(err: &NetError) -> Response {
    render_json(net_err_to_http(err.code()), err)
}
