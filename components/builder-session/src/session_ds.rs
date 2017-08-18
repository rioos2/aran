// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the SessionDS.

use chrono::{DateTime, UTC};
use error::{Result, Error};
use protocol::sessionsrv;
use postgres;
use privilege;
use db::data_store::DataStoreConn;

pub struct SessionDS;

impl SessionDS {
    //For new users to onboard in Rio/OS, which takes the full account creation arguments and returns the Session which has the token.
    //The default role and permission for the user is
    //The default origin is
    pub fn account_create(datastore: &DataStoreConn, session_create: &sessionsrv::SessionCreate) -> Result<sessionsrv::Session> {
        //call and do find_or_create_account_via_session
        SessionDS::find_or_create_account_via_session(
            datastore,
            session_create,
            true,
            false,
        )
        //do find_or_create_default_role_permission
        //do find_or_create_default_origin
        //return Session
    }

    pub fn find_or_create_account_via_session(datastore: &DataStoreConn, session_create: &sessionsrv::SessionCreate, is_admin: bool, is_service_access: bool) -> Result<sessionsrv::Session> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = conn.query(
            "SELECT * FROM select_or_insert_account_v1($1)",
            &[&session_create.get_email()],
        ).map_err(Error::AccountCreate)?;

        let row = rows.get(0);
        let account = row_to_account(row);

        let provider = match session_create.get_provider() {
            sessionsrv::OAuthProvider::GitHub => "openid",
            _ => "password",
        };

        let rows = conn.query(
            "SELECT * FROM insert_account_session_v1($1, $2, $3, $4, $5, $6, $7)",
            &[
                &(account.get_id()),
                &session_create.get_token(),
                &provider,
                &(session_create.get_extern_id()),
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
            "SELECT * FROM get_account_by_name_v1($1)",
            &[&account_get.get_name()],
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

    pub fn get_session(datastore: &DataStoreConn, session_get: &sessionsrv::SessionGet) -> Result<Option<sessionsrv::Session>> {
        let conn = datastore.pool.get_shard(0)?;

        let rows = conn.query(
            "SELECT * FROM get_account_session_v1($1, $2)",
            &[&session_get.get_name(), &session_get.get_token()],
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

    /*pub fn get_origins_by_account(&self, request: &sessionsrv::AccountOriginListRequest) -> Result<sessionsrv::AccountOriginListResponse> {
        let conn = self.pool.get(request)?;
        let rows = conn.query(
            "SELECT * FROM get_account_origins_v1($1)",
            &[&(request.get_account_id() as i64)],
        ).map_err(Error::OriginAccountList)?;
        let mut response = sessionsrv::AccountOriginListResponse::new();
        response.set_account_id(request.get_account_id());
        let mut origins = protobuf::RepeatedField::new();

        if rows.len() > 0 {
            for row in rows.iter() {
                origins.push(row.get("origin_name"));
            }
        }
        response.set_origins(origins);
        Ok(response)
    }

    pub fn accept_origin_invitation(&self, request: &sessionsrv::AccountOriginInvitationAcceptRequest) -> Result<()> {
        let conn = self.pool.get(request)?;
        let tr = conn.transaction().map_err(Error::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM accept_account_invitation_v1($1, $2)",
            &[&(request.get_invite_id() as i64), &request.get_ignore()],
        ).map_err(Error::AccountOriginInvitationAccept)?;
        tr.commit().map_err(Error::DbTransactionCommit)?;
        Ok(())
    }

    pub fn create_origin(&self, request: &sessionsrv::AccountOriginCreate) -> Result<()> {
        let conn = self.pool.get(request)?;
        conn.execute(
            "SELECT * FROM insert_account_origin_v1($1, $2, $3, $4)",
            &[
                &(request.get_account_id() as i64),
                &request.get_account_name(),
                &(request.get_origin_id() as i64),
                &request.get_origin_name(),
            ],
        ).map_err(Error::OriginCreate)?;
        Ok(())
    }

    pub fn create_account_origin_invitation(&self, invitation_create: &sessionsrv::AccountOriginInvitationCreate) -> Result<()> {
        let conn = self.pool.get(invitation_create)?;
        let _rows = conn.query(
            "SELECT * FROM insert_account_invitation_v1($1, $2, $3, $4, $5, $6)",
            &[
                &(invitation_create.get_origin_id() as i64),
                &invitation_create.get_origin_name(),
                &(invitation_create.get_origin_invitation_id() as i64),
                &(invitation_create.get_account_id() as i64),
                &invitation_create.get_account_name(),
                &(invitation_create.get_owner_id() as i64),
            ],
        ).map_err(Error::AccountOriginInvitationCreate)?;
        Ok(())
    }

    pub fn list_invitations(&self, ailr: &sessionsrv::AccountInvitationListRequest) -> Result<sessionsrv::AccountInvitationListResponse> {
        let conn = self.pool.get(ailr)?;
        let rows = &conn.query(
            "SELECT * FROM get_invitations_for_account_v1($1)",
            &[&(ailr.get_account_id() as i64)],
        ).map_err(Error::AccountOriginInvitationList)?;

        let mut response = sessionsrv::AccountInvitationListResponse::new();
        response.set_account_id(ailr.get_account_id());
        let mut invitations = protobuf::RepeatedField::new();
        for row in rows {
            let mut oi = sessionsrv::AccountOriginInvitation::new();
            let oi_id: i64 = row.get("id");
            oi.set_id(oi_id as u64);
            let oi_account_id: i64 = row.get("account_id");
            oi.set_account_id(oi_account_id as u64);
            oi.set_account_name(row.get("account_name"));
            let oi_origin_id: i64 = row.get("origin_id");
            oi.set_origin_id(oi_origin_id as u64);
            oi.set_origin_name(row.get("origin_name"));
            let oi_owner_id: i64 = row.get("owner_id");
            oi.set_owner_id(oi_owner_id as u64);
            let oi_origin_invitation_id: i64 = row.get("origin_invitation_id");
            oi.set_origin_invitation_id(oi_origin_invitation_id as u64);
            invitations.push(oi);
        }
        response.set_invitations(invitations);
        Ok(response)
    }
    */
}

fn row_to_account(row: postgres::rows::Row) -> sessionsrv::Account {
    let mut account = sessionsrv::Account::new();
    let id = row.get("id");
    account.set_id(id);
    account.set_email(row.get("email"));
    account.set_name(row.get("name"));
    account
}
