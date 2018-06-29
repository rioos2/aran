// Copyright 2018 The Rio Advancement Inc

use super::super::error::{self, Error, Result};
use db::data_store::DataStoreConn;
use protocol::api::session;
use protocol::api::session::*;
use rand::{self, Rng};
use rand::distributions::Alphanumeric;
use rioos;
use session::models::{passticket, session as sessions};
use util::goofy_crypto::GoofyCrypto;
use util::jwt_authenticator::JWTAuthenticator;
use util::token_target::TokenTarget;

const TOKEN_LEN: usize = 18;
const LEGACYUSERACCOUNTISSUER: &'static str = "rioos_sh/useraccount";
const USERACCOUNTNAMECLAIM: &'static str = "rioos_sh/useraccount/user-account.name";
const USERACCOUNTUIDCLAIM: &'static str = "rioos_sh/useraccount/user-account.uid";
const SECRETUIDCLAIM: &'static str = "rioos_sh/useraccount/secret.uid";
const SECRETNAMECLAIM: &'static str = "rioos_sh/useraccount/secret.name";

#[derive(Clone, Debug)]
pub struct UserAccountAuthenticate {}

impl UserAccountAuthenticate {
    //Generates a token of 18 ascii random character
    pub fn token() -> Result<String> {
        Ok(rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(TOKEN_LEN)
            .collect())
    }

    //Encrypts a password text using pbkdf2 using a random salt.
    pub fn encrypt(password_text: String) -> Result<String> {
        GoofyCrypto::new().encrypt_password(&password_text.to_string())
    }

    // it authenticates username and password values
    // it validate username and password are exists or not in database and return result in bool (true or false)
    // incase account is exists then it validate the password
    pub fn from_username_and_password(
        datastore: &DataStoreConn,
        username: String,
        password: String,
    ) -> Result<bool> {
        let mut account_get = session::AccountGet::new();
        account_get.set_email(username.clone());
        account_get.set_password(password);
        try!(get_account(&datastore, account_get, true));
        Ok(true)
    }

    // it authenticates username and bearer token values
    // it checks account is exists or not in database
    // then check valid bearer token and validate their expiry period
    // otherwise it returns error response
    pub fn from_email_and_token(
        datastore: &DataStoreConn,
        email: String,
        token: String,
    ) -> Result<bool> {
        let tk_target = TokenTarget::new(email.to_string(), token.to_string());
        let request: SessionGet = tk_target.into();

        match sessions::DataStore::get_session(datastore, &request) {
            Ok(Some(_session)) => {
                return Ok(true);
            }
            Ok(None) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!(
                        "Couldn't find {} or {} token expired in session.",
                        email, token
                    ),
                    error_description: "Unauthorized".to_string(),
                }));
            }
            Err(err) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("Error occurred during session verification"),
                    error_description: format!("{}", err),
                }))
            }
        }
    }

    // it authenticates passticket token values
    // it checks account is exists or not in database
    // then check valid bearer token and validate their expiry period
    // otherwise it returns error response
    pub fn from_passticket(datastore: &DataStoreConn, passticket_id: String) -> Result<bool> {
        match passticket::DataStore::get_passticket(datastore, &passticket_id) {
            Ok(Some(passticket)) => {
                match passticket::DataStore::remove_passticket(datastore, passticket) {
                    Ok(_) => Ok(true),
                    Err(err) => Err(Error::OldPassticketMustBeRemoved(format!("{}", err))),
                }
            }
            Ok(None) => return Err(Error::PassticketMismatch),
            Err(err) => return Err(Error::CantVerifyPassticket(format!("{}", err))),
        }
    }

    // it authenticates user email and JWT token values
    // first it validates some static header and payload claims
    // then token is valid or not
    pub fn from_email_and_webtoken(
        datastore: &DataStoreConn,
        email: String,
        webtoken: String,
    ) -> Result<bool> {
        let mut account_get = session::AccountGet::new();
        account_get.set_email(email);
        try!(get_account(&datastore, account_get, false));

        let jwt = try!(JWTAuthenticator::new(webtoken.clone()));
        try!(jwt.has_correct_issuer(LEGACYUSERACCOUNTISSUER));
        try!(jwt.has_correct_subject(USERACCOUNTNAMECLAIM));
        try!(jwt.has_secret_name_claim(SECRETNAMECLAIM));
        try!(jwt.has_account_uid_claim(USERACCOUNTUIDCLAIM));
        try!(jwt.has_correct_token_from_secret(datastore, SECRETUIDCLAIM));
        Ok(true)
    }
}

fn get_account(conn: &DataStoreConn, account_get: AccountGet, verify_password: bool) -> Result<()> {
    match sessions::DataStore::get_account(&conn, &account_get) {
        Ok(Some(opt_account)) => {
            if verify_password {
                GoofyCrypto::new()
                    .verify_password(
                        &opt_account.get_password().to_string(),
                        &account_get.get_password(),
                    )
                    .map_err(|e| {
                        error::Error::Auth(rioos::AuthErr {
                            error: String::from("Password match not found"),
                            error_description: format!("{}", e),
                        })
                    })?;
                return Ok(());
            }
            Ok(())
        }
        Err(err) => {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!(
                    "Error while retriving session for {}.",
                    account_get.get_email()
                ),
                error_description: format!("{}", err),
            }))
        }
        Ok(None) => {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Couldn't find {} in session.", account_get.get_email()),
                error_description: "Unauthorized".to_string(),
            }))
        }
    }
}
