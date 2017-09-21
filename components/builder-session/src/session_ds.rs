// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the SessionDS.
use chrono::prelude::*;
use error::{Result, Error};
use protocol::{sessionsrv, asmsrv, servicesrv};
use postgres;
use privilege;
use db::data_store::DataStoreConn;
use serde_json;


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
            "SELECT * FROM select_or_insert_account_v1($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
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
        let rows = conn.query(
            "SELECT * FROM get_account_by_id_v1($1)",
            &[&(account_get_id.get_id())],
        ).map_err(Error::AccountGetById)?;
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
    pub fn origin_create(datastore: &DataStoreConn, org_create: &sessionsrv::Origin) -> Result<Option<sessionsrv::Origin>> {
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
}

fn row_to_account(row: postgres::rows::Row) -> sessionsrv::Account {
    let mut account = sessionsrv::Account::new();
    let id: i64 = row.get("id");
    account.set_id(id.to_string());
    account.set_email(row.get("email"));
    account.set_name(row.get("name"));
    account.set_password(row.get("password"));
    account.set_apikey(row.get("api_key"));
    account
}


fn row_to_origin(row: &postgres::rows::Row) -> Result<sessionsrv::Origin> {
    let mut origin_data = sessionsrv::Origin::new();
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
