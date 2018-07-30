// Copyright 2018 The Rio Advancement Inc
//

use super::super::error::Result;
use protocol::api::session::SessionGet;
use base64;
use base64::decode_config as b64_dec;
use serde_json::Value as JsonValue;
use std::fmt;
use serde_json;
pub use error::Error;

pub trait TargetValidator: fmt::Display + Into<TokenTarget> {
    fn validate(&self) -> Result<()>;
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct TokenTarget {
    pub email: String,
    pub token: String,
    pub apikey: String,
    pub org_id: String,
    pub team_id: String,
    pub account_id: String,
}

impl TokenTarget {
    /// Creates a new token target
    ///
    /// Errors:
    /// * InvalidEmail
    /// * InvalidApikey
    pub fn new(email: String, token: String) -> Self {
        TokenTarget {
            email: email,
            token: token,
            apikey: Default::default(),
            org_id: Default::default(),
            team_id: Default::default(),
            account_id: Default::default(),
        }
    }

    pub fn new_with_values(email: String, token: String, apikey: String, org_id: String, team_id: String, account_id: String) -> Self {
        TokenTarget {
            email: email,
            token: token,
            apikey: apikey,
            org_id: org_id,
            team_id: team_id,
            account_id: account_id,
        }
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.apikey.clone()
    }

    pub fn get_org_id(&self) -> ::std::string::String {
        self.org_id.clone()
    }

    pub fn get_team_id(&self) -> ::std::string::String {
        self.team_id.clone()
    }

    pub fn get_account_id(&self) -> ::std::string::String {
        self.account_id.clone()
    }

    pub fn parse(token: String) -> Self {
        let b64_to_json = |seg| -> Result<JsonValue> {
            serde_json::from_slice(b64_dec(seg, base64::STANDARD)?.as_slice()).map_err(Error::from)
        };
        println!("{}", token);
        match b64_to_json(&token) {
            Ok(res) => {
                TokenTarget::new_with_values(
                    res["email"].to_string(), 
                    res["api_token"].to_string(), 
                    "".to_string(), 
                    res["org_id"].to_string(), 
                    res["team_id"].to_string(), 
                    res["account_id"].to_string()
                )
            }
            Err(err) => {               
                TokenTarget::new_with_values("".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string())
            }
        }
    }

}

impl fmt::Display for TokenTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.email, self.token, self.apikey)
    }
}

impl Into<SessionGet> for TokenTarget {
    fn into(self) -> SessionGet {
        let mut session_get = SessionGet::new();
        session_get.set_email(self.get_email().to_owned());
        session_get.set_token(self.get_token().to_owned());
        session_get
    }
}
