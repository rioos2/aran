// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the SessionDS.
use chrono::prelude::*;
use error::{Result, Error};
use protocol::{sessionsrv, asmsrv, servicesrv, originsrv};
use postgres;
use privilege;
use db::data_store::DataStoreConn;
use serde_json;
use ldap::{LDAPClient, LDAPUser};
use db;


pub struct SessionDS;

impl SessionDS {
    //For new users to onboard in Rio/OS, which takes the full account creation arguments and returns the Session which has the token.
    //The default role and permission for the user is
    //The default origin is
    pub fn account_create(datastore: &DataStoreConn, session_create: &sessionsrv::SessionCreate) -> Result<sessionsrv::Session> {
        //call and do find_or_create_account_via_session
        SessionDS::find_or_create_account_via_session(datastore, session_create, true, false)
        //do find_or_create_default_role_permission
        //do find_or_create_default_origin
        //return Session
    }

    pub fn find_or_create_account_via_session(datastore: &DataStoreConn, session_create: &sessionsrv::SessionCreate, is_admin: bool, is_service_access: bool) -> Result<sessionsrv::Session> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = conn.query(
            "SELECT * FROM select_or_insert_account_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
            &[
                &session_create.get_name(),
                &session_create.get_email(),
                &session_create.get_first_name(),
                &session_create.get_last_name(),
                &session_create.get_phone(),
                &session_create.get_apikey(),
                &session_create.get_password(),
                &session_create.get_states(),
                &session_create.get_approval(),
                &session_create.get_suspend(),
                &session_create.get_roles(),
                &session_create.get_registration_ip_address(),
            ],
        ).map_err(Error::AccountCreate)?;

        let row = rows.get(0);
        let account = row_to_account(row);
        let id = account.get_id().parse::<i64>().unwrap();


        let provider = match session_create.get_provider() {
            sessionsrv::OAuthProvider::OpenID => "openid",
            _ => "password",
        };

        let rows = conn.query(
            "SELECT * FROM insert_account_session_v1($1, $2, $3, $4, $5)",
            &[
                &id,
                &session_create.get_token(),
                &provider,
                &is_admin,
                &is_service_access,
            ],
        ).map_err(Error::AccountGetById)?;
        let session_row = rows.get(0);
        let mut session: sessionsrv::Session = account.into();
        session.set_token(session_row.get("token"));

