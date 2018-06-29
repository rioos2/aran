// Copyright 2018 The Rio Advancement Inc

use entitlement::models::license;
use db::data_store::DataStoreConn;
use protocol::api::licenses::Licenses;
use protocol::api::base::IdGet;
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_LICENSE};
use super::super::error::{Error, Result};


/// permission fascade: Permissions provides ability to declare the Permissions
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
#[derive(Clone)]
pub struct LicensesFascade {
    pub conn: Box<DataStoreConn>,
}

impl LicensesFascade {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        LicensesFascade { conn: datastore }
    }

    pub fn get_by_name(&self, name: String) -> Result<Licenses> {
        let license = license::DataStore::new(&self.conn).get_by_name_fascade(IdGet::with_id(name));
        match LicenseStaus::status(&license.get_status()) {
            LicenseStaus::ACTIVE => Ok(license),
            LicenseStaus::NOTFOUND => Err(Error::EntitlementError(format!("License Not Found"))),
            LicenseStaus::EXPIRY => Err(Error::EntitlementError(format!("License expired"))),
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
                println!("--------------license_service---------------------{:?}", id);
                license::DataStore::new(&_conn)
                    .license_show_by_name(&id)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));

        &self.conn.expander.with(license_service);
    }
}

enum LicenseStaus {
    ACTIVE,
    EXPIRY,
    NOTFOUND,
}

impl LicenseStaus {
    pub fn status(status: &str) -> LicenseStaus {
        match &status[..] {
            "active" => LicenseStaus::ACTIVE,
            "expiry" => LicenseStaus::EXPIRY,
            "" => LicenseStaus::NOTFOUND,
            _ => LicenseStaus::EXPIRY,
        }
    }
}
