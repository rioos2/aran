// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use iron::Handler;
use iron::headers::{self, Authorization, Bearer};
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::Status;
use iron::typemap::Key;
use router::Router;


use unicase::UniCase;
use protocol::sessionsrv::*;
use protocol::originsrv::*;

use protocol::asmsrv::IdGet;
use protocol::net::{self, ErrCode};
use ansi_term::Colour;

use super::rendering::*;
use super::super::auth::default::PasswordAuthClient;
use super::super::auth::shield::ShieldClient;
use super::super::metrics::prometheus::PrometheusClient;
use super::super::util::errors::*;
use config;
use session::privilege::FeatureFlags;
use super::headers::*;
use super::token_target::*;
use db::data_store::{Broker, DataStoreConn};
use session::session_ds::SessionDS;
use common::ui;


/// Wrapper around the standard `handler functions` to assist in formatting errors or success
// Can't Copy or Debug the fn.
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct C(pub fn(&mut Request) -> AranResult<Response>);

impl Handler for C {
    fn handle(&self, req: &mut Request) -> Result<Response, IronError> {
        let C(f) = *self;
        match f(req) {
            Ok(resp) => Ok(resp),
            Err(e) => {
                match e.response() {
                    Some(response) => Ok(response),
                    None => Err(render_json_error(
                        &net::err(ErrCode::BUG, "bug. report to development."),
                        Status::InternalServerError,
                        &"",
                    )),
                }
            }
        }
    }
}

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
        ///// Maybe move this Request to a seperate method.
        ui::rawdumpln(
            Colour::Green,
            '→',
            "------------------------------------------------------------------------------------",
        );
        ui::rawdumpln(
            Colour::Cyan,
            ' ',
            format!("======= {}:{}:{}", req.version, req.method, req.url),
        );
        ui::rawdumpln(Colour::Blue, ' ', "Headers:");
        ui::rawdumpln(Colour::White, ' ', "========");

        for hv in req.headers.iter() {
            ui::rawdump(Colour::Purple, ' ', hv);
        }
        ui::rawdumpln(Colour::Blue, ' ', "Body");
        ui::rawdumpln(Colour::White, ' ', "========");
        ui::rawdumpln(Colour::Purple, ' ', "»");

        //// dump ends.

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

    fn authenticate(&self, datastore: &DataStoreConn, email: &str, token: &str) -> AranResult<Session> {
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
                    return Err(unauthorized_error(
                        &format!("{}", "Feature flags in session are not active"),
                    ));
                }

                return Ok(session);
            }
            Err(err) => Err(not_found_error(&format!(
                "{}: Couldn't find {} {} in session.",
                email,
                token,
                err.to_string()
            ))),
        }
    }

    fn check_origin(&self, datastore: &DataStoreConn, org_name: String) -> AranResult<Origin> {
        let mut org_get = IdGet::new();
        org_get.set_id(org_name);
        match SessionDS::origin_show(datastore, &org_get) {
            Ok(Some(origin)) => Ok(origin),
            Err(err) => Err(internal_error(&format!("{}\n", err))),
            Ok(None) => {
                Err(not_found_error(&format!(
                    "Couldn't find in session.",
                )))
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
        ui::rawdumpln(
            Colour::Yellow,
            '☛',
            format!("======= {}:{}:{}", req.version, req.method, req.url),
        );

        let session = {
            let email = req.headers.get::<XAuthRioOSEmail>();

            if email.is_none() {
                let err = net::err(
                    ErrCode::ACCESS_DENIED,
                    format!("Email not found. Missing header X-AUTH-RIOOS-EMAIL."),
                );
                return Err(render_json_error(&err, Status::Unauthorized, &err));
            }

            match req.headers.get::<Authorization<Bearer>>() {
                Some(&Authorization(Bearer { ref token })) => {
                    match req.extensions.get::<DataStoreBroker>() {
                        Some(broker) => {
                            match self.authenticate(broker, email.unwrap(), token) {
                                Ok(data) => {
                                    if format!("{}", req.url).contains("origins") {
                                        let org_name = {
                                            let params = req.extensions.get::<Router>().unwrap();
                                            let org_name = params.find("origin").unwrap_or("").to_owned();
                                            org_name
                                        };
                                        if org_name.len() > 0 {
                                            match self.check_origin(&broker, org_name.to_owned()) {
                                                Ok(_) => data.to_owned(),
                                                Err(_) => {
                                                    let err = net::err(
                                                        ErrCode::ACCESS_DENIED,
                                                        format!("No origin for {}", org_name.to_owned()),
                                                    );
                                                    return Err(render_json_error(&err, Status::Unauthorized, &err));
                                                }
                                            };
                                        }
                                    }
                                    data.to_owned()
                                }
                                Err(err) => {
                                    let err1 = net::err(ErrCode::ACCESS_DENIED, err.to_string());
                                    return Err(render_json_error(&err1, Status::Unauthorized, &err1));
                                }
                            }
                        }
                        None => {
                            let err = net::err(
                                ErrCode::ACCESS_DENIED,
                                format!(
                                    "Unavailable datastore. Unable to authentication for {} and {}.",
                                    email.unwrap(),
                                    token
                                ),
                            );
                            return Err(render_json_error(&err, Status::Unauthorized, &err));
                        }
                    }
                }
                _ => {
                    let err = net::err(
                        ErrCode::ACCESS_DENIED,
                        format!(
                            "Malformed header, Authorization bearer token not found for  {}.",
                            email.unwrap()
                        ),
                    );
                    return Err(render_json_error(&err, Status::Unauthorized, &err));
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

        ui::rawdumpln(
            Colour::Green,
            ' ',
            format!("Response {}:{}:{}", _req.version, _req.method, _req.url),
        );
        ui::rawdumpln(Colour::White, ' ', "========");
        ui::rawdumpln(Colour::Purple, ' ', res.to_string());
        ui::rawdumpln(
            Colour::Cyan,
            '✓',
            "------------------------------------------------------------------------------------",
        );

        Ok(res)
    }
}

pub fn session_create(conn: &DataStoreConn, request: &SessionCreate) -> AranResult<Session> {
    //wrong name, use another fascade method session_create
    match SessionDS::find_account(&conn, &request) {
        Ok(session) => return Ok(session),
        Err(e) => Err(not_found_error(&format!(
            "{}: Couldn not create session for the account.",
            e.to_string()
        ))),

    }
}
