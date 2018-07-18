// Copyright 2018 The Rio Advancement Inc

use super::super::error::{self, Result};
use base64;
use db::data_store::DataStoreConn;
use protocol::api::base::IdGet;
use rioos;
use secret::models::secret;
use serde_json;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str;
use util::jwt::{decode, decode_segments};
use util::jwt::{decode_direct, Algorithm};

const ISSUERCLAIM: &'static str = "iss";
const SECRETPUBKEY: &'static str = "rioos_sh/ssh_pubkey";
const SUBJECTCLAIM: &'static str = "sub";

// this struct collect stores decoded JWT token values (like header, payload..)
#[derive(Debug, Clone, PartialEq)]
pub struct JWTAuthenticator {
    token: String,
    header: BTreeMap<String, String>,
    payload: BTreeMap<String, String>,
}

impl JWTAuthenticator {
    // this function first decode the token
    // and get header & payload values then
    // generate JWTAuthenticator struct and return it
    pub fn new(token: String) -> Result<JWTAuthenticator> {
        let parsed_token_data = decode_segments(&token.clone());
        match parsed_token_data {
            Ok(payload_string) => {
                let payload_claims: BTreeMap<String, String> =
                    match serde_json::from_value(payload_string.1) {
                        Ok(v) => v,
                        Err(err) => {
                            return Err(error::Error::Auth(rioos::AuthErr {
                                error: format!("Couldn't parse JWT payload claims."),
                                error_description: format!("{}", err),
                            }))
                        }
                    };
                let header_claims: BTreeMap<String, String> =
                    match serde_json::from_value(payload_string.0) {
                        Ok(v) => v,
                        Err(err) => {
                            return Err(error::Error::Auth(rioos::AuthErr {
                                error: format!("Couldn't parse JWT header claims."),
                                error_description: format!("{}", err),
                            }))
                        }
                    };

                return Ok(JWTAuthenticator {
                    token: token,
                    header: header_claims,
                    payload: payload_claims,
                });
            }
            Err(err) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("Couldn't parse JWT header and payload claims."),
                    error_description: format!("{:?}", err),
                }))
            }
        }
    }

    // has_correct_issuer returns true if tokenData is a valid JWT in compact
    // serialization format and the "iss" claim matches the iss field of this token
    // authenticator, and otherwise returns false.
    //
    pub fn has_correct_issuer(&self, issuer: &str) -> Result<bool> {
        if self.payload.get(ISSUERCLAIM) != Some(&issuer.to_string()) {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!(
                    "Must have issuer claim {} = {}",
                    ISSUERCLAIM,
                    &issuer.to_string()
                ),
                error_description: format!(""),
            }));
        }
        return Ok(true);
    }

    // has_correct_subject returns true if valid subject and account_name claims
    // serialization format and the "sub" claim matches the sub field of this token
    // authenticator, and otherwise returns false.
    // split the subject claim and validate account_name claim and splitted subject claim if valid return true
    // otherwise returns false
    pub fn has_correct_subject(&self, claim: &str) -> Result<bool> {
        let subject = self.payload.get(SUBJECTCLAIM).unwrap();
        if subject.is_empty() {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Must have subject claim {}.", SUBJECTCLAIM),
                error_description: format!(""),
            }));
        }

        let account_name = self.payload.get(claim).unwrap();
        if account_name.is_empty() {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!(
                    "Must have an account name[email/service account name] in the claim "
                ),
                error_description: format!(""),
            }));
        }

        let parts: Vec<_> = subject.split("::").collect();
        if parts[1] != account_name {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!(
                    "Must have subject claim matching {} = {}",
                    parts[1].to_string(),
                    account_name.to_string()
                ),
                error_description: format!(""),
            }));
        }
        Ok(true)
    }

    // it is returns true if payload has valid secretname claim
    // otherwise it returns false
    pub fn has_secret_name_claim(&self, claim: &str) -> Result<bool> {
        let secret = self.payload.get(claim).unwrap();
        if secret.is_empty() {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Must have secret name in the claim."),
                error_description: format!(""),
            }));
        }
        Ok(true)
    }

    // it is returns true if payload has valid account uid claim
    // otherwise it returns false
    pub fn has_account_uid_claim(&self, claim: &str) -> Result<bool> {
        let account_uid = self.payload.get(claim).unwrap();
        if account_uid.is_empty() {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Must have account uid (account_id of the user) for the claim."),
                error_description: format!(""),
            }));
        }
        Ok(true)
    }

    // this function validate JWT token from stored secret public key into database
    // requester sends their secret uid into payload
    // first get secret public key from database using secret_uid claim
    // then decode JWT token using public key, it is valid then returns true
    // otherwise it returns false
    pub fn has_correct_token_from_secret(
        &self,
        datastore: &DataStoreConn,
        secret_claim: &str,
    ) -> Result<bool> {
        let secret_id = match self.parse_secret_uid(secret_claim) {
            Ok(sid) => sid,
            Err(e) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("Could not retrieve secret claim from payload."),
                    error_description: format!("{}", e),
                }))
            }
        };
        let params = IdGet::with_id(secret_id.to_string().clone());
        let secret = match secret::DataStore::show(datastore, &params) {
            Ok(s) => s,
            Err(e) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("Could not retrieve secret."),
                    error_description: format!("{}", e),
                }))
            }
        };
        let unwrap_secret = secret.unwrap();

        let public_key = unwrap_secret.get_data().get(SECRETPUBKEY).unwrap();

        if public_key.is_empty() {
            return Err(error::Error::Auth(rioos::AuthErr {
                error: format!("Could not get secret public key."),
                error_description: format!(""),
            }));
        }

        let bytes = base64::decode(&public_key.clone()).unwrap();

        let token_data = decode_direct(&self.token.clone(), bytes, Algorithm::RS256);
        match token_data {
            Ok(_t) => return Ok(true),
            Err(err) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("JWT bearer token is invalid."),
                    error_description: format!("{:?}", err),
                }));
            }
        }
    }

    // this function decode JWT token using public key url
    pub fn has_correct_token_from_path(&self, key_path: PathBuf) -> Result<bool> {
        let token_data = decode(&self.token.clone(), &key_path, Algorithm::RS256);

        match token_data {
            Ok(_t) => return Ok(true),
            Err(err) => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("JWT bearer token is invalid."),
                    error_description: format!("{:?}", err),
                }));
            }
        }
    }

    // it is returns secret uid if payload valid secret uid claim
    // otherwise it returns false
    pub fn parse_secret_uid(&self, secret_claim: &str) -> Result<String> {
        match self.payload.get(secret_claim) {
            Some(id) => return Ok(id.to_string()),
            None => {
                return Err(error::Error::Auth(rioos::AuthErr {
                    error: format!("Must have secret uid (secret_id) in the claim."),
                    error_description: format!(""),
                }));
            }
        }
    }
}
