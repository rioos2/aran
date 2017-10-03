// Copyright (c) 2017 RioCorp Inc.

//! A module containing the health insight for the datacenter

use super::super::error::Result;
use chrono::prelude::*;
use metrics::prometheus::PrometheusClient;
use serde_json;
use std::collections::BTreeMap;

const GAUGE_SCOPES: &'static [&'static str] = &["cpu_total", "ram_total", "disk_total"];

type Timestamp = f64;
type Value = String;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Success,
    Error,
}


#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InvalidExpression(String),
    Timeout(String),
    InvalidResponse(serde_json::Error),
    Unexpected(u16),
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MatrixItem {
    pub metric: BTreeMap<String, String>,
    pub values: Vec<Scalar>,
}
pub type Matrix = Vec<MatrixItem>;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InstantVecItem {
    pub metric: BTreeMap<String, String>,
    pub value: Scalar,
}
pub type InstantVec = Vec<InstantVecItem>;

pub type Scalar = (Timestamp, Value);

pub type Str = (Timestamp, String);


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType", content = "result")]
#[serde(rename_all = "lowercase")]
pub enum Data {
    Matrix(Matrix),
    Vector(InstantVec),
    Scalar(Scalar),
    String(Str),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PromResponse {
    pub status: Status,
    pub data: Data,
    #[serde(rename = "errorType")]
    #[serde(default)]
    pub error_type: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}


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
    pub fn metrics(&mut self) -> Result<Vec<PromResponse>> {
        let mut content_datas = vec![];

        for scope in GAUGE_SCOPES.iter() {
            let content = self.client.pull_metrics(scope, "");
            println!("-- scope {:?}\n{:?}\n", scope, content);
            if content.is_ok() {
                let response: PromResponse = serde_json::from_str(&content.unwrap().data).unwrap();
                content_datas.push(response);
            }
        }

        for data in content_datas.iter() {
            data.sum_group();
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

pub trait SumGroup {
    fn sum_group(&self);
}

impl SumGroup for PromResponse {
    fn sum_group(&self) {
        use metrics::collector::Data;

        if let Data::Vector(ref instancevec) = (*self).data {
            println!("=> start sumgroup");
            let local: DateTime<UTC> = UTC::now();
            let initvec = vec![
                InstantVecItem {
                    metric: BTreeMap::new(),
                    value: (local.timestamp() as f64, "0".to_string()),
                },
            ];

            println!("=> start sumgroup {:?}", initvec);
            let sumvec = instancevec.iter().fold(initvec, |acc, ref mut x| {
                println!(" => accumultor is {:?}", acc);
                println!(" => x          is {:?}", x);
                acc.iter().map(|ref mut i| {
                    for (k, v) in &x.metric {
                        i.metric.clone().insert(k.to_string(), v.to_string());
                    }
                    i.value.clone().0 = x.value.0;
                    let b = x.value.1.trim().parse().unwrap_or(0);
                    let a = i.value.clone().1.trim().parse().unwrap_or(0);
                    i.value.clone().1 = (a + b).to_string()
                });

                println!(" => iterated   is {:?}", acc);

                acc
            });

            println!("=> start sumgroup {:?}", sumvec);
        }
        println!("=> sumgroup done");
    }
}
