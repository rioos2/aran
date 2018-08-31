// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server

use super::super::error::Result;

use http_client::reqwest_client::http_basic_get;
use std::io::Read;

use config;
use protocol::api::imagevuln::Vulnerable;
use serde_json;

/// Read the expression query language as per this link
//https://prometheus.io/docs/querying/basics/
#[derive(Clone)]
pub struct AnchoreClient {
    pub endpoint: String,
    pub username: String,
    pub password: String,
}

#[allow(unused_variables)]
impl AnchoreClient {
    pub fn new<T: config::Vulnerability>(config: &T) -> Self {
        AnchoreClient {
            endpoint: config.anchore_endpoint().to_string(),
            username: config.anchore_username().to_string(),
            password: config.anchore_password().to_string(),
        }
    }

    pub fn check_vulnerablity(&self, name: &str) -> Result<Option<Vulnerable>> {
        let url = format!("{}/images/by_id/{}/vuln/os", self.endpoint, name);
        let mut rep = http_basic_get(&url, self.username.clone(), self.password.clone())?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;

        let vulnercheck: Vulnerable = serde_json::from_str(&body)?;
        Ok(Some(vulnercheck))
    }
}
