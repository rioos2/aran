// Copyright 2018 The Rio Advancement Inc

//! Configuration for a Rio/OS Streamer server

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct IdentityCfg {
    //  The identity handlers enabled
    //  example ["password", "token", "serviceaccount", "apikey", "otp"]
    //  currently not turned on.
    //pub enabled: Vec<String>,

    //  The public key location of service account
    pub service_account: Option<String>,
}

impl Default for IdentityCfg {
    fn default() -> Self {
        IdentityCfg { service_account: Some("service_account.pub".to_string()) }
    }
}

pub trait Identity {
    fn service_account(&self) -> Option<String>;
}
