// Copyright 2018 The Rio Advancement Inc
//

use super::super::error::Result;
use protocol::api::session::SessionGet;
use std::fmt;

pub trait TargetValidator: fmt::Display + Into<TokenTarget> {
    fn validate(&self) -> Result<()>;
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct TokenTarget {
    pub email: String,
    pub token: String,
    pub apikey: String,
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