        /*
        This will be moved to role/permission
        let mut flags = privilege::FeatureFlags::empty();
        if session_row.get("is_admin") {
            flags.insert(privilege::ADMIN);
        }
        if session_row.get("is_early_access") {
            flags.insert(privilege::EARLY_ACCESS);
        }
        if session_row.get("is_build_worker") {
            flags.insert(privilege::BUILD_WORKER);
        }
        session.set_flags(flags.bits());
       */
        Ok(session)
    }

    pub fn get_account(datastore: &DataStoreConn, account_get: &sessionsrv::AccountGet) -> Result<Option<sessionsrv::Account>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = conn.query(
            "SELECT * FROM get_account_by_email_v1($1)",
            &[&account_get.get_email()],
        ).map_err(Error::AccountGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(row_to_account(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_account_by_id(datastore: &DataStoreConn, account_get_id: &sessionsrv::AccountGetId) -> Result<Option<sessionsrv::Account>> {
        let conn = datastore.pool.get_shard(0)?;
        let id = account_get_id.get_id().parse::<i64>().unwrap();
        let rows = conn.query("SELECT * FROM get_account_by_id_v1($1)", &[&id])
            .map_err(Error::AccountGetById)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(row_to_account(row)))
        } else {
            Ok(None)
        }
    }

    /*pub fn get_session_by_token(datastore: &DataStoreConn, session: &str) -> Result<Option<sessionsrv::Session>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = conn.query(
            "SELECT * FROM get_account_session_by_token_v1($1)",
            &[&session_get.get_token()],
        ).map_err(Error::SessionGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            let mut session = sessionsrv::Session::new();
            let id = row.get("id");
            session.set_id(id);
            let email: String = row.get("email");
            session.set_email(email);
            let name: String = row.get("name");
            session.set_name(name);
            let token: String = row.get("token");
            session.set_token(token);
            let mut flags = privilege::FeatureFlags::empty();
            if row.get("is_admin") {
                flags.insert(privilege::ADMIN);
            }
            if row.get("is_service_access") {
                flags.insert(privilege::SERVICE_ACCESS);
            }
            if row.get("is_default_worker") {
                flags.insert(privilege::DEFAULT_ACCESS);
            }
            session.set_flags(flags.bits());
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }
*/

    pub fn get_session(datastore: &DataStoreConn, session_get: &sessionsrv::SessionGet) -> Result<Option<sessionsrv::Session>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = conn.query(
            "SELECT * FROM get_account_session_v1($1, $2)",
            &[&session_get.get_email(), &session_get.get_token()],
        ).map_err(Error::SessionGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            let mut session = sessionsrv::Session::new();
            let id: i64 = row.get("id");
            session.set_id(id.to_string());
            let email: String = row.get("email");
            session.set_email(email);
            let name: String = row.get("name");
            session.set_name(name);
            let token: String = row.get("token");
            session.set_token(token);
            let api_key: String = row.get("api_key");
            session.set_apikey(api_key);
            let mut flags = privilege::FeatureFlags::empty();
            if row.get("is_admin") {
                flags.insert(privilege::ADMIN);
            }
            if row.get("is_service_access") {
                flags.insert(privilege::SERVICE_ACCESS);
            }
            session.set_flags(flags.bits());
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }
    pub fn origin_create(datastore: &DataStoreConn, org_create: &originsrv::Origin) -> Result<Option<originsrv::Origin>> {
        let conn = datastore.pool.get_shard(0)?;
        let id = org_create
            .get_object_meta()
            .get_uid()
            .parse::<i64>()
            .unwrap();
        let object_meta = serde_json::to_string(org_create.get_object_meta()).unwrap();
        let type_meta = serde_json::to_string(org_create.get_type_meta()).unwrap();
        let rows = &conn.query(
            "SELECT * FROM insert_origin_v1($1,$2,$3,$4,$5)",
            &[
                &(org_create.get_object_meta().get_origin() as String),
                &(id),
                &(org_create.get_object_meta().get_name() as String),
                &(type_meta as String),
                &(object_meta as String),
            ],
        ).map_err(Error::OriginCreate)?;
        let origin = row_to_origin(&rows.get(0))?;
        return Ok(Some(origin.clone()));
    }

    pub fn origin_list(datastore: &DataStoreConn) -> Result<Option<originsrv::OriginGetResponse>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_origins_v1()", &[]).map_err(
            Error::OriginGetResponse,
        )?;

        let mut response = originsrv::OriginGetResponse::new();

        let mut org_collection = Vec::new();
        for row in rows {
            org_collection.push(row_to_origin(&row)?)
        }
        response.set_org_collection(org_collection, "OriginList".to_string(), "v1".to_string());
        Ok(Some(response))
    }

    pub fn origin_show(datastore: &DataStoreConn, get_origin: &asmsrv::IdGet) -> Result<Option<originsrv::Origin>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query("SELECT * FROM get_origin_v1($1)", &[&get_origin.get_id()])
            .map_err(Error::OriginGet)?;
        for row in rows {
            let origin = row_to_origin(&row)?;
            return Ok(Some(origin));
        }
        Ok(None)
    }


    pub fn ldap_config_create(datastore: &DataStoreConn, ldap_config: &sessionsrv::LdapConfig) -> Result<Option<sessionsrv::LdapConfig>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_ldap_config_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)",
            &[
                &(ldap_config.get_host() as String),
                &(ldap_config.get_port() as String),
                &(ldap_config.get_enforce_starttls() as bool),
                &(ldap_config.get_use_ldaps() as bool),
                &(ldap_config.get_lookup_dn() as String),
                &(ldap_config.get_lookup_password() as String),
                &(ldap_config.get_ca_certs() as String),
                &(ldap_config.get_client_cert() as String),
                &(serde_json::to_string(ldap_config.get_user_search()).unwrap()),
                &(serde_json::to_string(ldap_config.get_group_search()).unwrap()),
            ],
        ).map_err(Error::LdapConfigCreate)?;
        let ldap = row_to_ldap_config(&rows.get(0))?;
        return Ok(Some(ldap.clone()));
    }


    pub fn test_ldap_config(datastore: &DataStoreConn, get_id: &asmsrv::IdGet) -> Result<Option<sessionsrv::Success>> {
        match Self::get_ldap_config(datastore, get_id) {
            Ok(Some(ldap_config)) => return test_ldap(ldap_config),
            Err(err) => Err(err),
            _ => {
                return Err(Error::Db(
                    db::error::Error::RecordsNotFound("No Record".to_string()),
                ))
            }
        }
    }

    pub fn get_ldap_config(datastore: &DataStoreConn, get_id: &asmsrv::IdGet) -> Result<Option<sessionsrv::LdapConfig>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_ldap_config_v1($1)",
            &[&(get_id.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::LdapConfigCreate)?;
        if rows.len() != 0 {
            for row in rows {
                let data = row_to_ldap_config(&row)?;
                return Ok(Some(data));
            }
        }
        Ok(None)
    }
    pub fn import_ldap_config(datastore: &DataStoreConn, get_id: &asmsrv::IdGet) -> Result<()> {
        //write a get_ldap_config and use that in both the above and this.
        match Self::get_ldap_config(datastore, get_id) {
            Ok(ldap_config) => {
                let ldusers = ldap_users(ldap_config);
                ldusers.for_each(|l| {
                    //call AccountDS and insert the data.
                    //how do we trap success/failure.
                });
            }
            Err(e) => Err(e),
        }
    }

    pub fn saml_provider_create(datastore: &DataStoreConn, saml_provider: &sessionsrv::SamlProvider) -> Result<Option<sessionsrv::SamlProvider>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_saml_provider_v1($1,$2,$3)",
            &[
                &(saml_provider.get_description() as String),
                &(saml_provider.get_idp_metadata() as String),
                &(saml_provider.get_sp_base_url() as String),
            ],
        ).map_err(Error::SamlProviderCreate)?;
        let saml = row_to_saml_provider(&rows.get(0))?;
        return Ok(Some(saml.clone()));
    }

    pub fn oidc_provider_create(datastore: &DataStoreConn, oidc_provider: &sessionsrv::OidcProvider) -> Result<Option<sessionsrv::OidcProvider>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_oidc_provider_v1($1,$2,$3,$4,$5,$6,$7)",
            &[
                &(oidc_provider.get_description() as String),
                &(oidc_provider.get_issuer() as String),
                &(oidc_provider.get_base_url() as String),
                &(oidc_provider.get_client_secret() as String),
                &(oidc_provider.get_client_id() as String),
                &(oidc_provider.get_verify_server_certificate() as bool),
                &(oidc_provider.get_ca_certs() as String),


            ],
        ).map_err(Error::OidcProviderCreate)?;
        let oidc = row_to_oidc_provider(&rows.get(0))?;
        return Ok(Some(oidc.clone()));
    }

}

