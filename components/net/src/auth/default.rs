// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use std::error::Error as StdError;
use std::collections::HashMap;
use std::fmt;
use rand::{self, Rng};
use std::io::Read;
use std::result::Result as StdResult;
use std::time::Duration;

use hyper::{self, Url};
use hyper::status::StatusCode;
use hyper::header::{Authorization, Accept, Bearer, UserAgent, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;
use protocol::sessionsrv;
use protocol::net::{self, ErrCode};

use serde_json;

use config;
use error::{Error, Result};

const USER_AGENT: &'static str = "Aran-PasswordAuth";
const HTTP_TIMEOUT: u64 = 3_000;

// These OAuth scopes are required for a user to be authenticated. If this list is updated, then
// the front-end also needs to be updated in `components/builder-web/app/util.ts`. Both the
// front-end app and back-end app should have identical requirements to make things easier for
// our users and less cumbersome for us to message out.
// https://developer.github.com/v3/oauth/#scopes
const AUTH_SCOPES: &'static [&'static str] = &["user:email", "read:org"];
const TOKEN_LEN: usize = 15;


#[derive(Clone)]
pub struct PasswordAuthClient {
    pub url: String,
    pub web_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl PasswordAuthClient {
    pub fn new<T>(config: &T) -> Self
    where
        T: config::PasswordAuth,
    {
        PasswordAuthClient {
            url: config.github_url().to_string(),
            web_url: config.github_url().to_string(),
            client_id: config.github_client_id().to_string(),
            client_secret: config.github_client_secret().to_string(),
        }
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
    //Authenticates an user with email/password.
    pub fn authenticate(&self, session_create: &sessionsrv::SessionCreate, code: &str) -> Result<String> {
        /*let url = Url::parse(&format!(
            "{}/login/oauth/access_token?\
                                client_id={}&client_secret={}&code={}",
            self.web_url,
            self.client_id,
            self.client_secret,
            code
        )).unwrap();
        let mut rep = http_post(url)?;
        if rep.status.is_success() {
            let mut encoded = String::new();
            rep.read_to_string(&mut encoded)?;
            match serde_json::from_str::<AuthOk>(&encoded) {
                Ok(msg) => {
                    let missing = msg.missing_auth_scopes();
                    if missing.is_empty() {
                        Ok(msg.access_token)
                    } else {
                        let msg = format!("Missing OAuth scope(s), '{}'", missing.join(", "));
                        let err = net::err(net::ErrCode::AUTH_SCOPE, msg);
                        Err(Error::from(err))
                    }
                }
                Err(_) => {
                    match serde_json::from_str::<AuthErr>(&encoded) {
                        Ok(gh_err) => {
                            let err = net::err(net::ErrCode::ACCESS_DENIED, gh_err.error);
                            Err(Error::from(err))
                        }
                        Err(_) => {
                            let err = net::err(net::ErrCode::BAD_REMOTE_REPLY, "net:github:0");
                            Err(Error::from(err))
                        }
                    }
                }
            }
        } else {
            Err(Error::HTTP(rep.status))
        }*/
        let hello = String::from("Hello, world!");
        Ok(hello)
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
    pub error_uri: String,
}

impl fmt::Display for AuthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "err={}, desc={}, uri={}",
            self.error,
            self.error_description,
            self.error_uri
        )
    }
}

#[derive(Deserialize, Serialize)]
pub enum AuthResp {
    AuthOk,
    AuthErr,
}
