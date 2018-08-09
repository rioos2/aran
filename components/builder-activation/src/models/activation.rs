// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the DataStore.


use db::data_store::DataStoreConn;
use error::{Error, Result};
use protocol::api::activation;

use protocol::api::base::{IdGet, MetaFields, WhoAmITypeMeta};
use protocol::api::schema::type_meta_url;

use serde_json::from_str;


pub struct DataStore<'a> {
    db: &'a DataStoreConn,
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore { db: db }
    }

    pub fn wizard(&self, activation_completed: bool) -> Result<Option<activation::Wizard>> {
        let mut wizard = row_to_wizard(activation_completed);

        let conn = self.db.pool.get_shard(0)?;
        let rows = conn.query(
            "SELECT * FROM get_accounts_v1_by_team($1)",
            &[
                &(vec![activation::BUILTIN_TEAM_RIOOS_SUPERUSER.to_string()]),
            ],
        ).map_err(Error::WizardGet)?;

        wizard.set_registered(rows.len() > 0);

        Ok(Some(wizard))
    }
}


/// A convertor of postgres Row to the required structure.
/// In this case wizard.
fn row_to_wizard(activation_completed: bool) -> activation::Wizard {

    let mut wizard = activation::Wizard::new();
    let m = wizard.mut_meta(
        wizard.object_meta(),
        wizard.get_name(),
        wizard.get_account(),
    );
    let jackie = wizard.who_am_i();
    wizard.set_meta(type_meta_url(jackie), m);
    wizard.set_license(activation_completed);
    wizard
}
