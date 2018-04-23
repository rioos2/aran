// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server
use std::collections::BTreeMap;

use iron::Handler;
use iron::headers;
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::NotFound;
use iron::typemap::Key;
use router::NoRoute;
use persistent;

use unicase::UniCase;
use ansi_term::Colour;
use super::rendering::*;
use super::super::util::errors::*;
use super::header_extracter::HeaderDecider;

use db::data_store::DataStoreConn;
use common::ui;

use auth::rioos::AuthenticateDelegate;
use auth::rbac::authorizer;
use config::base::AuthenticationFlowCfg;

use util::errors::{bad_err, internal_error};

/// Wrapper around the standard `handler functions` to assist in formatting errors or success
// Can't Copy or Debug the fn.
#[allow(missing_debug_implementations, missing_copy_implementations)]
pub struct C<T>
where
    T: Send + Sync + 'static + Fn(&mut Request) -> AranResult<Response>,
{
    pub inner: T,
}

impl<T> C<T>
where
    T: Send + Sync + 'static + Fn(&mut Request) -> AranResult<Response>,
{
    pub fn new(t: T) -> Self {
        C { inner: t }
    }
}

impl<T> Handler for C<T>
where
    T: Send + Sync + 'static + Fn(&mut Request) -> AranResult<Response>,
{
    fn handle(&self, req: &mut Request) -> Result<Response, IronError> {
        match (&self.inner)(req) {
            Ok(resp) => Ok(resp),
            Err(e) => match e.response() {
                Some(response) => {
                    println!("\n----------------\nOutput response: \n{}\n", response);
                    Ok(response)
                }
                None => {
                    let err = internal_error(&format!(
                        "BUG! Report to development http://bit.ly/rioosbug"
                    ));
                    Err(render_json_error(&bad_err(&err), err.http_code()))
                }
            },
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

pub struct DataStoreBroker;

impl Key for DataStoreBroker {
    type Value = DataStoreConn;
}

/// Setup authenticate flows and validate them.
pub trait AuthFlow {
    const AUTH_FLOW_NAME: &'static str;
    //flow: F; type of flow

    //Get the value as set in the Flow.
    fn get(&self) -> Option<String>;

    //Say if the flow is valid or not.
    /// The readiness check will be done for
    /// 1. system auth =  which is the presence of service account key.
    /// We should have a Readier trait that is registered for the Autheticated
    /// Every readier will inform they are ready() or not ()
    fn valid(&self) -> Option<String> {
        //self.get().and_then(self.reason())
        None
    }

    //Tell the reason the auth flow is invalid
    fn reason(&self) -> Option<String> {
        Some("You must have a service account. `systemctl stop rioos-api-server`, `rioos-api-server setup`.".to_string())
    }
}

#[derive(Clone)]
pub struct Authenticated {
    pub config_map: BTreeMap<String, String>,
}

impl Authenticated {
    pub fn new<T: AuthenticationFlowCfg>(config: &T) -> Self {
        Authenticated {
            config_map: config.modes().into_iter().collect::<BTreeMap<_, String>>(),
        }
    }
}

/// When an api request needs to be authenticated we will check for the following
/// email + bearer token (or) email + apikey
/// Returns a status 200 on success. Any non-200 responses.
impl BeforeMiddleware for Authenticated {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        ui::rawdumpln(
            Colour::Yellow,
            '☛',
            format!(
                "======= {}:{}:{}:{}",
                req.version, req.method, req.url, req.headers
            ),
        );

        ProceedAuthenticating::proceed(req, self.config_map.clone())
    }
}

/// ProceedAuthenticating starts by decoding the header.
/// We support the following delegates.
/// 1 user email and bearer Token (Headers needed are ?)
/// 2 user email and jsonwebtoken
/// 3 service account name and jsonwebtoken
///
/// first it collects all header fields and
/// create authenticatable enum and pass to authentication delegate function
#[derive(Clone, Debug)]
pub struct ProceedAuthenticating {}

impl ProceedAuthenticating {
    pub fn proceed(req: &mut Request, config_map: BTreeMap<String, String>) -> IronResult<()> {
        let broker = match req.get::<persistent::Read<DataStoreBroker>>() {
            Ok(broker) => broker,
            Err(err) => {
                let err = internal_error(&format!("{}\n", err));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        };

        let header = HeaderDecider::new(req.headers.clone(), config_map.clone())?;

        let delegate = AuthenticateDelegate::new(broker.clone());

        match delegate.authenticate(&header.decide()?) {
            Ok(_validate) => Ok(()),
            Err(err) => {
                let err = unauthorized_error(&format!("{}\n", err));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        }
    }
}

#[derive(Clone)]
pub struct TrustAccessed {
    pub trusted: String,
}

impl TrustAccessed {
    pub fn new(trusted: String) -> Self {
        TrustAccessed { trusted: trusted }
    }

    fn get(&self) -> String {
        self.trusted.clone()
    }
}

impl BeforeMiddleware for TrustAccessed {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let broker = match req.get::<persistent::Read<DataStoreBroker>>() {
            Ok(broker) => broker,
            Err(err) => {
                let err = internal_error(&format!("{}\n", err));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        };

        let header = HeaderDecider::new(req.headers.clone(), BTreeMap::new())?;

        let roles: authorizer::RoleType = header.decide()?.into();
        // return Ok if the request has no header with email and serviceaccount name
        if roles.name.get_id().is_empty() {
            return Ok(());
        }

        match authorizer::Authorization::new(broker, roles).verify(self.get()) {
            Ok(_validate) => Ok(()),
            Err(err) => {
                let err = unauthorized_error(&format!("{}\n", err));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        }
    }
}

pub struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.is::<NoRoute>() {
            Ok(Response::with((
                NotFound,
                format!("404: {:?}", req.url.path()),
            )))
        } else {
            Err(err)
        }
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
        res.headers.set(headers::AccessControlAllowMethods(vec![
            Method::Put,
            Method::Delete,
        ]));

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

/*pub fn session_create(conn: &DataStoreConn, request: SessionCreate) -> AranResult<Session> {
    //wrong name, use another fascade method session_create
    match sessions::DataStore::find_account(&conn, &request) {
        Ok(session) => return Ok(session),
        Err(e) => Err(not_found_error(&format!(
            "{}: Couldn not create session for the account.",
            e.to_string()
        ))),
    }
}*/
