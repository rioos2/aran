// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the DataStore.
use chrono::prelude::*;
use error::{Error, Result};

use protocol::api::base::{IdGet, MetaFields,WhoAmITypeMeta};
use protocol::cache::PULL_DIRECTLY;
use protocol::api::session;
use protocol::cache::InMemoryExpander;

use db;
use db::data_store::DataStoreConn;
use postgres;
use serde_json;
use protocol::api::schema::type_meta_url;
use rand;

use super::super::{OpenIdOutputList, SamlOutputList};
use ldap::{LDAPClient, LDAPUser};

pub struct DataStore<'a> {
    db: &'a DataStoreConn,
    expander: &'a InMemoryExpander,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore {
            db: db,
            expander: &db.expander,
        }
    }
    pub fn find_account(
        datastore: &DataStoreConn,
        session_create: &session::SessionCreate,
        device: &session::Device,
    ) -> Result<session::Session> {
        let conn = datastore.pool.get_shard(0)?;
        let query = "SELECT * FROM get_account_by_email_v1($1)";
        let rows = conn.query(&query, &[&session_create.get_email()])
            .map_err(Error::AccountGetById)?;

        if rows.len() > 0 {
            let account_row = rows.get(0);
            let account = row_to_account(account_row);
            let id = account.get_id().parse::<i64>().unwrap();
            let provider = match session_create.get_provider() {
                session::OAuthProvider::OpenID => "openid",
                _ => "password",
            };

            let session_rows = conn.query(
                "SELECT * FROM get_session_v1($1, $2)",
                &[&id, &serde_json::to_value(device).unwrap()],
            ).map_err(Error::SessionGet)?;

            if session_rows.len() > 0 {
                let session_row = session_rows.get(0);
                let mut session: session::Session = account.into();
                session.set_token(session_row.get("token"));
                return Ok(session);
            } else {
                let new_rows = conn.query(
                    "SELECT * FROM insert_account_session_v1($1, $2, $3, $4)",
                    &[
                        &id,
                        &session_create.get_token(),
                        &provider,
                        &(serde_json::to_value(device).unwrap()),
                    ],
                ).map_err(Error::SessionCreate)?;

                let session_row = new_rows.get(0);
                let mut session: session::Session = account.into();
                session.set_token(session_row.get("token"));
                return Ok(session);
            }
        } else {
            return Err(Error::Db(db::error::Error::RecordsNotFound));
        }
    }

    pub fn account_create(
        datastore: &DataStoreConn,
        session_create: &session::SessionCreate,
        device: &session::Device
    ) -> Result<session::Session> {
        let conn = datastore.pool.get_shard(0)?;

        let query =
            "SELECT * FROM insert_account_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)";
        let rows = conn.query(
            &query,
            &[
                &session_create.get_email(),
                &session_create.get_first_name(),
                &session_create.get_last_name(),
                &session_create.get_phone(),
                &session_create.get_apikey(),
                &session_create.get_password(),
                &session_create.get_approval(),
                &session_create.get_suspend(),
                &session_create.get_teams(),
                &session_create.get_registration_ip_address(),
                &session_create.get_trust_level(),
                &session_create.get_company_name(),
                &(serde_json::to_value(session_create.object_meta()).unwrap()),
                &(serde_json::to_value(session_create.type_meta()).unwrap()),
                &session_create.get_avatar(),
            ],
        ).map_err(Error::AccountCreate)?;
        if rows.len() > 0 {
            let row = rows.get(0);

            let account = row_to_account(row);

            let id = account.get_id().parse::<i64>().unwrap();

            let provider = match session_create.get_provider() {
                session::OAuthProvider::OpenID => "openid",
                _ => "password",
            };

            let rows = conn.query(
                "SELECT * FROM insert_account_session_v1($1, $2, $3, $4)",
                &[
                    &id,
                    &session_create.get_token(),
                    &provider,
                    &serde_json::to_value(device).unwrap(),
                ],
            ).map_err(Error::SessionCreate)?;

            let session_row = rows.get(0);
            let mut session: session::Session = account.into();
            session.set_token(session_row.get("token"));

            Ok(session)
        } else {
            return Err(Error::Db(db::error::Error::RecordsNotFound));
        }
    }

    pub fn get_account_by_email_fascade(&self, account_get: session::AccountGet) -> session::Account {
        let mut account = session::Account::new();
        account.set_email(account_get.get_email());
        self.expander
            .with_account(&mut account, PULL_DIRECTLY);
        account
    }

    pub fn get_account(
        datastore: &DataStoreConn,
        account_get: &session::AccountGet,
    ) -> Result<Option<session::Account>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = conn.query(
            "SELECT * FROM get_account_by_email_v1($1)",
            &[&account_get.get_email()],
        ).map_err(Error::AccountGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            return Ok(Some(row_to_account(row)));
        } else {
            Ok(None)
        }
    }

    pub fn get_account_by_id(
        datastore: &DataStoreConn,
        account_get_id: &session::AccountGetId,
    ) -> Result<Option<session::Account>> {
        let conn = datastore.pool.get_shard(0)?;
        let id = account_get_id.get_id().parse::<i64>().unwrap();
        let rows = conn.query("SELECT * FROM get_account_by_id_v1($1)", &[&id])
            .map_err(Error::AccountGetById)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            return Ok(Some(row_to_account(row)));
        } else {
            Ok(None)
        }
    }

    pub fn get_session(
        datastore: &DataStoreConn,
        session_get: &session::SessionGet,
    ) -> Result<Option<session::Session>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = conn.query(
            "SELECT * FROM get_account_session_by_email_token_v1($1, $2)",
            &[&session_get.get_email(), &session_get.get_token()],
        ).map_err(Error::SessionGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            let mut session = session::Session::new();
            let id: i64 = row.get("id");
            session.set_id(id.to_string());
            let email: String = row.get("email");
            session.set_email(email);
            let token: String = row.get("token");
            session.set_token(token);
            let api_key: String = row.get("api_key");
            session.set_apikey(api_key);
            return Ok(Some(session));
        } else {
            Ok(None)
        }
    }

    pub fn account_logout(
        datastore: &DataStoreConn,
        logout: &session::AccountTokenGet,
        device: &session::Device,
    ) -> Result<Option<session::Account>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_logout_v1($1,$2,$3)",
            &[
                &(logout.get_email() as String),
                &(logout.get_token() as String),
                &(serde_json::to_value(device).unwrap()),
            ],
        ).map_err(Error::SessionGet)?;

        if rows.len() != 0 {
            let row = rows.get(0);
            return Ok(Some(row_to_account(row)));
        } else {
            Ok(None)
        }
    }

    pub fn ldap_config_create(
        datastore: &DataStoreConn,
        ldap_config: &session::LdapConfig,
    ) -> Result<Option<session::LdapConfig>> {
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
        if rows.len() > 0 {
            let ldap = row_to_ldap_config(&rows.get(0))?;
            return Ok(Some(ldap));
        }
        Ok(None)
    }

    pub fn test_ldap_config(
        datastore: &DataStoreConn,
        get_id: &IdGet,
    ) -> Result<Option<session::Success>> {
        match Self::get_ldap_config(datastore, get_id) {
            Ok(Some(ldap_config)) => return test_ldap(ldap_config),
            Err(err) => Err(err),
            _ => return Err(Error::Db(db::error::Error::RecordsNotFound)),
        }
    }

    pub fn get_ldap_config(
        datastore: &DataStoreConn,
        get_id: &IdGet,
    ) -> Result<Option<session::LdapConfig>> {
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

    pub fn import_ldap_config(
        datastore: &DataStoreConn,
        get_id: &IdGet,
    ) -> Result<session::ImportResult> {
        match Self::get_ldap_config(datastore, get_id) {
            Ok(Some(ldap_config)) => {
                let importing_users = ldap_users(ldap_config)?;
                let mut imported_users = vec![];
                let imported: Vec<Result<session::Session>> = importing_users
                    .into_iter()
                    .map(|import_user| {
                        let mut add_account: session::SessionCreate = import_user.into();
                        let session = Self::account_create(
                            datastore,
                            &mut add_account,
                            &session::Device::new()
                        )?;
                        imported_users.push(session.get_email());
                        Ok(session)
                    })
                    .collect();

                let fail_count = &imported.iter().filter(|f| (*f).is_err()).count();

                let msg = format!("{} failure, {} successful", fail_count, imported.len(),);

                let mut impres = session::ImportResult::new();
                impres.set_result(msg);
                impres.set_users(imported_users);
                Ok(impres)
            }
            Err(e) => Err(e),
            _ => return Err(Error::Db(db::error::Error::RecordsNotFound)),
        }
    }

    pub fn saml_provider_create(
        datastore: &DataStoreConn,
        saml_provider: &session::SamlProvider,
    ) -> Result<Option<session::SamlProvider>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM insert_saml_provider_v1($1,$2,$3)",
            &[
                &(saml_provider.get_description() as String),
                &(saml_provider.get_idp_metadata() as String),
                &(saml_provider.get_sp_base_url() as String),
            ],
        ).map_err(Error::SamlProviderCreate)?;
        if rows.len() > 0 {
            let saml = row_to_saml_provider(&rows.get(0))?;
            return Ok(Some(saml));
        }
        Ok(None)
    }

    pub fn saml_provider_list_blank(datastore: &DataStoreConn) -> SamlOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_saml_provider_all_v1()", &[])
            .map_err(Error::SamlProviderGetResponse)?;

        let mut response = Vec::new();
        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_saml_provider(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn saml_show(
        datastore: &DataStoreConn,
        saml_provider_get: &IdGet,
    ) -> Result<Option<session::SamlProvider>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_saml_v1($1)",
            &[&(saml_provider_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::SamlProviderGet)?;
        if rows.len() > 0 {
            for row in rows {
                let saml = row_to_saml_provider(&row)?;
                return Ok(Some(saml));
            }
        }
        Ok(None)
    }

    pub fn oidc_provider_create(
        datastore: &DataStoreConn,
        oidc_provider: &session::OidcProvider,
    ) -> Result<Option<session::OidcProvider>> {
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
        if rows.len() > 0 {
            let oidc = row_to_oidc_provider(&rows.get(0))?;
            return Ok(Some(oidc));
        }
        Ok(None)
    }

    pub fn openid_provider_list_blank(datastore: &DataStoreConn) -> OpenIdOutputList {
        let conn = datastore.pool.get_shard(0)?;

        let rows = &conn.query("SELECT * FROM get_oidc_provider_all_v1()", &[])
            .map_err(Error::OpenidProviderGetResponse)?;

        let mut response = Vec::new();

        if rows.len() > 0 {
            for row in rows {
                response.push(row_to_oidc_provider(&row)?)
            }
            return Ok(Some(response));
        }
        Ok(None)
    }

    pub fn oidc_show(
        datastore: &DataStoreConn,
        oidc_provider_get: &IdGet,
    ) -> Result<Option<session::OidcProvider>> {
        let conn = datastore.pool.get_shard(0)?;
        let rows = &conn.query(
            "SELECT * FROM get_odic_v1($1)",
            &[&(oidc_provider_get.get_id().parse::<i64>().unwrap())],
        ).map_err(Error::OidcProviderGet)?;

        if rows.len() > 0 {
            for row in rows {
                let oidc = row_to_oidc_provider(&row)?;
                return Ok(Some(oidc));
            }
        }
        Ok(None)
    }
}

