// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use iron::Handler;
use iron::headers::{self, Authorization, Bearer};
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::Status;
use iron::typemap::Key;
use unicase::UniCase;
use protocol::sessionsrv::*;
use protocol::net::{self, ErrCode};

// use serde_json;
use super::net_err_to_http;
// use super::super::error::Error;
use super::super::auth::default::PasswordAuthClient;
use super::super::auth::shield::ShieldClient;
use super::super::metrics::prometheus::PrometheusClient;
use config;
use session::privilege::FeatureFlags;
use super::headers::*;
use super::token_target::*;


use db::data_store::{Broker, DataStoreConn};
use session::session_ds::SessionDS;

/// Wrapper around the standard `iron::Chain` to assist in adding middleware on a per-handler basis
pub struct XHandler(Chain);

impl XHandler {
    /// Create a new XHandler
    pub fn new<H: Handler>(handler: H) -> Self {
        XHandler(Chain::new(handler))
    }

    /// Add one or more before-middleware to the handler's chain
    pub fn before<M: BeforeMiddleware>(mut self, middleware: M) -> Self {
        self.0.link_before(middleware);
        self
    }

    /// Add one or more after-middleware to the handler's chain
    pub fn after<M: AfterMiddleware>(mut self, middleware: M) -> Self {
        self.0.link_after(middleware);
        self
    }

    /// Ad one or more around-middleware to the handler's chain
    pub fn around<M: AroundMiddleware>(mut self, middleware: M) -> Self {
        self.0.link_around(middleware);
        self
    }
}

impl Handler for XHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.0.handle(req)
    }
}


pub struct PrometheusCli;

impl Key for PrometheusCli {
    type Value = PrometheusClient;
}


pub struct PasswordAuthCli;

impl Key for PasswordAuthCli {
    type Value = PasswordAuthClient;
}

pub struct DataStoreBroker;

impl Key for DataStoreBroker {
    type Value = DataStoreConn;
}


impl BeforeMiddleware for DataStoreBroker {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let conn = Broker::connect().unwrap();
        req.extensions.insert::<DataStoreBroker>(conn);
        Ok(())
    }
}


#[derive(Clone)]
pub struct Authenticated {
    github: PasswordAuthClient,
    features: FeatureFlags,
}


impl Authenticated {
    pub fn new<T: config::PasswordAuth>(config: &T) -> Self {
        let github = PasswordAuthClient::new(config);
        Authenticated {
            github: github,
            features: FeatureFlags::empty(),
        }
    }

    pub fn require(mut self, flag: FeatureFlags) -> Self {
        self.features.insert(flag);
        self
    }

    fn authenticate(&self, datastore: &DataStoreConn, email: &str, token: &str) -> IronResult<Session> {
        let tk_target = TokenTarget::new(email.to_string(), token.to_string());
        let request: SessionGet = tk_target.into();

        match SessionDS::get_session(datastore, &request) {
            Ok(Some(session)) => Ok(session),
            Ok(None) => {
                let mut session_tk: SessionCreate = SessionCreate::new();
                session_tk.set_email(email.to_string());
                session_tk.set_token(token.to_string());

                let session = try!(session_create(datastore, &session_tk));
                let flags = FeatureFlags::from_bits(session.get_flags()).unwrap();
                if !flags.contains(self.features) {
                    let err = net::err(ErrCode::ACCESS_DENIED, "net:auth:0");
                    return Err(IronError::new(err, Status::Forbidden));
                }

                return Ok(session);
            }
            Err(err) => {
                let nerr = net::err(
                    ErrCode::DATA_STORE,
                    format!("{} {}", "net::todo-change-it-auth-1", err.to_string()),
                );
                return Err(IronError::new(nerr, Status::Unauthorized));
            }

        }
    }
}


impl Key for Shielded {
    type Value = Session;
}

//If the header has custom flags X-AUTH-SHIELDs then we do a check with the shield tokens.
impl BeforeMiddleware for Shielded {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let session = {
            match req.headers.get::<XAuthShield>() {
                Some(shield_token) => try!(self.shield(shield_token)),
                _ => {
                    //log_event!(req, Event::AuthShieldSkipped {});
                    return Ok(());
                }
            }
        };
        req.extensions.insert::<Self>(session);
        Ok(())
    }
}



