// Copyright 2018 The Rio Advancement Inc

use db::data_store::DataStoreConn;
use session::models::{session as sessions, passticket};

use util::goofy_crypto::GoofyCrypto;
use protocol::api::session;
use protocol::api::session::*;
use rioos;
use util::token_target::TokenTarget;
use util::jwt_authenticator::JWTAuthenticator;
use super::super::error::{self, Result, Error};
use rand::{self, Rng};

#[derive(Clone, Debug)]
pub struct ServiceAccountAccess {}

impl ServiceAccountAccess {
    // it authenticates username and password values
    // it validate username and password are exists or not in database and return result in bool (true or false)
    // incase account is exists then it validate the password
    pub fn from_name_and_webtoken(datastore: &DataStoreConn, username: String, password: String) -> Result<bool> {
        let mut account_get = session::AccountGet::new();
        account_get.set_email(username.clone());
        account_get.set_password(password);
        match sessions::DataStore::get_account(&datastore, &account_get) {
            Ok(opt_account) => {
                let account = opt_account.unwrap();
                GoofyCrypto::new()
                    .verify_password(
                        &account.get_password().to_string(),
                        &account_get.get_password(),
                    )
                    .map_err(|e| {
                        error::Error::Auth(rioos::AuthErr {
                            error: String::from("Password match not found"),
                            error_description: format!("{}", e),
                        })
                    })?;

                Ok(true)
            }
            Err(err) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("Couldn't find {} in session.", username.clone()),
                    error_description: format!("{}", err),
                }))
            }
        }
    }
}
