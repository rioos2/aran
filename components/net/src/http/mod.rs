// Copyright (c) 2017 RioCorp Inc.


pub mod controller;
pub mod headers;
pub mod middleware;
pub mod rendering;
pub mod token_target;

use iron::status::Status;

use protocol::net::ErrCode;

pub fn net_err_to_http(err: ErrCode) -> Status {
    match err {
        ErrCode::BUG => Status::InternalServerError,
        ErrCode::TIMEOUT => Status::GatewayTimeout,
        ErrCode::REMOTE_REJECTED => Status::NotAcceptable,
        ErrCode::BAD_REMOTE_REPLY => Status::BadGateway,
        ErrCode::ENTITY_NOT_FOUND => Status::NotFound,
        ErrCode::NO_SHARD => Status::ServiceUnavailable,
        ErrCode::MALFORMED_DATA => Status::UnprocessableEntity,
        ErrCode::ACCESS_DENIED => Status::Unauthorized,
        ErrCode::SESSION_EXPIRED => Status::Unauthorized,
        ErrCode::ENTITY_CONFLICT => Status::Conflict,
        ErrCode::DATA_STORE => Status::ServiceUnavailable,
        ErrCode::AUTH_SCOPE => Status::Forbidden,
        ErrCode::WORKSPACE_SETUP => Status::InternalServerError,
        ErrCode::SECRET_KEY_FETCH => Status::BadGateway,
        ErrCode::SECRET_KEY_IMPORT => Status::InternalServerError,
        ErrCode::VCS_CLONE => Status::BadGateway,
        ErrCode::BUILD => Status::InternalServerError,
        ErrCode::POST_PROCESSOR => Status::InternalServerError,
    }
}
