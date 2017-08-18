// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors

// Configuration for a Habitat SessionSrv service
/*
use hab_net::config::{ PasswordAuth};

use error::Error;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub github: PasswordCfg,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            github: PasswordCfg::default(),
        }
    }
}

impl PasswordAuth for Config {
    fn github_url(&self) -> &str {
        &self.github.url
    }

    fn github_client_id(&self) -> &str {
        &self.github.client_id
    }

    fn github_client_secret(&self) -> &str {
        &self.github.client_secret
    }
}
*/