#[derive(Clone)]
pub struct Shielded {
    shield: ShieldClient,
    features: FeatureFlags,
}

impl Shielded {
    pub fn new<T: config::ShieldAuth>(config: &T) -> Self {
        let shielder = ShieldClient::new(config);
        Shielded {
            shield: shielder,
            features: FeatureFlags::empty(),
        }
    }

    pub fn require(mut self, flag: FeatureFlags) -> Self {
        self.features.insert(flag);
        self
    }

    fn shield(&self, token: &str) -> IronResult<Session> {
        let mut request = SessionGet::new();
        request.set_token(token.to_string());

        /*match SessionDS::get_session(&conn, &request) {
            Ok(session) => Ok(session),
            Err(err) => {
                if err.get_code() == ErrCode::SESSION_EXPIRED {
                    /*    let session = try!(shield_create(&self.github, token));
                    let flags = FeatureFlags::from_bits(session.get_flags()).unwrap();
                    if !flags.contains(self.features) {
                        let err = net::err(ErrCode::ACCESS_DENIED, "net:shield:0");
                        return Err(IronError::new(err, Status::Forbidden));
                    }
                    Ok(session)*/
                    return Err(IronError::new(err, Status::Forbidden));
                } else {
                    let status = net_err_to_http(err.get_code());
                    let body = itry!(serde_json::to_string(&err));
                    Err(IronError::new(err, (body, status)))
                }
            }
        }*/
        let err = net::err(ErrCode::ACCESS_DENIED, "net:shield:0");
        Err(IronError::new(err, Status::Forbidden))
    }
}

impl Key for Authenticated {
    type Value = Session;
}

/// When an api request needs to be authenticateed we will check for the following
/// email + bearer token (or) email + apikey
/// Returns a status 200 on success. Any non-200 responses.
impl BeforeMiddleware for Authenticated {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("--> {}", req.url);

        let session = {
            let email = req.headers.get::<XAuthRioOSEmail>();
            //This is malformed header actually.
            if email.is_none() {
                let err = net::err(ErrCode::ACCESS_DENIED, "net:auth:2");
                return Err(IronError::new(err, Status::Unauthorized));
            }
            match req.headers.get::<Authorization<Bearer>>() {
                Some(&Authorization(Bearer { ref token })) => {
                    match req.extensions.get_mut::<DataStoreBroker>() {
                        Some(broker) => self.authenticate(broker, email.unwrap(), token)?,
                        None => {
                            let err = net::err(ErrCode::ACCESS_DENIED, "net:auth:1");
                            return Err(IronError::new(err, Status::Unauthorized));
                        }
                    }
                }
                _ => {
                    let err = net::err(ErrCode::ACCESS_DENIED, "net:auth:1");
                    return Err(IronError::new(err, Status::Unauthorized));
                }
            }
        };

        req.extensions.insert::<Self>(session);
        Ok(())
    }
}


pub struct Cors;

impl AfterMiddleware for Cors {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(headers::AccessControlAllowOrigin::Any);
        res.headers.set(headers::AccessControlAllowHeaders(vec![
            UniCase("authorization".to_string()),
            UniCase("range".to_string()),
        ]));
        res.headers.set(headers::AccessControlAllowMethods(
            vec![Method::Put, Method::Delete],
        ));
        Ok(res)
    }
}

pub fn session_create(conn: &DataStoreConn, request: &SessionCreate) -> IronResult<Session> {
    //wrong name, use another fascade method session_create
    match SessionDS::account_create(&conn, &request) {
        Ok(session) => return Ok(session),
        Err(e) => {
            error!("Unexpected error, err={:?}", e);
            let err = net::err(ErrCode::BAD_REMOTE_REPLY, "net:session-create:3");
            let status = net_err_to_http(ErrCode::BUG);
            let body = format!("{}\n", err);
            return Err(IronError::new(err, (body, status)));
        }
    }
}
