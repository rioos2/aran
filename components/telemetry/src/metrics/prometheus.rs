// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server

use super::super::error::Result;

use chrono::prelude::*;
use config;
use http_client::reqwest_client::{http_bearer_post, http_bearer_get};
use metrics::query::PrometheusQuery;
use protocol::api::node::{MetricResponse, PromResponse};

use serde_json;

use std::io::Read;

/// Read the expression query language as per this link
//https://prometheus.io/docs/querying/basics/
#[derive(Clone)]
pub struct PrometheusClient {
    pub url: String,
}

impl PrometheusClient {
    pub fn new<T: config::Telemetry>(config: &T) -> Self {
        PrometheusClient { url: config.endpoint().to_string() }
    }

    /// Returns the instant vector metric for all nodes
    /// https://prometheus.io/docs/querying/basics/
    //  Here is a query
    ///   https://<prometheus_url>?query/query=cpu_total{job="prometheus",group="nodes"}
    /// The above is actually <metric name>{<label name>=<label value>, ...}
    /// where
    ///       metric_name = cpu_total
    ///       label_name  = job (first label)
    ///       label_value = prometheus (first labels value)
    ///       label_name  = group (first label)
    ///       label_value = nodes (first labels value)
    pub fn pull_metrics(&self, body: PrometheusQuery) -> Result<MetricResponse> {
        let url = format!("{}/querys", self.url);
        let mut res = http_bearer_post(&url, serde_json::to_value(&body)?)?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;
        let data: MetricResponse = serde_json::from_str(&body)?;
        Ok(data)
    }

    /// Returns the contents of the node metrics
    ///http://localhost:9090/api/v1/query_range?query=up&start=2015-07-01T20:10:30.781Z&end=2015-07-01T20:11:00.781Z&step=15s'
    pub fn pull_osusage(&self, path: &str) -> Result<PromResponse> {
        let utc: DateTime<Utc> = Utc::now();
        let url =
            format!(
            "{}/query_range?query={}&start={}&end={}&step=15s",
            self.url,
            path,
            utc.timestamp() - 180,
            utc.timestamp(),
        );

        let mut rep = http_bearer_get(&url, path)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;

        let contents: PromResponse = serde_json::from_str(&body)?;

        Ok(contents)
    }
}