fn row_to_account(row: postgres::rows::Row) -> session::Account {
    let mut account = session::Account::with(
        serde_json::from_value(row.get("type_meta")).unwrap(),
        serde_json::from_value(row.get("object_meta")).unwrap(),
    );

    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");
    account.set_id(id.to_string());
    account.set_email(row.get("email"));
    account.set_password(row.get("password"));
    account.set_first_name(row.get("first_name"));
    account.set_last_name(row.get("last_name"));
    account.set_teams(row.get("teams"));
    account.set_apikey(row.get("api_key"));
    account.set_company_name(row.get("company_name"));
    account.set_trust_level(row.get("trust_level"));
    account.set_created_at(created_at.to_rfc3339());
    account
}

fn row_to_ldap_config(row: &postgres::rows::Row) -> Result<session::LdapConfig> {
    let mut ldap = session::LdapConfig::new();
    let id: i64 = row.get("id");
    let user_search: String = row.get("user_search");
    let group_search: String = row.get("group_search");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    ldap.set_id(id.to_string());
    ldap.set_host(row.get("host"));
    ldap.set_port(row.get("port"));
    ldap.set_enforce_starttls(row.get("enforce_starttls"));
    ldap.set_use_ldaps(row.get("use_ldaps"));
    ldap.set_lookup_dn(row.get("lookup_dn"));
    ldap.set_lookup_password(row.get("lookup_password"));
    ldap.set_ca_certs(row.get("ca_certs"));
    ldap.set_client_cert(row.get("client_cert"));
    let user_search: session::UserSearch = serde_json::from_str(&user_search).unwrap();
    let group_search: session::GroupSearch = serde_json::from_str(&group_search).unwrap();
    ldap.set_user_search(user_search);
    ldap.set_group_search(group_search);
    ldap.set_created_at(created_at.to_rfc3339());

    Ok(ldap)
}

