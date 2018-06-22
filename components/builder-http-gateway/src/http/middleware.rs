// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server
use super::super::util::errors::*;
use super::header_extracter::HeaderDecider;
use super::rendering::*;
use ansi_term::Colour;
use auth::config::AuthenticationFlowCfg;
use entitlement::config::License;
use auth::rbac::{authorizer, permissions::Permissions};
use auth::rioos::AuthenticateDelegate;
use common::ui;
use db::data_store::DataStoreConn;
use iron::headers;
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::NotFound;
use iron::typemap::Key;
use iron::Handler;
use persistent;
use regex::Regex;
use router::NoRoute;
use std::collections::HashMap;
use unicase::UniCase;
use util::errors::{bad_err, forbidden_error, internal_error};
use auth::rbac::license::LicensesFascade;

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
    pub plugins: Vec<String>,
    pub conf: HashMap<String, String>,
}

impl Authenticated {
    pub fn new<T: AuthenticationFlowCfg>(config: &T) -> Self {
        let plugins_and_its_configuration_tuple = config.modes();

        Authenticated {
            plugins: plugins_and_its_configuration_tuple.0,
            conf: plugins_and_its_configuration_tuple.1,
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

        ProceedAuthenticating::proceed(req, self.plugins.clone(), self.conf.clone())
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
    pub fn proceed(
        req: &mut Request,
        plugins: Vec<String>,
        conf: HashMap<String, String>,
    ) -> IronResult<()> {
        let broker = match req.get::<persistent::Read<DataStoreBroker>>() {
            Ok(broker) => broker,
            Err(err) => {
                let err = internal_error(&format!("{}\n", err));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        };

        let header = HeaderDecider::new(req.headers.clone(), plugins, conf)?;

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

struct URLGrabber {}

impl URLGrabber {
    const WHITE_LIST: &'static [&'static str] = &[
        "RIOOS.ACCOUNTS.POST",
        "RIOOS.ACCOUNTS.*.AUDITS.POST",
        "RIOOS.ACCOUNTS.*.AUDITS.GET",
        "RIOOS.ACCOUNTS.*.ASSEMBLYS*EXEC",
        "RIOOS.AUTHENTICATE.POST",
        "RIOOS.LOGOUT.POST",
        "RIOOS.LOGS.GET",
        "RIOOS.IMAGES.*.VULNERABILITY.GET",
        "RIOOS.ROLES.GET",
        "RIOOS.ROLES.POST",
        "RIOOS.PERMISSIONS.GET",
        "RIOOS.PERMISSIONS.POST",
        "RIOOS.TEAMS.POST",
        "RIOOS.ORIGINS.GET",
        "RIOOS.ORIGINS.POST",
        "RIOOS.ORIGINS.*.SECRETS.POST",
        "RIOOS.ORIGINS.*.SECRETS.GET",
        "RIOOS.ORIGINS.*.SECRETS*.POST",
        "RIOOS.ORIGINS.*.SERVICEACCOUNTS.POST",
        "RIOOS.ORIGINS.*.SERVICEACCOUNTS*.GET",
        "RIOOS.ORIGINS.*.SERVICEACCOUNTS*.PUT",
        "RIOOS.ORIGINS.*.SETTINGSMAP*.POST",
        "RIOOS.SERVICEACCOUNTS.GET",
        "RIOOS.SETTINGSMAP.GET",
        "RIOOS.PING.GET",
    ];

    fn grab(req: &mut Request) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(0|[1-9][0-9]*)$").unwrap();
        }

        let system = "rioos";
        let method = &req.method;
        let resource = format!(
            "{}{}.{:?}",
            system,
            &req.url
                .path()
                .into_iter()
                .map(|p| {
                    if RE.is_match(&p) {
                        ".*".to_string()
                    } else {
                        format!(".{}", &p)
                    }
                })
                .collect::<String>(),
            method
        ).to_uppercase();

        info!("{}", format!("↑ Permission {} {}", "→", resource));

        if !URLGrabber::WHITE_LIST.contains(&resource.as_str()) {
            info!(
                "{}",
                format!("↑ Permission Verify {} {}", "→", resource)
            );
            return Some(resource.clone());
        }

        info!(
            "{}",
            format!("↑ Permission WHITE_LIST {} {}", "→", resource)
        );

        None
    }
}

#[derive(Clone)]
pub struct RBAC {
    pub plugins: Vec<String>,
    pub conf: HashMap<String, String>,
    authorizer: authorizer::Authorization,
}

impl RBAC {
    pub fn new<T: AuthenticationFlowCfg>(config: &T, permissions: Permissions) -> Self {
        let plugins_and_its_configuration_tuple = config.modes();
        RBAC {
            plugins: plugins_and_its_configuration_tuple.0,
            conf: plugins_and_its_configuration_tuple.1,
            authorizer: authorizer::Authorization::new(permissions),
        }
    }

    fn input_trust(&self, req: &mut Request) -> Option<String> {
        URLGrabber::grab(req)
    }
}

impl BeforeMiddleware for RBAC {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let input_trust = self.input_trust(req);

        if input_trust.is_none() {
            return Ok(());
        }

        let header =
            HeaderDecider::new(req.headers.clone(), self.plugins.clone(), self.conf.clone())?;
        let roles: authorizer::RoleType = header.decide()?.into();

        info!(
            "↑ RBAC {} {} {:?}",
            "→",
            &roles.name.get_id(),
            input_trust
        );

        if roles.name.get_id().pop().is_none() {
            info!("↑ RBAC SKIP {} {} {:?}", "→", &roles.name, input_trust);
            return Ok(());
        }

        match self.authorizer
            .clone()
            .verify(roles.clone(), input_trust.clone().unwrap())
        {
            Ok(_validate) => Ok(()),
            Err(_) => {
                info!(
                    "↑☒ RBAC ERROR {} {} {:?}",
                    "→",
                    &roles.clone().name,
                    input_trust.clone()
                );

                let err = forbidden_error(&format!(
                    "{}, is denied access. Must have permission for [{}].",
                    &roles.clone().name.get_id(),
                    input_trust.clone().unwrap_or("".to_string())
                ));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        }
    }
}

pub struct EntitlementAct {
   license: LicensesFascade,
   backend: String,
}

impl EntitlementAct {
    pub fn new<T: License>(config: &T, fascade: LicensesFascade) -> Self {
        EntitlementAct {
            license: fascade, 
            backend: config.backend().to_string(),           
        }
    }
}

impl BeforeMiddleware for EntitlementAct {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        Ok(())
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