fn row_to_account(row: postgres::rows::Row) -> sessionsrv::Account {
    let mut account = sessionsrv::Account::new();
    let id: i64 = row.get("id");
    account.set_id(id.to_string());
    account.set_email(row.get("email"));
    account.set_name(row.get("name"));
    account.set_password(row.get("password"));
    account.set_first_name(row.get("first_name"));
    account.set_last_name(row.get("last_name"));
    account.set_roles(row.get("roles"));
    account.set_apikey(row.get("api_key"));
    account
}


fn row_to_origin(row: &postgres::rows::Row) -> Result<originsrv::Origin> {
    let mut origin_data = originsrv::Origin::new();
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");
    let object_meta: String = row.get("object_meta");
    let type_meta: String = row.get("type_meta");

    origin_data.set_id(id.to_string() as String);
    let object_meta_obj: servicesrv::ObjectMetaData = serde_json::from_str(&object_meta).unwrap();
    origin_data.set_object_meta(object_meta_obj);
    let type_meta_obj: asmsrv::TypeMeta = serde_json::from_str(&type_meta).unwrap();
    origin_data.set_type_meta(type_meta_obj);
    origin_data.set_created_at(created_at.to_rfc3339());
    Ok(origin_data)
}

fn row_to_ldap_config(row: &postgres::rows::Row) -> Result<sessionsrv::LdapConfig> {
    let mut ldap = sessionsrv::LdapConfig::new();
    let id: i64 = row.get("id");
    let user_search: String = row.get("user_search");
    let group_search: String = row.get("group_search");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    ldap.set_id(id.to_string());
    ldap.set_host(row.get("host"));
    ldap.set_port(row.get("port"));
    ldap.set_enforce_starttls(row.get("enforce_starttls"));
    ldap.set_use_ldaps(row.get("use_ldaps"));
    ldap.set_lookup_dn(row.get("lookup_dn"));
    ldap.set_lookup_password(row.get("lookup_password"));
    ldap.set_ca_certs(row.get("ca_certs"));
    ldap.set_client_cert(row.get("client_cert"));
    let user_search: sessionsrv::UserSearch = serde_json::from_str(&user_search).unwrap();
    let group_search: sessionsrv::GroupSearch = serde_json::from_str(&group_search).unwrap();
    ldap.set_user_search(user_search);
    ldap.set_group_search(group_search);
    ldap.set_created_at(created_at.to_rfc3339());

    Ok(ldap)
}

