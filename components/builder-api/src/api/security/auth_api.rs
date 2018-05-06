// Copyright 2018 The Rio Advancement Inc

//! A collection of auth [accounts, login, roles, permissions,] for the HTTP server

use std::sync::Arc;

use bodyparser;
use iron::prelude::*;
use iron::status;
use router::Router;
use api::{Api, ApiValidator, Validator, ParmsVerifier};
use protocol::api::schema::{dispatch, type_meta};

use config::Config;
use error::Error;
use error::ErrorMessage::MissingParameter;

use http_gateway::http::controller::*;
use http_gateway::util::errors::{AranResult, AranValidResult};
use http_gateway::util::errors::{bad_request, internal_error, not_found_error, unauthorized_error, conflict_error};

use rand;
use session::models::session as sessions;
use protocol::api::session::*;

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use protocol::api::base::MetaFields;
use auth::rioos::AuthenticateDelegate;
use auth::util::authenticatable::Authenticatable;
use auth::rioos::user_account::UserAccountAuthenticate;
const DEFAULTROLE: &'static str = "rioos:loneranger";

#[derive(Clone)]
pub struct AuthenticateApi {
    conn: Arc<DataStoreConn>,
}

/// Authenticate api: AuthenticateyApi provides ability to authenticate the user.
/// Needs a DataStore mapper, hence a DataStoreConn needs to be sent in.
//
/// Authentication: URLs supported are.
/// POST: /authenticate,
/// GET: /account/id
/// Record the other apis here.

impl AuthenticateApi {
    pub fn new(datastore: Arc<DataStoreConn>) -> Self {
        AuthenticateApi { conn: datastore }
    }

    //POST: /authenticate
    //Input the user credentials - (default password authentication)
    //The body contains email, password, authenticate and if all is well return a token.
    fn default_authenticate(&self, req: &mut Request) -> AranResult<Response> {
        let account = self.validate(req.get::<bodyparser::Struct<AccountGet>>()?)?;
        let delegate = AuthenticateDelegate::new(self.conn.clone());
        let auth = Authenticatable::UserAndPass {
            username: account.get_email(),
            password: account.get_password(),
        };

        match delegate.authenticate(&auth) {
            Ok(_validate) => {
                let mut session_data = SessionCreate::new();
                session_data.set_email(account.get_email());
                session_data.set_password(account.get_password());

                session_data.set_token(UserAccountAuthenticate::token().unwrap());

                let mut device: Device = user_agent(req).into();
                device.set_ip(format!("{}", req.remote_addr.ip()));

                match sessions::DataStore::find_account(&self.conn, &session_data, &device) {
                    Ok(session) => Ok(render_json(status::Ok, &session)),
                    Err(err) => Err(internal_error(&format!("{}", err))),
                }
            }
            Err(e) => Err(unauthorized_error(&format!("{}", e))),
        }
    }

    //POST: accounts",
    //Input account and creates an user, by returning the Account information of an user
    fn account_create(&self, req: &mut Request) -> AranResult<Response> {
        let mut unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<SessionCreate>>()?,
        )?;

        if unmarshall_body.get_apikey().len() <= 0 {
            unmarshall_body.set_apikey(rand::random::<u64>().to_string());
        }

        let m = unmarshall_body.mut_meta(
            unmarshall_body.object_meta(),
            unmarshall_body.get_email(),
            unmarshall_body.get_account(),
        );

        unmarshall_body.set_meta(type_meta(req), m);
        if unmarshall_body.get_roles().is_empty() {
            unmarshall_body.set_roles(vec![DEFAULTROLE.to_string()]);
        }

        unmarshall_body.set_token(UserAccountAuthenticate::token().unwrap());

        let en = unmarshall_body.get_password();
        unmarshall_body.set_password(UserAccountAuthenticate::encrypt(en).unwrap());

        let mut account_get = AccountGet::new();
        account_get.set_email(unmarshall_body.get_email());

        let mut device: Device = user_agent(req).into();
        device.set_ip(format!("{}", req.remote_addr.ip()));

