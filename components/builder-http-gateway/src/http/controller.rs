// Copyright (c) 2018 Rio Advancement Inc
//

pub use hab_net::{ErrCode, NetError, NetOk, NetResult};
pub use iron::{status, headers};
pub use iron::headers::{ContentType, UserAgent};
pub use iron::prelude::*;
use protobuf;
use protocol::Routable;

pub use super::net_err_to_http;
pub use super::headers::*;
pub use super::middleware::*;
pub use super::rendering::{render_json, render_net_error};
use super::middleware::XRouteClient;

pub fn route_message<M, R>(req: &mut Request, msg: &M) -> NetResult<R>
where
    M: Routable,
    R: protobuf::MessageStatic,
{
    req.extensions
        .get_mut::<XRouteClient>()
        .expect("no XRouteClient extension in request")
        .route::<M, R>(msg)
}
