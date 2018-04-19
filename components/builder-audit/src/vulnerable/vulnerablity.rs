// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server

use super::super::error::Result;

use std::io::Read;
use http_client::reqwest_client::http_basic_get;

use serde_json;
use protocol::api::imagevuln::Vulnerable;
use config;

/// Read the expression query language as per this link
//https://prometheus.io/docs/querying/basics/
#[derive(Clone)]
pub struct AnchoreClient {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[allow(unused_variables)]
impl AnchoreClient {
    pub fn new<T: config::Anchore>(config: &T) -> Self {
        AnchoreClient {
            url: config.endpoint().to_string(),
            username: config.username().to_string(),
            password: config.password().to_string(),
        }
    }

    pub fn check_vulnerablity(&self, name: &str) -> Result<Option<Vulnerable>> {
        let url = format!("{}/images/by_id/{}/vuln/os", self.url, name);
        let mut rep = http_basic_get(&url, self.username.clone(), self.password.clone())?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;

        let vulnercheck: Vulnerable = serde_json::from_str(&body)?;
        Ok(Some(vulnercheck))
    }
}
