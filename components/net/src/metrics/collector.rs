// Copyright (c) 2017 RioCorp Inc.

//! A module containing the health insight for the datacenter

use super::super::error::{Result};
use metrics::prometheus::PrometheusClient;

const GAUGE_SCOPES: &'static [&'static str] = &["cpu_total", "ram_total", "disk_total"];

/// const STATISTICS_SCOPES: &'static [&'static str] = &["cpu"];

#[derive(Clone)]
pub struct Collector<'a> {
    client: &'a PrometheusClient,
}

impl<'a> Collector<'a> {

    pub fn new(prom: &'a PrometheusClient) -> Self {
        Collector { client: &*prom }
    }

    /// Change the return result as per you need.
    pub fn gauges(&mut self) -> Result<Vec<String>> {
        let mut content_datas = vec![];

        for scope in GAUGE_SCOPES.iter() {
            let content = self.client.pull_gauge(scope, "");
            println!("-- scope {:?}\n{:?}\n", scope, content);
            if content.is_ok() {
                content_datas.push(content.unwrap().data);
            }
        }

        Ok(content_datas)
    }

    //This is bad, but use the same Collector and you need last_fetched data.
    pub fn statistics(&self, gauges: Vec<String>) -> Result<Vec<String>> {
        let content_datas = vec![];

        if gauges.len() > 0 {
            return Ok(gauges);
        }

        Ok(content_datas)
    }

    /*pub fn osusages(&self, path: &str) -> Result<()> {
        for scope in GAUGE_SCOPES.iter() {
            client.collect_gauge(scope, "");
            scopes.push(*scope);
        }

        Ok(scopes)
    }*/
}
