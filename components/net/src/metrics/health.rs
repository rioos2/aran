// Copyright (c) 2017 RioCorp Inc.

//! A module containing the health insight for the datacenter

use super::super::error::{self, Result};
use std::error::Error as StdError;
use std::result::Result as StdResult;

use serde_json;
use metrics::prometheus::PrometheusClient;

const GAUGE_SCOPES: &'static [&'static str] = &["cpu_total", "ram_total", "disk_total"];

const STATISTICS_SCOPES: &'static [&'static str] = &["cpu"];

#[derive(Clone)]
pub struct Health {
    client: PrometheusClient,
    /// This is just stubbed out for now to make it compile
    /// You need to stick the correct type for storing gauges
    last_fetched: Option<bool>,
}

impl Health {
    pub fn new(client: PrometheusClient) -> Self {
        Health { client: client, last_fetched: None }
    }

    /// Change the return result as per you need.
    pub fn gauges(&mut self) -> Result<()> {
        let mut scopes = vec![];

        for scope in GAUGE_SCOPES.iter() {
            self.client.pull_gauge(scope, "");
            scopes.push(*scope);
        }
        //You'll have to do your stuff in side the structure and save it inside
        // last_fetched. The last_fetched will be used by statistics.
        self.last_fetched = None;

        Ok(())
    }

    //This is bad, but use the same Health and you need last_fetched data.
    pub fn statistics(&self) -> Result<()> {
        //if self.last_fetched.is_err then return  ?

        //take the self.last_fetched data and do what you need.

        //You'll have to for your stuff in side the structure and returnt hat.
        Ok(())
    }

    /*pub fn osusages(&self, path: &str) -> Result<()> {
        for scope in GAUGE_SCOPES.iter() {
            client.collect_gauge(scope, "");
            scopes.push(*scope);
        }

        Ok(scopes)
    }*/
}
