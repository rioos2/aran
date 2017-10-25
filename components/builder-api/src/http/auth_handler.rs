// Copyright (c) 2017 RioCorp Inc.

//! A collection of auth [accounts, login, roles, permissions,] for the HTTP server

use bodyparser;
use rand;
use rio_core::event::*;
use rio_net::http::controller::*;
use session::session_ds::SessionDS;
use iron::prelude::*;
use iron::status;
use persistent;
use router::Router;
use iron::typemap;
use protocol::net::{self, ErrCode};
use protocol::sessionsrv::*;
use protocol::asmsrv::IdGet;
use db::data_store::Broker;


define_event_log!();

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SessionCreateReq {
    name: String,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
    api_key: String,
    password: String,
    states: String,
    approval: String,
    suspend: String,
    roles: Vec<String>,
    registration_ip_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SessionLoginReq {
    email: String,
    password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AccountGetReq {
    id: String,
    name: String,
    email: String,
    token: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct LdapConfigReq {
    host: String,
    port: String,
    enforce_starttls: bool,
    use_ldaps: bool,
    lookup_dn: String,
    lookup_password: String,
    user_search: UserSearchReq,
    group_search: GroupSearchReq,
    ca_certs: String,
    client_cert: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserSearchReq {
    search_base: String,
    search_filter_template: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GroupSearchReq {
    search_base: String,
    search_filter_template: String,
    member_attributes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SamlProviderReq {
    description: String,
    idp_metadata: String,
    sp_base_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OidcProviderReq {
    description: String,
    issuer: String,
    base_url: String,
    client_secret: String,
    client_id: String,
    verify_server_certificate: bool,
    ca_certs: String,
}


//Default password authentication.
//The body contains email, password, authenticate and if all is well return a token.
pub fn default_authenticate(req: &mut Request) -> IronResult<Response> {
    let mut account_get: AccountGet = AccountGet::new();

    let mut session_data = SessionCreate::new();
    {

        match req.get::<bodyparser::Struct<SessionLoginReq>>() {
            Ok(Some(body)) => {
                if body.email.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `email`",
                    )));
                }

                if body.password.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `password`",
                    )));
                }
                account_get.set_email(body.email);
                account_get.set_password(body.password);

            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let authcli = req.get::<persistent::Read<PasswordAuthCli>>().unwrap();
    let conn = Broker::connect().unwrap();

    //make sure authenticate returns an account.
    match authcli.authenticate(&conn, &account_get) {
        Ok(account) => {
            session_data.set_email(account.get_email());
            session_data.set_password(account.get_password());
            let authcli = req.get::<persistent::Read<PasswordAuthCli>>().unwrap();

            session_data.set_token(authcli.token().unwrap());

            let session = try!(session_create(&conn, &session_data));


            Ok(render_json(status::Ok, &session))
        }
        Err(e) => {
            error!("unhandled password authentication, err={:?}", e);
            let err = net::err(ErrCode::BUG, "rg:auth:0");
            Ok(render_net_error(&err))
        }
    }
}

pub fn account_create(req: &mut Request) -> IronResult<Response> {
    let mut account_create = SessionCreate::new();
    {

        match req.get::<bodyparser::Struct<SessionCreateReq>>() {
            Ok(Some(body)) => {
                if body.email.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `email`",
                    )));
                }

                if body.api_key.len() <= 0 {
                    let api_key = rand::random::<u64>().to_string();
                    account_create.set_apikey(api_key);
                }

                //Don't know if this a good way to do so as why should PasswordAuthCli
                //act as token generator
                let authcli = req.get::<persistent::Read<PasswordAuthCli>>().unwrap();
                let email = body.email.to_string();


                account_create.set_token(authcli.token().unwrap());
                account_create.set_name(body.name);
                account_create.set_email(email.clone());
                account_create.set_first_name(body.first_name);
                account_create.set_last_name(body.last_name);
                account_create.set_phone(body.phone);
                account_create.set_password(authcli.encrypt(body.password.clone()).unwrap());
                account_create.set_states(body.states);
                account_create.set_approval(body.approval);
                account_create.set_suspend(body.suspend);
                account_create.set_roles(body.roles);
                account_create.set_registration_ip_address(body.registration_ip_address);
            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::account_create(&conn, &account_create) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn account_get_by_id(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();
    let mut account_get_by_id = AccountGetId::new();
    account_get_by_id.set_id(id.to_string());
    match SessionDS::get_account_by_id(&conn, &account_get_by_id) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn account_get(req: &mut Request) -> IronResult<Response> {
    let name = {
        let params = req.extensions.get::<Router>().unwrap();
        let name = params.find("name").unwrap().to_owned();
        name
    };
    let mut account_get = AccountGet::new();
    account_get.set_email(name);
    let conn = Broker::connect().unwrap();

    match SessionDS::get_account(&conn, &account_get) {
        Ok(account) => Ok(render_json(status::Ok, &account)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

pub fn session_get(req: &mut Request) -> IronResult<Response> {
    let mut session_get = SessionGet::new();
    {

        match req.get::<bodyparser::Struct<AccountGetReq>>() {
            Ok(Some(body)) => {
                if body.email.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `email`",
                    )));
                }
                if body.token.len() <= 0 {
                    return Ok(Response::with((
                        status::UnprocessableEntity,
                        "Missing value for field: `token`",
                    )));
                }
                session_get.set_email(body.email.to_string());
                session_get.set_token(body.token.to_string());
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }

    let conn = Broker::connect().unwrap();

    match SessionDS::get_session(&conn, &session_get) {
        Ok(session) => Ok(render_json(status::Ok, &session)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn set_ldap_config(req: &mut Request) -> IronResult<Response> {


    let mut ldap_config = LdapConfig::new();
    {
        match req.get::<bodyparser::Struct<LdapConfigReq>>() {
            Ok(Some(body)) => {

                ldap_config.set_host(body.host);
                ldap_config.set_port(body.port);
                ldap_config.set_enforce_starttls(body.enforce_starttls);
                ldap_config.set_use_ldaps(body.use_ldaps);
                ldap_config.set_lookup_dn(body.lookup_dn);
                ldap_config.set_lookup_password(body.lookup_password);
                ldap_config.set_ca_certs(body.ca_certs);
                ldap_config.set_client_cert(body.client_cert);

                let mut user_search = UserSearch::new();
                user_search.set_search_base(body.user_search.search_base);
                user_search.set_search_filter_template(body.user_search.search_filter_template);
                ldap_config.set_user_search(user_search);


                let mut group_search = GroupSearch::new();
                group_search.set_search_base(body.group_search.search_base);
                group_search.set_search_filter_template(body.group_search.search_filter_template);
                group_search.set_member_attributes(body.group_search.member_attributes);
                ldap_config.set_group_search(group_search);

            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = Broker::connect().unwrap();
    match SessionDS::ldap_config_create(&conn, &ldap_config) {
        Ok(ldap) => Ok(render_json(status::Ok, &ldap)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


pub fn test_ldap_config(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();
    let mut serach_id = IdGet::new();
    serach_id.set_id(id.to_string());

    match SessionDS::test_ldap_config(&conn, &serach_id) {
        Ok(result) => Ok(render_json(status::Ok, &result)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn import_ldap(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("id").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();
    let mut serach_id = IdGet::new();
    serach_id.set_id(id.to_string());

    match SessionDS::import_ldap_config(&conn, &serach_id) {
        Ok(result) => Ok(render_json(status::Ok, &result)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn config_saml_provider(req: &mut Request) -> IronResult<Response> {
    let mut saml_provider = SamlProvider::new();
    {
        match req.get::<bodyparser::Struct<SamlProviderReq>>() {
            Ok(Some(body)) => {
                saml_provider.set_description(body.description);
                saml_provider.set_idp_metadata(body.idp_metadata);
                saml_provider.set_sp_base_url(body.sp_base_url);

            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = Broker::connect().unwrap();
    match SessionDS::saml_provider_create(&conn, &saml_provider) {
        Ok(saml) => Ok(render_json(status::Ok, &saml)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}

#[allow(unused_variables)]
pub fn saml_provider_list(req: &mut Request) -> IronResult<Response> {

    let conn = Broker::connect().unwrap();
    match SessionDS::saml_provider_listall(&conn) {
        Ok(saml_list) => Ok(render_json(status::Ok, &saml_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn saml_provider_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("providerid").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut saml_provider_get = IdGet::new();
    saml_provider_get.set_id(id.to_string());

    match SessionDS::saml_show(&conn, &saml_provider_get) {
        Ok(saml) => Ok(render_json(status::Ok, &saml)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn config_oidc_provider(req: &mut Request) -> IronResult<Response> {
    let mut oidc_provider = OidcProvider::new();
    {
        match req.get::<bodyparser::Struct<OidcProviderReq>>() {
            Ok(Some(body)) => {
                oidc_provider.set_description(body.description);
                oidc_provider.set_issuer(body.issuer);
                oidc_provider.set_base_url(body.base_url);
                oidc_provider.set_client_secret(body.client_secret);
                oidc_provider.set_client_id(body.client_id);
                oidc_provider.set_verify_server_certificate(body.verify_server_certificate);
                oidc_provider.set_ca_certs(body.ca_certs);

            }
            Err(err) => {
                return Ok(render_net_error(&net::err(
                    ErrCode::MALFORMED_DATA,
                    format!("{}, {:?}\n", err.detail, err.cause),
                )));
            }
            _ => return Ok(Response::with(status::UnprocessableEntity)),
        }
    }
    let conn = Broker::connect().unwrap();
    match SessionDS::oidc_provider_create(&conn, &oidc_provider) {
        Ok(oidc) => Ok(render_json(status::Ok, &oidc)),

        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),

    }
}


#[allow(unused_variables)]
pub fn openid_listall(req: &mut Request) -> IronResult<Response> {

    let conn = Broker::connect().unwrap();
    match SessionDS::openid_provider_listall(&conn) {
        Ok(oidc_list) => Ok(render_json(status::Ok, &oidc_list)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}

pub fn openid_provider_show(req: &mut Request) -> IronResult<Response> {
    let id = {
        let params = req.extensions.get::<Router>().unwrap();
        match params.find("providerid").unwrap().parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Ok(Response::with(status::BadRequest)),
        }
    };

    let conn = Broker::connect().unwrap();

    let mut oidc_provider_get = IdGet::new();
    oidc_provider_get.set_id(id.to_string());

    match SessionDS::oidc_show(&conn, &oidc_provider_get) {
        Ok(saml) => Ok(render_json(status::Ok, &saml)),
        Err(err) => Ok(render_net_error(
            &net::err(ErrCode::DATA_STORE, format!("{}\n", err)),
        )),
    }
}
