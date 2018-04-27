// Copyright 2018 The Rio Advancement Inc

use std::collections::HashMap;

pub const PLUGIN_PASSWORD: &'static str = "password";
pub const PLUGIN_SERVICE_ACCOUNT: &'static str = "service_account";
pub const PLUGIN_PASSTICKET: &'static str = "passticket";
pub const PLUGIN_JWT: &'static str = "jwt";

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct IdentityCfg {
    //  The identity handlers enabled
    //  example ["password", "token", "serviceaccount", "apikey", "otp"]
    //  currently not turned on.
    pub enabled: Vec<String>,

    //  A key value params hash as needed in AuthenticationFlow
    pub params: HashMap<String, String>,
}

impl Default for IdentityCfg {
    fn default() -> Self {
        IdentityCfg {
            enabled: vec!["password".to_string(), "token".to_string()],
            params: HashMap::new(),
        }
    }
}

pub trait Identity {
    fn enabled(&self) -> Vec<String>;

    fn params(&self) -> HashMap<String, String>;
}
