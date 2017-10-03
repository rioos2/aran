// Copyright (c) 2017 RioCorp Inc.

//! A module containing the health insight for the datacenter

use super::super::error::{self, Result};
use chrono::prelude::*;
use metrics::prometheus::PrometheusClient;
use serde_json;
use std::collections::BTreeMap;

const GAUGE_SCOPES: &'static [&'static str] = &["cpu_total", "ram_total", "disk_total"];

type Timestamp = f64;
type Value = String;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixItem {
    pub metric: BTreeMap<String, String>,
    pub values: Vec<Scalar>,
}
pub type Matrix = Vec<MatrixItem>;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantVecItem {
    pub metric: BTreeMap<String, String>,
    pub value: Scalar,
}
pub type InstantVec = Vec<InstantVecItem>;

pub type Scalar = (Timestamp, Value);

pub type Str = (Timestamp, String);


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "resultType", content = "result")]
#[serde(rename_all = "lowercase")]
pub enum Data {
    Matrix(Matrix),
    Vector(InstantVec),
    Scalar(Scalar),
    String(Str),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    pub fn metrics(&mut self) -> Result<(Vec<PromResponse>, Vec<PromResponse>)> {
        let mut content_datas = vec![];

        for scope in GAUGE_SCOPES.iter() {
            let content = self.client.pull_metrics(scope);
            if content.is_ok() {
                let response: PromResponse = serde_json::from_str(&content.unwrap().data).unwrap();
                content_datas.push(response);
            }
        }

        let gauges = self.set_gauges(Ok(content_datas.clone()));
        println!("------guages-----------------------------{:?}", guages);
        let statistics = self.set_statistics(Ok(content_datas.clone()));
        Ok((gauges.unwrap(), statistics.unwrap()))
    }

    fn set_gauges(&self, response: Result<Vec<PromResponse>>) -> Result<Vec<PromResponse>> {
        match response {
            Ok(proms) => {
                return Ok(
                    proms
                        .iter()
                        .map(|p| (*p.sum_group()).clone())
                        .collect::<Vec<_>>(),
                )
            }
            _ => return Err(error::Error::CryptoError(String::new())),
        }

    }

    fn set_statistics(&self, response: Result<Vec<PromResponse>>) -> Result<Vec<PromResponse>> {
        match response {
            Ok(proms) => return Ok(proms),
            _ => return Err(error::Error::CryptoError(String::new())),
        }
    }
}

pub trait SumGroup {
    fn sum_group(&self) -> &PromResponse;
}

impl SumGroup for PromResponse {
    fn sum_group(&self) -> &Self {

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
                acc.iter()
                    .map(|ref mut i| {
                        for (k, v) in &x.metric {
                            i.metric.clone().insert(k.to_string(), v.to_string());
                        }
                        i.value.clone().0 = x.value.0;
                        let b = x.value.1.trim().parse().unwrap_or(0);
                        let a = i.value.clone().1.trim().parse().unwrap_or(0);
                        i.value.clone().1 = (a + b).to_string()
                    })
                    .collect::<Vec<_>>();

                println!(" => iterated   is {:?}", acc);

                acc
            });

            println!("=> start sumgroup {:?}", sumvec);
        }
        println!("=> sumgroup done");
        //i don't know if this is the right way to do so.
        self
    }
}
