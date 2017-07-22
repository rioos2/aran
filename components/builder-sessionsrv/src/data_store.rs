// Copyright (c) 2017 RioCorp Inc.

//! The PostgreSQL backend for the Account Server.

use db::pool::Pool;
use hab_net::privilege;
use protocol::sessionsrv;
use postgres;
use protobuf;

use config::Config;
use error::{Result, Error};

#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: Pool,
}

impl DataStore {
    pub fn new(config: &Config) -> Result<DataStore> {
        let pool = Pool::new(&config.datastore, config.shards.clone())?;
        Ok(DataStore { pool: pool })
    }

    pub fn from_pool(pool: Pool) -> Result<DataStore> {
        Ok(DataStore { pool: pool })
    }

    pub fn setup(&self) -> Result<()> {
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        Ok(())
    }

    fn row_to_account(&self, row: postgres::rows::Row) -> sessionsrv::Account {
        let mut account = sessionsrv::Account::new();
        let id: i64 = row.get("id");
        account.set_id(id as u64);
        account.set_email(row.get("email"));
        account.set_name(row.get("name"));
        account
    }

    pub fn find_or_create_account_via_session(
        &self,
        session_create: &sessionsrv::SessionCreate,
        is_admin: bool,
        is_early_access: bool,
        is_build_worker: bool,
    ) -> Result<sessionsrv::Session> {
        let conn = self.pool.get(session_create)?;
        let rows = conn.query(
            "SELECT * FROM select_or_insert_account_v1($1, $2)",
            &[&session_create.get_name(), &session_create.get_email()],
        ).map_err(Error::AccountCreate)?;
        let row = rows.get(0);
        let account = self.row_to_account(row);

        let provider = match session_create.get_provider() {
            sessionsrv::OAuthProvider::GitHub => "github",
        };
        let rows = conn.query(
            "SELECT * FROM insert_account_session_v1($1, $2, $3, $4, $5, $6, $7)",
            &[
                &(account.get_id() as i64),
                &session_create.get_token(),
                &provider,
                &(session_create.get_extern_id() as i64),
                &is_admin,
                &is_early_access,
                &is_build_worker,
            ],
        ).map_err(Error::AccountGetById)?;
        let session_row = rows.get(0);

        let mut session: sessionsrv::Session = account.into();
        session.set_token(session_row.get("token"));

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

        Ok(session)
    }

    pub fn get_account(
        &self,
        account_get: &sessionsrv::AccountGet,
    ) -> Result<Option<sessionsrv::Account>> {
        let conn = self.pool.get(account_get)?;
        let rows = conn.query(
            "SELECT * FROM get_account_by_name_v1($1)",
            &[&account_get.get_name()],
        ).map_err(Error::AccountGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_account(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_account_by_id(
        &self,
        account_get_id: &sessionsrv::AccountGetId,
    ) -> Result<Option<sessionsrv::Account>> {
        let conn = self.pool.get(account_get_id)?;
        let rows = conn.query(
            "SELECT * FROM get_account_by_id_v1($1)",
            &[&(account_get_id.get_id() as i64)],
        ).map_err(Error::AccountGetById)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_account(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_session(
        &self,
        session_get: &sessionsrv::SessionGet,
    ) -> Result<Option<sessionsrv::Session>> {
        let conn = self.pool.get(session_get)?;
        let rows = conn.query(
            "SELECT * FROM get_account_session_v1($1, $2)",
            &[&session_get.get_name(), &session_get.get_token()],
        ).map_err(Error::SessionGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            let mut session = sessionsrv::Session::new();
            let id: i64 = row.get("id");
            session.set_id(id as u64);
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
            if row.get("is_early_access") {
                flags.insert(privilege::EARLY_ACCESS);
            }
            if row.get("is_build_worker") {
                flags.insert(privilege::BUILD_WORKER);
            }
            session.set_flags(flags.bits());
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub fn get_origins_by_account(
        &self,
        request: &sessionsrv::AccountOriginListRequest,
    ) -> Result<sessionsrv::AccountOriginListResponse> {
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

    pub fn accept_origin_invitation(
        &self,
        request: &sessionsrv::AccountOriginInvitationAcceptRequest,
    ) -> Result<()> {
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

    pub fn create_account_origin_invitation(
        &self,
        invitation_create: &sessionsrv::AccountOriginInvitationCreate,
    ) -> Result<()> {
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

    pub fn list_invitations(
        &self,
        ailr: &sessionsrv::AccountInvitationListRequest,
    ) -> Result<sessionsrv::AccountInvitationListResponse> {
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
}
