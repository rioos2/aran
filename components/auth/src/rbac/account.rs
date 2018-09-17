// Copyright 2018 The Rio Advancement Inc

use super::super::error::{Error, Result};
use db::data_store::DataStoreConn;
use session::models::{session as sessions};
use protocol;
use protocol::api::session;
use protocol::api::base::IdGet;
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_ACCOUNT, CACHE_PREFIX_SERVICEACCOUNT};
use serviceaccount::models::service_account;
use serde_json;


/// Account fascade: In this fascade declare the cache service fn for getting accounts from database 
/// and store it to inmemory cache.
/// Then RBAC middleware get account from cache for verify account accesibility.
//
#[derive(Clone)]
pub struct AccountsFascade {
    pub conn: Box<DataStoreConn>,
}

impl AccountsFascade {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        AccountsFascade { conn: datastore }
    }

     pub fn get_by_email(&self, account: session::AccountGet) -> session::Account {
        sessions::DataStore::new(&self.conn).get_account_by_email_fascade(account)
    }
}

impl ExpanderSender for AccountsFascade {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let account_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_ACCOUNT.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                let mut account_get = session::AccountGet::new();
                account_get.set_email(id.get_id());
                sessions::DataStore::get_account(&_conn, &account_get)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));

        &self.conn.expander.with(account_service);
    }
}


/// ServiceAccount fascade: In this fascade declare the cache service fn for getting service_accounts from database 
/// and store it to inmemory cache.
/// Then RBAC middleware get service_account team from cache for verify account accesibility.
//
#[derive(Clone)]
pub struct ServiceAccountsFascade {
    pub conn: Box<DataStoreConn>,
}

impl ServiceAccountsFascade {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        ServiceAccountsFascade { conn: datastore }
    }

     pub fn get_by_name(&self, account: IdGet) -> protocol::api::service_account::ServiceAccount {
        service_account::DataStore::new(&self.conn).get_service_account_by_name_fascade(&account)
    }
}

impl ExpanderSender for ServiceAccountsFascade {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let account_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_SERVICEACCOUNT.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {               
                service_account::DataStore::show(&_conn, &id)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));

        &self.conn.expander.with(account_service);
    }
}
