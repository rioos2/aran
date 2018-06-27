// Copyright 2018 The Rio Advancement Inc

use entitlement::models::license;
use db::data_store::DataStoreConn;
use protocol::api::licenses::Licenses;
use protocol::api::base::IdGet;
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_LICENSE};

/// permission fascade: Permissions provides ability to declare the Permissions
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
#[derive(Clone)]
pub struct LicensesFascade {
    conn: Box<DataStoreConn>,
}

impl LicensesFascade {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        LicensesFascade { conn: datastore }
    }

    pub fn get_by_name(&self, name: IdGet) -> Licenses {
        license::DataStore::new(&self.conn).get_by_name_fascade(name)
    }
}

use serde_json;
impl ExpanderSender for LicensesFascade {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let license_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_LICENSE.to_string(),
            Box::new(move |id: IdGet| -> Option<String> { license::DataStore::new(&_conn).license_show_by_name(&id).ok().and_then(|p| serde_json::to_string(&p).ok()) }),
        ));

        &self.conn.expander.with(license_service);
    }
}
