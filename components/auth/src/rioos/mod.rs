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
pub struct AuthenticateDelegate {
    conn: Arc<DataStoreConn>,
}

impl AuthenticateDelegate {
    pub fn new(datastore: Arc<DataStoreConn>) -> Self {
        AuthenticateDelegate { conn: datastore }
    }

    //authenticate method get inputs of "Authenticatable" enum
    //and verify enum type and proceed it
    pub fn authenticate<T>(&self, auth: &T) -> error::Result<bool>
    where
        T: ToAuth,
    {
        match auth.to_auth() {
            Authenticatable::UserAndPass {
                username: u,
                password: p,
            } => UserAccountAuthenticate::from_username_and_password(&self.conn, u.to_string(), p.to_string()),
            Authenticatable::PassTicket { token: t } => UserAccountAuthenticate::from_passticket(&self.conn, t.to_string()),
            Authenticatable::UserEmailAndToken { email: u, token: p } => UserAccountAuthenticate::from_email_and_token(&self.conn, u.to_string(), p.to_string()),
            Authenticatable::UserEmailAndWebtoken {
                email: u,
                webtoken: p,
            } => UserAccountAuthenticate::from_email_and_webtoken(&self.conn, u.to_string(), p.to_string()),
            Authenticatable::ServiceAccountNameAndWebtoken {
                name: u,
                webtoken: p,
                key: k,
            } => ServiceAccountAuthenticate::from_name_and_webtoken(&self.conn, u.to_string(), p.to_string(), k.to_string()),
        }
    }
}

//AuthErr struct is contains authetication error and their descriptions
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthErr {
    pub error: String,
    pub error_description: String,
}

impl AuthErr {
    pub fn description(&self) -> &str {
        return &self.error_description;
    }
}

impl fmt::Display for AuthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