fn test_ldap(ldap_data: sessionsrv::LdapConfig) -> Result<Option<sessionsrv::Success>> {
    let ldap = LDAPClient::new(ldap_data);
    if let Err(err) = ldap.connection() {
        return Err(err);
    }
    let mut success = sessionsrv::Success::new();
    success.set_result("Success".to_string());
    Ok(Some(success))
}

fn ldap_users(ldap_data: sessionsrv::LdapConfig) -> Result<Vec<LDAPUser>> {
    let ldap = LDAPClient::new(ldap_data);
    if let Err(err) = ldap.connection() {
        return Err(err);
    }
    ldap.search()
}


fn row_to_saml_provider(row: &postgres::rows::Row) -> Result<sessionsrv::SamlProvider> {
    let mut saml = sessionsrv::SamlProvider::new();
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    saml.set_id(id.to_string());
    saml.set_description(row.get("description"));
    saml.set_idp_metadata(row.get("idp_metadata"));
    saml.set_sp_base_url(row.get("sp_base_url"));
    saml.set_created_at(created_at.to_rfc3339());

    Ok(saml)

}
fn row_to_oidc_provider(row: &postgres::rows::Row) -> Result<sessionsrv::OidcProvider> {
    let mut oidc = sessionsrv::OidcProvider::new();
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<UTC>>("created_at");

    oidc.set_id(id.to_string());
    oidc.set_description(row.get("description"));
    oidc.set_issuer(row.get("issuer"));
    oidc.set_base_url(row.get("base_url"));
    oidc.set_client_secret(row.get("client_secret"));
    oidc.set_client_id(row.get("client_id"));
    oidc.set_verify_server_certificate(row.get("verify_server_certificate"));
    oidc.set_ca_certs(row.get("ca_certs"));
    oidc.set_created_at(created_at.to_rfc3339());

    Ok(oidc)

}
