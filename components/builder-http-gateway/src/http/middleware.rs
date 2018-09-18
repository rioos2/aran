// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server

use super::header_extracter::HeaderDecider;
use super::rendering::*;
use super::super::util::errors::*;
use auth::config::AuthenticationFlowCfg;
use auth::rbac::account::{AccountsFascade, ServiceAccountsFascade};
use auth::rbac::teams::TeamsFascade;
use auth::rbac::policies::PolicyFascade;
use auth::rbac::authorizer;
use auth::rbac::license::LicensesFascade;
use auth::rbac::permissions::Permissions;
use auth::rioos::AuthenticateDelegate;
use db::data_store::DataStoreConn;
use entitlement::config::License;
use iron::Handler;
use iron::headers;
use iron::method::Method;
use iron::middleware::{AfterMiddleware, AroundMiddleware, BeforeMiddleware};
use iron::prelude::*;
use iron::status::NotFound;
use iron::typemap::Key;
use persistent;
use regex::Regex;
use router::NoRoute;
use std::collections::HashMap;
use unicase::UniCase;
use util::errors::{bad_err, forbidden_error, internal_error, entitlement_error};

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
            Err(e) => {
                match e.response() {
                    Some(response) => {
                        debug!("\n----------------\nOutput response: \n{}\n", response);
                        Ok(response)
                    }
                    None => {
                        let err = internal_error(&format!(
                            "BUG! Report to development http://bit.ly/rioosbug"
                        ));
                        Err(render_json_error(&bad_err(&err), err.http_code()))
                    }
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
        debug!(
            "{}",
            format!("→ ------------------------------------------------------------------------------------")
        );
        debug!(
            "{}",
            format!("======= {}:{}:{}", req.version, req.method, req.url)
        );
        debug!("{}", "Headers:");
        debug!("{}", "========");

        for hv in req.headers.iter() {
            debug!("{}", hv);
        }
        debug!("{}", "Body");
        debug!("{}", "========");
        debug!("{}", "»");

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
        Some(
            "You must have a service account. `systemctl stop rioos-api-server`, `rioos-api-server setup`.".to_string(),
        )
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
        debug!("{} ☛",
            format!(
                "======= {}:{}:{}:{}",
                req.version,
                req.method,
                req.url,
                req.headers
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
    pub fn proceed(req: &mut Request, plugins: Vec<String>, conf: HashMap<String, String>) -> IronResult<()> {
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
        "ACCOUNTS.POST",
        "AUTHENTICATE.POST",
        "LOGOUT.POST",
        "ORIGINS.GET",
        "ORIGINS.POST",
        "SECRETS.POST",
        "SECRETS.GET",
        "SERVICEACCOUNTS.POST",
        "SERVICEACCOUNTS.GET",
        "SERVICEACCOUNTS.PUT",
        "PING.GET",
        "HEALTHZ.GET",
        "WIZARDS.GET",
        "AUDITS.POST",
        "AUDITS.GET",
    ];

    fn grab(req: &mut Request) -> Option<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(0|[1-9][0-9]*)$").unwrap();
        }

        let method = &req.method;
        /*let resource = format!(
            "{}.{:?}",
            &req.url
                .path()
                .into_iter()
                .map(|p| if RE.is_match(&p) {
                    "*".to_string()
                } else {
                    format!("{}", &p)
                })
                .collect::<String>(),
            method
        ).to_uppercase();*/

        let resource = format!("{}.{:?}", &req.url.path()[0], method).to_uppercase();

        debug!("{}", format!("↑ Permission {} {}", "→", resource));

        if !URLGrabber::WHITE_LIST.contains(&resource.as_str()) {
            debug!(
                "{}",
                format!("↑ Permission Verify {} {}", "→", resource)
            );
            return Some(resource.clone());
        }

        debug!(
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
    pub fn new<T: AuthenticationFlowCfg>(config: &T, permissions: Permissions, accounts: AccountsFascade, service_accounts: ServiceAccountsFascade, teams: TeamsFascade, policy: PolicyFascade) -> Self {
        let plugins_and_its_configuration_tuple = config.modes();
        RBAC {
            plugins: plugins_and_its_configuration_tuple.0,
            conf: plugins_and_its_configuration_tuple.1,
            authorizer: authorizer::Authorization::new(permissions, accounts, service_accounts, teams, policy),
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

        let header = HeaderDecider::new(req.headers.clone(), self.plugins.clone(), self.conf.clone())?;
        let teams: authorizer::AccountType = header.decide()?.into();

        debug!("↑ RBAC {} {} {:?}", "→", &teams.name, input_trust);

        if teams.name.is_empty() {
            debug!("↑ RBAC SKIP {} {} {:?}", "→", &teams.name, input_trust);
            return Ok(());
        }

        match self.authorizer.clone().verify(
            teams.clone(),
            input_trust.clone().unwrap(),
        ) {
            Ok(_validate) => Ok(()),
            Err(_) => {
                debug!(
                    "↑☒ RBAC ERROR {} {} {:?}",
                    "→",
                    &teams.clone().name,
                    input_trust.clone()
                );

                let err = forbidden_error(&format!(
                    "{}, is denied access. Must have permission for [{}].",
                    &teams.clone().name,
                    input_trust.clone().unwrap_or("".to_string())
                ));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        }
    }
}


pub struct EntitlementAct {
    license: LicensesFascade,
    _backend: String,
}

impl EntitlementAct {
    pub fn new<T: License>(config: &T, fascade: LicensesFascade) -> Self {
        EntitlementAct {
            license: fascade,
            _backend: config.backend().to_string(),
        }
    }
}

impl BeforeMiddleware for EntitlementAct {
    fn before(&self, _req: &mut Request) -> IronResult<()> {
        match self.license.clone().get_by_name("senseis".to_string()) {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = entitlement_error(&format!("{}\n", err));
                return Err(render_json_error(&bad_err(&err), err.http_code()));
            }
        }
    }
}

pub struct Custom404;

impl AfterMiddleware for Custom404 {
    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        if err.error.is::<NoRoute>() {
            Ok(Response::with(
                (NotFound, format!("404: {:?}", req.url.path())),
            ))
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
        res.headers.set(headers::AccessControlAllowMethods(
            vec![Method::Put, Method::Delete],
        ));

        debug!("{}",
            format!("Response {}:{}:{}", _req.version, _req.method, _req.url),
        );
        debug!("{}", "========");
        debug!("{}", res.to_string());
        debug!("✓{}",
            "------------------------------------------------------------------------------------",
        );

        Ok(res)
    }
}
