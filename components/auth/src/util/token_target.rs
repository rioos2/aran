// Copyright 2018 The Rio Advancement Inc
//

use std::fmt;

use super::super::error::Result;
use protocol::api::session::SessionGet;
use base64;
use base64::decode_config as b64_dec;
use serde_json;
pub use error::Error;

pub trait TargetValidator: fmt::Display + Into<TokenTarget> {
    fn validate(&self) -> Result<()>;
}

#[derive(Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
pub struct TokenTarget {
    #[serde(default)]
    pub email: String,
    pub token: String,
    #[serde(default)]
    pub apikey: String,
    #[serde(default)]
    pub org_id: String,
    #[serde(default)]
    pub team_id: String,
    #[serde(default)]
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

    //parse riotoken 
    // and convert it to TokenTarget form
    pub fn parse(token: String) -> Self {
        let b64_to_json = |seg| -> Result<TokenTarget> {
            serde_json::from_slice(b64_dec(seg, base64::STANDARD)?.as_slice()).map_err(Error::from)
        };
        match b64_to_json(&token) {
            Ok(res) => {              
                res
            }
            Err(err) => {      
                debug!("« TokenTarget parse Error : {:?}", err); 
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
