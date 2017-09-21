// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use std::fmt;
use rand::{self, Rng};

use protocol::sessionsrv;

use config;
use auth::goofy_crypto::GoofyCrypto;

use super::super::error::{self, Result};

use db::data_store::DataStoreConn;
use session::session_ds::SessionDS;


// These OAuth scopes are required for a user to be authenticated. If this list is updated, then
// the front-end also needs to be updated in `components/builder-web/app/util.ts`. Both the
// front-end app and back-end app should have identical requirements to make things easier for
// our users and less cumbersome for us to message out.
// https://developer.github.com/v3/oauth/#scopes
const AUTH_SCOPES: &'static [&'static str] = &["user:email", "read:org"];
const TOKEN_LEN: usize = 18;


#[derive(Clone)]
pub struct PasswordAuthClient {}

impl PasswordAuthClient {
    pub fn new<T>(config: &T) -> Self
    where
        T: config::PasswordAuth,
    {
        PasswordAuthClient {}
    }

    //Generates a token of 15 ascii random character
    pub fn token(&self) -> Result<String> {
        Ok(
            rand::thread_rng()
                .gen_ascii_chars()
                .take(TOKEN_LEN)
                .collect(),
        )
    }

    //Encrypts a password text using pbkdf2 using a random salt.
    pub fn encrypt(&self, password_text: String) -> Result<String> {
        GoofyCrypto::new().encrypt_password(&password_text.to_string())
    }

    //Authenticates an user with email/password.
    //AccountGet has the attempted_password
    //Retrieved Account has the actual_password
    pub fn authenticate(&self, datastore: &DataStoreConn, account_get: &sessionsrv::AccountGet) -> Result<sessionsrv::Account> {

        match SessionDS::get_account(&datastore, &account_get) {
            Ok(opt_account) => {
                let account = opt_account.unwrap();
                GoofyCrypto::new()
                    .verify_password(
                        &account.get_password().to_string(),
                        &account_get.get_password(),
                    )
                    .map_err(|e| {
                        error::Error::Auth(AuthErr {
                            error: String::from("Password match not found"),
                            error_description: format!("{}", e),
                        })
                    })?;

                Ok(account)
            }
            Err(err) => {
                return Err(error::Error::Auth(AuthErr {
                    error: String::from("Account not found"),
                    error_description: format!("{}", err),
                }))
            }
        }

    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
    pub public_repos: Option<u32>,
    pub public_gists: Option<u32>,
    pub followers: Option<u32>,
    pub following: Option<u32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<User> for sessionsrv::Account {
    fn from(user: User) -> sessionsrv::Account {
        let mut account = sessionsrv::Account::new();
        account.set_name(user.login);
        if let Some(email) = user.email {
            account.set_email(email);
        }
        account
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthOk {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

impl AuthOk {
    pub fn missing_auth_scopes(&self) -> Vec<&'static str> {
        let mut scopes = vec![];
        for scope in AUTH_SCOPES.iter() {
            if !self.scope.split(",").collect::<Vec<&str>>().iter().any(
                |p| {
                    p == scope
                },
            )
            {
                scopes.push(*scope);
            }
        }
        scopes
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthErr {
    pub error: String,
    pub error_description: String,
}

impl fmt::Display for AuthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "err={}, desc={}", self.error, self.error_description)
    }
}

#[derive(Deserialize, Serialize)]
pub enum AuthResp {
    AuthOk,
    AuthErr,
}
