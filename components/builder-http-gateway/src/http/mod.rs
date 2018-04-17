// Copyright (c) 2018 Rio Advancement Inc
//

//! A module containing the HTTP server and handlers for servicing client requests

pub mod controller;
pub mod headers;
pub mod helpers;
pub mod middleware;
pub mod rendering;

use hab_net::ErrCode;
use iron::status::Status;

pub fn net_err_to_http(err: ErrCode) -> Status {
    match err {
        ErrCode::TIMEOUT => Status::GatewayTimeout,
        ErrCode::REMOTE_REJECTED => Status::NotAcceptable,
        ErrCode::ENTITY_NOT_FOUND => Status::NotFound,
        ErrCode::ENTITY_CONFLICT => Status::Conflict,

        ErrCode::ACCESS_DENIED |
        ErrCode::SESSION_EXPIRED => Status::Unauthorized,

        ErrCode::BAD_REMOTE_REPLY |
        ErrCode::SECRET_KEY_FETCH |
        ErrCode::VCS_CLONE => Status::BadGateway,

        ErrCode::NO_SHARD |
        ErrCode::SOCK |
        ErrCode::REMOTE_UNAVAILABLE => Status::ServiceUnavailable,

        ErrCode::BAD_TOKEN => Status::Forbidden,
        ErrCode::GROUP_NOT_COMPLETE => Status::UnprocessableEntity,
        ErrCode::PARTIAL_JOB_GROUP_PROMOTE => Status::PartialContent,

        ErrCode::BUG |
        ErrCode::POST_PROCESSOR |
        ErrCode::BUILD |
        ErrCode::EXPORT |
        ErrCode::SYS |
        ErrCode::DATA_STORE |
        ErrCode::WORKSPACE_SETUP |
        ErrCode::SECRET_KEY_IMPORT |
        ErrCode::INVALID_INTEGRATIONS |
        ErrCode::REG_CONFLICT |
        ErrCode::REG_NOT_FOUND => Status::InternalServerError,
    }
}
