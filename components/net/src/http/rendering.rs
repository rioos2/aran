
use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use serde::Serialize;
use serde_json;
use protocol::net::NetError;
use super::net_err_to_http;

/// Return an Modifier<Result> containing the body of a NetError and the appropriate HTTP response status
/// as json
/// This is used by BeforeMiddlerware and others where an error needs to be communicated as json.
pub fn render_json_error<T: Serialize>(err: &NetError, status: status::Status, response: &T) -> IronError {
    let encoded = serde_json::to_string(response).unwrap();
    let headers = Header(ContentType(
        Mime(TopLevel::Application, SubLevel::Json, vec![]),
    ));

    IronError::new(err.clone(), (status, encoded, headers))
}

pub fn render_json<T: Serialize>(status: status::Status, response: &T) -> Response {
    let encoded = serde_json::to_string(response).unwrap();
    let headers = Header(ContentType(
        Mime(TopLevel::Application, SubLevel::Json, vec![]),
    ));

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
    render_json(net_err_to_http(err.get_code()), err)
}

/*
// Status is a return value for calls that don't return other objects.
type Status struct {
	TypeMeta `json:",inline"`
	// Standard list metadata.
	// More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
	// +optional
	ListMeta `json:"metadata,omitempty"`

	// Status of the operation.
	// One of: "Success" or "Failure".
	// More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
	// +optional
	Status string `json:"status,omitempty"`
	// A human-readable description of the status of this operation.
	// +optional
	Message string `json:"message,omitempty"`
	// A machine-readable description of why this operation is in the
	// "Failure" status. If this value is empty there
	// is no information available. A Reason clarifies an HTTP status
	// code but does not override it.
	// +optional
	Reason StatusReason `json:"reason,omitempty"`
	// Extended data associated with the reason.  Each reason may define its
	// own extended details. This field is optional and the data returned
	// is not guaranteed to conform to any schema except that defined by
	// the reason type.
	// +optional
	Details *StatusDetails `json:"details,omitempty"`
	// Suggested HTTP return code for this status, 0 if not set.
	// +optional
	Code int32 `json:"code,omitempty"`
}
*/
