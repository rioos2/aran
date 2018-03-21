// Copyright 2018 The Rio Advancement Inc

use db::data_store::DataStoreConn;
use session::models::session::SessionDS;
use session::models::otp::OtpDS;

use util::goofy_crypto::GoofyCrypto;
use protocol::api::session;
use protocol::api::session::*;
use rioos;
use util::token_target::TokenTarget;
use util::jwt_authenticator::JWTAuthenticator;
use super::super::error::{self, Result, Error};
use rand::{self, Rng};

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
        Ok(
            rand::thread_rng()
                .gen_ascii_chars()
                .take(TOKEN_LEN)
                .collect(),
        )
    }

    //Encrypts a password text using pbkdf2 using a random salt.
    pub fn encrypt(password_text: String) -> Result<String> {
        GoofyCrypto::new().encrypt_password(&password_text.to_string())
    }

    // it authenticates username and password values
    // it validate username and password are exists or not in database and return result in bool (true or false)
    // incase account is exists then it validate the password
    pub fn from_username_and_password(datastore: &DataStoreConn, username: String, password: String) -> Result<bool> {
        let mut account_get = session::AccountGet::new();
        account_get.set_email(username.clone());
        account_get.set_password(password);
        match SessionDS::get_account(&datastore, &account_get) {
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

    // it authenticates username and bearer token values
    // it checks account is exists or not in database
    // then check valid bearer token and validate their expiry period
    // otherwise it returns error response
    pub fn from_email_and_token(datastore: &DataStoreConn, email: String, token: String) -> Result<bool> {
        let tk_target = TokenTarget::new(email.to_string(), token.to_string());
        let request: SessionGet = tk_target.into();

        match SessionDS::get_session(datastore, &request) {
            Ok(Some(_session)) => Ok(true),
            Ok(None) => {
                let mut session_tk: SessionCreate = SessionCreate::new();
                session_tk.set_email(email.to_string());
                session_tk.set_token(token.to_string());

                let _session = try!(session_create(datastore, session_tk));
                return Ok(true);
            }
            Err(err) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("Couldn't find {} in session.", email),
                    error_description: format!("{}", err),
                }))
            }
        }
    }


    // it authenticates otp token values
    // it checks account is exists or not in database
    // then check valid bearer token and validate their expiry period
    // otherwise it returns error response
    pub fn from_otp(datastore: &DataStoreConn, otp: String) -> Result<bool> {
        match OtpDS::get_otp(datastore, &otp) {
            Ok(Some(otp)) => {
                match OtpDS::remove_otp(datastore, otp) {
                    Ok(_) => Ok(true),
                    Err(err) => Err(Error::OldOTPMustBeRemoved(format!("{}", err))),
                }
            }
            Ok(None) => return Err(Error::OTPMismatch),
            Err(err) => return Err(Error::CantVerifyOT(format!("{}", err))),
        }
    }

    // it authenticates user email and JWT token values
    // first it validates some static header and payload claims
    // then token is valid or not
    pub fn from_email_and_webtoken(datastore: &DataStoreConn, email: String, webtoken: String) -> Result<bool> {
        let jwt = try!(JWTAuthenticator::new(webtoken.clone()));
        try!(jwt.has_correct_issuer(LEGACYUSERACCOUNTISSUER));
        try!(jwt.has_correct_subject(USERACCOUNTNAMECLAIM));
        try!(jwt.has_secret_name_claim(SECRETNAMECLAIM));
        try!(jwt.has_account_uid_claim(USERACCOUNTUIDCLAIM));
        try!(jwt.has_correct_token_from_secret(datastore, SECRETUIDCLAIM));
        let mut session_tk: SessionCreate = SessionCreate::new();
        session_tk.set_email(email);
        session_tk.set_token(webtoken.clone());

        let _session = try!(session_create(datastore, session_tk));
        Ok(true)
    }
}

pub fn session_create(conn: &DataStoreConn, request: SessionCreate) -> Result<Session> {
    match SessionDS::find_account(&conn, &request) {
        Ok(session) => return Ok(session),
        Err(e) => {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Couldn not create session for the account."),
                error_description: format!("{}", e),
            }))
        }
    }
}
