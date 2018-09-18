// Copyright 2018 The Rio Advancement Inc

use db::data_store::DataStoreConn;
use protocol::api::base::IdGet;
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_POLICY};
use auth::models::{policy};
use serde_json;
use protocol::api::authorize::PoliciesForLevel;


/// Account fascade: In this fascade declare the cache service fn for getting accounts from database 
/// and store it to inmemory cache.
/// Then RBAC middleware get account from cache for verify account accesibility.
//
#[derive(Clone)]
pub struct PolicyFascade {
    pub conn: Box<DataStoreConn>,
}

impl PolicyFascade {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        PolicyFascade { conn: datastore }
    }

     pub fn list_by_level(&self, level: IdGet) -> PoliciesForLevel {
        policy::DataStore::new(&self.conn).list_by_level_fascade(level)
    }
}

impl ExpanderSender for PolicyFascade {
    fn with_cache(&mut self) {        

        let _conn = self.conn.clone();
        let policy_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_POLICY.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                debug!("» Policy live load for ≈ {}", id);
                policy::DataStore::new(&_conn).list_by_level(&id)
                    .ok()
                    .and_then(|e| serde_json::to_string(&e).ok())
            }),
        ));
       
        &self.conn.expander.with(policy_service); 
    }
}


