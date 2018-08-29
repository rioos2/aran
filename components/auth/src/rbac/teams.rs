// Copyright 2018 The Rio Advancement Inc

use super::super::error::{Error, Result};
use db::data_store::DataStoreConn;
use protocol;
use protocol::api::authorize::Teams;
use protocol::api::base::IdGet;
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_TEAM, CACHE_PREFIX_POLICY_MEMBER};
use auth::models::{team, policy_members};
use serde_json;


/// Account fascade: In this fascade declare the cache service fn for getting accounts from database 
/// and store it to inmemory cache.
/// Then RBAC middleware get account from cache for verify account accesibility.
//
#[derive(Clone)]
pub struct TeamsFascade {
    pub conn: Box<DataStoreConn>,
}

impl TeamsFascade {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        TeamsFascade { conn: datastore }
    }

     pub fn get_by_id(&self, team_id: IdGet) -> Teams {
        team::DataStore::new(&self.conn).show_by_fascade(team_id)
    }
}

impl ExpanderSender for TeamsFascade {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let team_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_TEAM.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {                
                team::DataStore::new(&_conn).show(&id)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));       

        let _conn = self.conn.clone();
        let policy_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_POLICY_MEMBER.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                debug!("» Policy Members live load for ≈ {}", id);
                policy_members::DataStore::new(&_conn).list_by_team(&id)
                    .ok()
                    .and_then(|e| serde_json::to_string(&e).ok())
            }),
        ));
       
        &self.conn.expander.with(policy_service); 
        &self.conn.expander.with(team_service);
    }
}