        match sessions::DataStore::get_account(&self.conn, &account_get) {
            Ok(Some(_account)) => Err(conflict_error(
                &format!("alreay exists {}", account_get.get_email()),
            )),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => {
                match sessions::DataStore::account_create(&self.conn, &unmarshall_body, &device) {
                    Ok(account) => Ok(render_json(status::Ok, &account)),
                    Err(err) => Err(internal_error(&format!("{}", err))),
                }
            }
        }
    }

    //GET: accounts/:id",
    //Input id, and returns the Account information of an user
    fn account_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        let mut account_get_by_id = AccountGetId::new();
        account_get_by_id.set_id(params.get_id());

        match sessions::DataStore::get_account_by_id(&self.conn, &account_get_by_id) {
            Ok(Some(account)) => Ok(render_json(status::Ok, &account)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &account_get_by_id.get_id()
            ))),
        }
    }

    //GET: accounts/:name",
    //Input name, and returns the Account information of an user
    //Change it by building a NameParmsVerifier
    fn account_show_by_name(&self, req: &mut Request) -> AranResult<Response> {
        let name = {
            let params = req.extensions.get::<Router>().unwrap();
            let name = params.find("name").unwrap().to_owned();
            name
        };
        let mut account_get = AccountGet::new();
        account_get.set_email(name);

        match sessions::DataStore::get_account(&self.conn, &account_get) {
            Ok(Some(account)) => Ok(render_json(status::Ok, &account)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &account_get.get_email()
            ))),
        }
    }

    //POST: /logout
    //Global: Logout the current user
    fn account_logout(&self, req: &mut Request) -> AranResult<Response> {
        let account = self.validate(
            req.get::<bodyparser::Struct<AccountTokenGet>>()?,
        )?;

        let mut device: Device = user_agent(req).into();
        device.set_ip(format!("{}", req.remote_addr.ip()));

        match sessions::DataStore::account_logout(&self.conn, &account, &device) {
            Ok(Some(account)) => Ok(render_json(status::Ok, &account)),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    //POST: ldap/configd",
    //Input LdapConfig as body json, and returns LDAPConfgit as the response
    fn config_ldap(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate(req.get::<bodyparser::Struct<LdapConfig>>()?)?;

        match sessions::DataStore::ldap_config_create(&self.conn, &unmarshall_body) {
            Ok(Some(ldap)) => Ok(render_json(status::Ok, &ldap)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //POST: ldap/test/:id",
    //Input id - u64 as input and returns the response after testing ldapconfig
    fn test_ldap(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match sessions::DataStore::test_ldap_config(&self.conn, &params) {
            Ok(Some(result)) => Ok(render_json(status::Ok, &result)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //POST: ldap/import/:id",
    //Input id - u64 as input and returns the response after importing ldapconfig into
    //Rioos
    fn import_ldap(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match sessions::DataStore::import_ldap_config(&self.conn, &params) {
            Ok(result) => Ok(render_json(status::Ok, &result)),
            Err(err) => Err(internal_error(&format!("{}", err))),
        }
    }

    //POST: Create a new saml provider
    ///auth/saml/providers/:providerid
    fn config_saml(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<SamlProvider>>()?,
        )?;

        match sessions::DataStore::saml_provider_create(&self.conn, &unmarshall_body) {
            Ok(Some(saml)) => Ok(render_json(status::Ok, &saml)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: auth/saml/providers/:providerid",
    //Input id - u64 as input and returns a SamlProvider
    fn saml_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match sessions::DataStore::saml_show(&self.conn, &params) {
            Ok(Some(saml)) => Ok(render_json(status::Ok, &saml)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: auth/saml/providers",
    //Returns all saml (no origion)
    // Move to a separatedatastore samls
    fn saml_list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match sessions::DataStore::saml_provider_list_blank(&self.conn) {
            Ok(Some(samls)) => Ok(render_json_list(status::Ok, dispatch(req), &samls)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //POST: Create a new openid
    //  /auth/oidc/providers/:providerid
    fn config_openid(&self, req: &mut Request) -> AranResult<Response> {
        let unmarshall_body = self.validate(
            req.get::<bodyparser::Struct<OidcProvider>>()?,
        )?;

        //do you have to set the provider id in unmarshall_body here ?

        match sessions::DataStore::oidc_provider_create(&self.conn, &unmarshall_body) {
            Ok(Some(oidc)) => Ok(render_json(status::Ok, &oidc)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }

    //GET: auth/oidc/providers/:providerid",
    //Input id - u64 as input and returns a OpenIdProvider
    fn openid_show(&self, req: &mut Request) -> AranResult<Response> {
        let params = self.verify_id(req)?;

        match sessions::DataStore::oidc_show(&self.conn, &params) {
            Ok(Some(openid)) => Ok(render_json(status::Ok, &openid)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!(
                "{} for {}",
                Error::Db(RecordsNotFound),
                &params.get_id()
            ))),
        }
    }

    //GET: auth/oidc/providers",
    //Returns all openid (no origion)
    // Move to a separatedatastore openidds
    fn openid_list_blank(&self, req: &mut Request) -> AranResult<Response> {
        match sessions::DataStore::openid_provider_list_blank(&self.conn) {
            Ok(Some(openids)) => Ok(render_json_list(status::Ok, dispatch(req), &openids)),
            Err(err) => Err(internal_error(&format!("{}", err))),
            Ok(None) => Err(not_found_error(&format!("{}", Error::Db(RecordsNotFound)))),
        }
    }
}

impl Api for AuthenticateApi {
    fn wire(&mut self, config: Arc<Config>, router: &mut Router) {
        let basic = Authenticated::new(&*config);

        //closures : scaling
        let _self = self.clone();
        let account_create = move |req: &mut Request| -> AranResult<Response> { _self.account_create(req) };

        let _self = self.clone();
        let account_show = move |req: &mut Request| -> AranResult<Response> { _self.account_show(req) };

        let _self = self.clone();
        let account_show_by_name = move |req: &mut Request| -> AranResult<Response> { _self.account_show_by_name(req) };

        let _self = self.clone();
        let authenticate = move |req: &mut Request| -> AranResult<Response> { _self.default_authenticate(req) };

        let _self = self.clone();
        let account_logout = move |req: &mut Request| -> AranResult<Response> { _self.account_logout(req) };

        let _self = self.clone();
        let authenticate_ldap = move |req: &mut Request| -> AranResult<Response> { _self.default_authenticate(req) };

        //closures: ldap
        let _self = self.clone();
        let config_ldap = move |req: &mut Request| -> AranResult<Response> { _self.config_ldap(req) };

        let _self = self.clone();
        let import_ldap = move |req: &mut Request| -> AranResult<Response> { _self.import_ldap(req) };

        let _self = self.clone();
        let test_ldap = move |req: &mut Request| -> AranResult<Response> { _self.test_ldap(req) };

        //closures: saml
        let _self = self.clone();
        let config_saml = move |req: &mut Request| -> AranResult<Response> { _self.config_saml(req) };

        let _self = self.clone();
        let saml_show = move |req: &mut Request| -> AranResult<Response> { _self.saml_show(req) };

        let _self = self.clone();
        let saml_list_blank = move |req: &mut Request| -> AranResult<Response> { _self.saml_list_blank(req) };

        //closures: openid
        let _self = self.clone();
        let config_openid = move |req: &mut Request| -> AranResult<Response> { _self.config_openid(req) };

        let _self = self.clone();
        let openid_show = move |req: &mut Request| -> AranResult<Response> { _self.openid_show(req) };

        let _self = self.clone();
        let openid_list_blank = move |req: &mut Request| -> AranResult<Response> { _self.openid_list_blank(req) };

        router.post(
            "/accounts",
            XHandler::new(C { inner: account_create }),
            "account_create:signup",
        );
        router.get(
            "/accounts/:id",
            XHandler::new(C { inner: account_show })
                .before(basic.clone())
                .before(TrustAccessed::new(
                    "rioos.account.get".to_string(),
                    &*config,
                )),
            "account_show",
        );

        router.get(
            "/accounts/name/:name",
            XHandler::new(C { inner: account_show_by_name })
                .before(basic.clone())
                .before(TrustAccessed::new(
                    "rioos.account.get".to_string(),
                    &*config,
                )),
            "account_show_by_name",
        );

        router.post(
            "/authenticate",
            XHandler::new(C { inner: authenticate }),
            "authenticate",
        );
        router.post(
            "/logout",
            XHandler::new(C { inner: account_logout }).before(basic.clone()),
            "account_logout",
        );

        router.post(
            "/authenticate/ldap/:code",
            XHandler::new(C { inner: authenticate_ldap }),
            "authenticate_ldap",
        );

        router.post("/ldap/config", C { inner: config_ldap }, "config_ldap");
        router.post("/ldap/config/:id/test", C { inner: test_ldap }, "test_ldap");
        router.post("/ldap/import/:id", C { inner: import_ldap }, "import_ldap");

        router.post(
            "/auth/saml/providers",
            C { inner: config_saml },
            "config_saml",
        );

        router.get(
            "/auth/saml/providers",
            C { inner: saml_list_blank },
            "saml_list",
        );

        router.get(
            "/auth/saml/providers/:id",
            C { inner: saml_show },
            "saml_show",
        );

        router.post(
            "/auth/oidc/providers/:providerid",
            C { inner: config_openid },
            "config_openid",
        );
        router.get(
            "/auth/oidc/providers",
            C { inner: openid_list_blank },
            "openid_list_blank",
        );
        router.get(
            "auth/oidc/providers/:id",
            C { inner: openid_show },
            "openid_show",
        );
    }
}

impl ApiValidator for AuthenticateApi {}

impl ParmsVerifier for AuthenticateApi {}

impl Validator for AccountGet {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
impl Validator for AccountTokenGet {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_email().len() <= 0 {
            s.push("email".to_string());
        }
        if self.get_token().len() <= 0 {
            s.push("token".to_string());
        }
        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for SessionCreate {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let mut s: Vec<String> = vec![];

        if self.get_email().len() <= 0 {
            s.push("email".to_string());
        }
        if self.get_password().len() <= 0 {
            s.push("password".to_string());
        }
        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for SessionGet {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for LdapConfig {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for SamlProvider {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}

impl Validator for OidcProvider {
    //default implementation is to check for `name` and 'origin'
    fn valid(self) -> AranValidResult<Self> {
        let s: Vec<String> = vec![];

        if s.is_empty() {
            return Ok(Box::new(self));
        }

        Err(bad_request(&MissingParameter(format!("{:?}", s))))
    }
}
