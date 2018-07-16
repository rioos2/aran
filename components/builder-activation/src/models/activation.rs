// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the DataStore.
use error::{Error, Result};

use protocol::api::base::{IdGet, MetaFields,WhoAmITypeMeta};
use protocol::api::session;

use db;
use db::data_store::DataStoreConn;
use postgres;
use serde_json;
use protocol::api::schema::type_meta_url;

pub const BUILTIN_ROLE_RIOOS_SUPERUSER: &'static str = "RIOOS:SUPERUSER";
pub const ACTIVE: &'static str = "active";

pub struct DataStore<'a> {
    db: &'a DataStoreConn
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore {
            db: db
        }
    }

    pub fn wizard(&self, license_status: IdGet) -> Result<Option<session::Wizard>> {
        let mut wizard = session::Wizard::new();
        let m = wizard.mut_meta(
            wizard.object_meta(),
            wizard.get_name(),
            wizard.get_account(),
        );
        let jackie = wizard.who_am_i();
        wizard.set_meta(type_meta_url(jackie), m);
        if license_status.get_id() == ACTIVE.to_string() {
            wizard.set_license(true);
        }

        let conn = self.db.pool.get_shard(0)?;
        let rows = conn.query("SELECT * FROM get_accounts_v1_by_role($1)",
         &[&(vec![BUILTIN_ROLE_RIOOS_SUPERUSER.to_string()])])
         .map_err(Error::WizardGet)?;

        if rows.len() > 0 {
            wizard.set_registered(true);
        }
        Ok(Some(wizard))
    }

}
