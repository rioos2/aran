// Copyright 2018 The Rio Advancement Inc
use rioos;
use serviceaccount::service_account_ds::ServiceAccountDS;
use util::jwt_authenticator::JWTAuthenticator;
use super::super::error::{self, Result};
use db::data_store::DataStoreConn;
use std::path::PathBuf;
use protocol::api::base::IdGet;

const LEGACYUSERACCOUNTISSUER: &'static str = "rioos_sh/serviceaccount";
const SERVICEACCOUNTNAMECLAIM: &'static str = "rioos_sh/serviceaccount/service-account.name";
const SERVICEACCOUNTUIDCLAIM: &'static str = "rioos_sh/serviceaccount/service-account.uid";
const SECRETNAMECLAIM: &'static str = "rioos_sh/serviceaccount/secret.name";

#[derive(Clone, Debug)]
pub struct ServiceAccountAuthenticate {}

impl ServiceAccountAuthenticate {
    // it authenticates serviceaccount name and JWT token values
    // first it validates some static header and payload claims
    // then token is valid or not
    pub fn from_name_and_webtoken(datastore: &DataStoreConn, name: String, webtoken: String, key: PathBuf) -> error::Result<bool> {
        let jwt = try!(JWTAuthenticator::new(webtoken.clone()));
        try!(jwt.has_correct_issuer(LEGACYUSERACCOUNTISSUER));
        try!(jwt.has_correct_subject(SERVICEACCOUNTNAMECLAIM));
        try!(jwt.has_secret_name_claim(SECRETNAMECLAIM));
        try!(jwt.has_account_uid_claim(SERVICEACCOUNTUIDCLAIM));
        try!(jwt.has_correct_token_from_path(key));
        try!(get_service_account(datastore, name));
        Ok(true)
    }
}

fn get_service_account(conn: &DataStoreConn, name: String) -> Result<()> {
    match ServiceAccountDS::show(&conn, &IdGet::with_id(name.clone())) {
        Ok(_account) => Ok(()),
        Err(err) => {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Couldn't find {} in ServiceAccount", name.clone()),
                error_description: format!("{}", err),
            }))
        }
    }
}
