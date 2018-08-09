// Copyright 2018 The Rio Advancement Inc

//! A collection of permissions by  [assembly, assembly_factory, for the HTTP server

use auth::models::permission;
use db::data_store::DataStoreConn;
use protocol::api::authorize::PermissionsForTeam;
use protocol::api::base::IdGet;
use protocol::cache::{ExpanderSender, NewCacheServiceFn, CACHE_PREFIX_PERMISSION};

/// permission fascade: Permissions provides ability to declare the Permissions
/// and manage them.
/// Needs a Datastore mapper, hence a DataStoreConn needs to be sent in.
//
#[derive(Clone)]
pub struct Permissions {
    conn: Box<DataStoreConn>,
}

impl Permissions {
    pub fn new(datastore: Box<DataStoreConn>) -> Self {
        Permissions { conn: datastore }
    }

    /*pub fn list_by_email(&self, email: IdGet) -> PermissionsForAccount {
        permission::DataStore::new(&self.conn).list_by_email_fascade(email)
    }*/

    pub fn list_by_team(&self, team: IdGet) -> PermissionsForTeam {
        permission::DataStore::new(&self.conn).list_by_team_fascade(team)
    }
}

use serde_json;
impl ExpanderSender for Permissions {
    fn with_cache(&mut self) {
        let _conn = self.conn.clone();
        let permission_service = Box::new(NewCacheServiceFn::new(
            CACHE_PREFIX_PERMISSION.to_string(),
            Box::new(move |id: IdGet| -> Option<String> {
                debug!("« ExpanderSender GET: with cache ≈ {:?}", id);
                permission::DataStore::new(&_conn)
                    .list_by_team_name(&id)
                    .ok()
                    .and_then(|p| serde_json::to_string(&p).ok())
            }),
        ));

        &self.conn.expander.with(permission_service);
    }
}
