// Copyright 2018 The Rio Advancement Inc

use rioos;
use serviceaccount::service_account_ds::ServiceAccountDS;
use util::jwt_authenticator::JWTAuthenticator;
use super::super::error::{self, Result};
use protocol::api::session::*;
use db::data_store::DataStoreConn;

const LEGACYUSERACCOUNTISSUER: &'static str = "rioos_sh/serviceaccount";
const SERVICEACCOUNTNAMECLAIM: &'static str = "rioos_sh/serviceaccount/service-account.name";
const SERVICEACCOUNTUIDCLAIM: &'static str = "rioos_sh/serviceaccount/service-account.uid";
//const SECRETUIDCLAIM: &'static str = "rioos_sh/serviceaccount/secret.uid";
const SECRETNAMECLAIM: &'static str = "rioos_sh/serviceaccount/secret.name";

#[derive(Clone, Debug)]
pub struct ServiceAccountAuthenticate {}

impl ServiceAccountAuthenticate {   

	// it authenticates serviceaccount name and JWT token values
    // first it validates some static header and payload claims
    // then token is valid or not 
    pub fn from_name_and_webtoken(datastore: &DataStoreConn, name: String, webtoken: String, key: String) -> error::Result<bool> {
    	let jwt = try!(JWTAuthenticator::new(webtoken.clone()));
        try!(jwt.has_correct_issuer(LEGACYUSERACCOUNTISSUER));
        try!(jwt.has_correct_subject(SERVICEACCOUNTNAMECLAIM));
        try!(jwt.has_secret_name_claim(SECRETNAMECLAIM));
        try!(jwt.has_account_uid_claim(SERVICEACCOUNTUIDCLAIM));
        try!(jwt.has_correct_token_from_path(key));
        let mut session_tk: SessionCreate = SessionCreate::new();
                session_tk.set_email(name);
                session_tk.set_token(webtoken.clone());

        let _session = try!(session_create(datastore, session_tk));
    	Ok(true)
    }

}

pub fn session_create(conn: &DataStoreConn, request: SessionCreate) -> Result<Session> {
    match ServiceAccountDS::find_service_account(&conn, &request) {
        Ok(session) => return Ok(session),
        Err(e) => {
        	return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Couldn not create session for the service account."),
                error_description: format!("{}", e),
            }))
        }        	
    }
}