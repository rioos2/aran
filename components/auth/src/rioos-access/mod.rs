// Copyright 2018 The Rio Advancement Inc

//! A module containing the auth middleware of the HTTP server
pub mod service_account;
pub mod user_account;

use std::fmt;
use super::error;
use self::user_account::UserAccountAuthenticate;
use self::service_account::ServiceAccountAuthenticate;
use util::authenticatable::{Authenticatable, ToAuth};
use db::data_store::DataStoreConn;
use std::sync::Arc;


//Authenticate delegate main function of authentication
//Authenticate delegate struct support following authentication types
//1. user email and password
//2. user email and bearer token
//3. user email and jsonwebtoken
//4. serviceaccount name and jsonwebtoken
//
//Example of authenticate user email and token data:
//let delegate = AuthenticateDelegate::new(broker.clone());
//let auth_enum = Authenticatable::UserEmailAndToken {
//                    email: "info@megam.io",
//                   token: "jvhkrere vkrjvkrjvnjvhoivivn",
//                };
//let auth = delegate.authenticate(&auth_enum);
//
#[derive(Clone)]
pub struct AccessDelegate {
    conn: Arc<DataStoreConn>,
}

impl AccessDelegate {
    pub fn new(datastore: Arc<DataStoreConn>) -> Self {
        AccessDelegate { conn: datastore }
    }

    //authenticate method get inputs of "Authenticatable" enum
    //and verify enum type and proceed it
    pub fn access<T>(&self, auth: &T) -> error::Result<bool>
    where
        T: ToAuth,
    {
        match auth.to_auth() {
            Authenticatable::UserAndPass {
                username: u,
                password: p,
            } => UserAccountAccess::from_username_and_password(&self.conn, u.to_string(), p.to_string()),
            Authenticatable::ServiceAccountNameAndWebtoken {
                name: u,
                webtoken: p,
                key: k,
            } => ServiceAccountAccess::from_name_and_webtoken(&self.conn, u.to_string(), p.to_string(), k),
        }
    }
}
