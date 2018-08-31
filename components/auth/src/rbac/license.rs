// Copyright 2018 The Rio Advancement Inc

use super::super::error::{Error, Result};
use db::data_store::DataStoreConn;
use entitlement::models::license;
use protocol::api::base::IdGet;
use protocol::api::licenses::{Licenses, LicenseStatus};
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_LICENSE};


/// LicensesFascade fascade: Licenses provides ability to verify the license
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
#[derive(Clone)]
pub struct LicensesFascade {
    pub conn: Box<DataStoreConn>,
}

impl LicensesFascade {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        LicensesFascade { conn: datastore }
    }

    pub fn get_by_name(&self, name: String) -> Result<Licenses> {
        let license = license::DataStore::new(&self.conn).show(IdGet::with_id(name));
        match LicenseStatus::status(&license.get_status()) {
            LicenseStatus::ACTIVE | LicenseStatus::TRIAL => Ok(license),
            LicenseStatus::INVALID => Err(Error::EntitlementError(format!("License Invalid"))),
            LicenseStatus::EXPIRED => Err(Error::EntitlementError(format!("License expired"))),
        }
    }
}

use serde_json;
impl ExpanderSender for LicensesFascade {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let license_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_LICENSE.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                license::DataStore::new(&_conn)
                    .license_show_by_name(&id)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));

        &self.conn.expander.with(license_service);
    }
}