fn test_ldap(ldap_data: session::LdapConfig) -> Result<Option<session::Success>> {
    let ldap = LDAPClient::new(ldap_data);
    if let Err(err) = ldap.connection() {
        return Err(err);
    }
    let mut success = session::Success::new();
    success.set_result("Success".to_string());
    Ok(Some(success))
}

fn ldap_users(ldap_data: session::LdapConfig) -> Result<Vec<LDAPUser>> {
    let ldap = LDAPClient::new(ldap_data);
    if let Err(err) = ldap.connection() {
        return Err(err);
    }
    ldap.search()
}

fn row_to_saml_provider(row: &postgres::rows::Row) -> Result<session::SamlProvider> {
    let mut saml = session::SamlProvider::new();
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

    saml.set_id(id.to_string());
    saml.set_description(row.get("description"));
    saml.set_idp_metadata(row.get("idp_metadata"));
    saml.set_sp_base_url(row.get("sp_base_url"));
    saml.set_created_at(created_at.to_rfc3339());

    Ok(saml)
}
fn row_to_oidc_provider(row: &postgres::rows::Row) -> Result<session::OidcProvider> {
    let mut oidc = session::OidcProvider::new();
    let id: i64 = row.get("id");
    let created_at = row.get::<&str, DateTime<Utc>>("created_at");

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
