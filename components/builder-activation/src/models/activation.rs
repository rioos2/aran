// Copyright 2018 The Rio Advancement Inc

//! The PostgreSQL backend for the DataStore.
use error::{Error, Result};

use protocol::api::base::{IdGet, MetaFields,WhoAmITypeMeta};
use protocol::api::activation;

use db::data_store::DataStoreConn;
use protocol::api::schema::type_meta_url;



pub struct DataStore<'a> {
    db: &'a DataStoreConn
}

impl<'a> DataStore<'a> {
    pub fn new(db: &'a DataStoreConn) -> Self {
        DataStore {
            db: db
        }
    }

    pub fn wizard(&self, license_status: IdGet) -> Result<Option<activation::Wizard>> {
        let mut wizard = row_to_wizard(license_status.get_id() == activation::ACTIVE.to_string());

        let conn = self.db.pool.get_shard(0)?;
        let rows = conn.query("SELECT * FROM get_accounts_v1_by_role($1)",
         &[&(vec![activation::BUILTIN_ROLE_RIOOS_SUPERUSER.to_string()])])
         .map_err(Error::WizardGet)?;

        wizard.set_registered(rows.len() > 0);

        Ok(Some(wizard))
    }

}


/// A convertor of postgres Row to the required structure.
/// In this case wizard.
fn row_to_wizard(status: bool) -> activation::Wizard {

    let mut wizard = activation::Wizard::new();
    let m = wizard.mut_meta(
        wizard.object_meta(),
        wizard.get_name(),
        wizard.get_account(),
    );
    let jackie = wizard.who_am_i();
    wizard.set_meta(type_meta_url(jackie), m);
    wizard.set_license(status);

    wizard
}
