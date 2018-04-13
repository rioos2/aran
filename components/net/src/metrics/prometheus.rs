// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server

use super::super::error::{self, Result};

use std::collections::HashMap;
use std::io::Read;
use reqwest::Url;
use reqwest::StatusCode;
use metrics::reqwest_client::http_bearer_get;
use serde_json;
use chrono::prelude::*;

use config;

/// Read the expression query language as per this link
//https://prometheus.io/docs/querying/basics/
#[derive(Clone)]
pub struct PrometheusClient {
    pub url: String,
}

impl PrometheusClient {
    pub fn new<T: config::Prometheus>(config: &T) -> Self {
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
    pub fn pull_metrics(&self, path: &str) -> Result<Contents> {
        let url = Url::parse(&format!("{}/query?query={}", self.url, path))?;
        let mut rep = http_bearer_get(url, path)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;

        if rep.status() != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(error::Error::PrometheusAPI(rep.status(), err));
        }

        let contents: Contents = Contents { data: body };

        Ok(contents)
    }

    /// Returns the contents of the node metrics
    ///http://localhost:9090/api/v1/query_range?query=up&start=2015-07-01T20:10:30.781Z&end=2015-07-01T20:11:00.781Z&step=15s'
    pub fn pull_osusage(&self, path: &str) -> Result<Contents> {
        let utc: DateTime<Utc> = Utc::now();
        let url = Url::parse(&format!(
            "{}/query_range?query={}&start={}&end={}&step=15s",
            self.url,
            path,
            utc.timestamp() - 180,
            utc.timestamp(),
        )).unwrap();

        let mut rep = http_bearer_get(url, path)?;
        let mut body = String::new();
        rep.read_to_string(&mut body)?;

        if rep.status() != StatusCode::Ok {
            let err: HashMap<String, String> = serde_json::from_str(&body)?;
            return Err(error::Error::PrometheusAPI(rep.status(), err));
        }

        let contents: Contents = Contents { data: body };

        Ok(contents)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Contents {
    pub data: String,
}
