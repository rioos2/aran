// Copyright (c) 2017 RioCorp Inc.

//! A module containing the health insight for the datacenter

use super::super::error::{self, Result};
use chrono::prelude::*;
use metrics::prometheus::PrometheusClient;
use serde_json;
use std::collections::BTreeMap;
use protocol::nodesrv;


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

    pub fn metrics(&mut self) -> Result<(Vec<nodesrv::PromResponse>, Vec<nodesrv::PromResponse>)> {
        let mut content_datas = vec![];

        for scope in GAUGE_SCOPES.iter() {
            let content = self.client.pull_metrics(scope);
            if content.is_ok() {
                let response: nodesrv::PromResponse = serde_json::from_str(&content.unwrap().data).unwrap();
                content_datas.push(response);
            }
        }
        println!(
            "-----------------------content_datas------------------------------------------{:?}",
            content_datas
        );
        let gauges = self.set_gauges(Ok(content_datas.clone()));
        println!("--------gauges-----------------------------{:?}", gauges);
        let statistics = self.set_statistics(Ok(content_datas.clone()));
        Ok((gauges.unwrap(), statistics.unwrap()))
    }

    fn set_gauges(&self, response: Result<Vec<nodesrv::PromResponse>>) -> Result<Vec<nodesrv::PromResponse>> {
        match response {
            Ok(mut proms) => {
                return Ok(
                    proms
                        .iter_mut()
                        .map(|mut p| (p.sum_group().clone()))
                        .collect::<Vec<_>>(),
                )
            }
            _ => return Err(error::Error::CryptoError(String::new())),
        }
    }

    fn set_statistics(&self, response: Result<Vec<nodesrv::PromResponse>>) -> Result<Vec<nodesrv::PromResponse>> {
        match response {
            Ok(proms) => return Ok(proms),
            _ => return Err(error::Error::CryptoError(String::new())),
        }
    }
}

pub trait SumGroup {
    fn sum_group(&mut self) -> nodesrv::PromResponse;
}

impl SumGroup for nodesrv::PromResponse {
    fn sum_group(&mut self) -> Self {

        use self::nodesrv::Data;
        if let Data::Vector(ref mut instantvec) = (*self).data {
            let local: DateTime<UTC> = UTC::now();
            let initvec = vec![
                nodesrv::InstantVecItem {
                    metric: BTreeMap::new(),
                    value: (local.timestamp() as f64, "0".to_string()),
                },
            ];

            instantvec.iter_mut().fold(initvec, |mut acc, ref mut x| {
                acc.iter_mut()
                    .map(|ref mut i| {
                        for (k, v) in &x.metric {
                            i.metric.insert(k.to_string(), v.to_string());
                        }
                        i.value.0 = x.value.clone().0;
                        let b = x.value.1.trim().parse::<f64>().unwrap_or(1.0);
                        let a = i.value.clone().1.trim().parse::<f64>().unwrap_or(1.0);
                        i.value.clone().1 = (a + b).to_string()
                    })
                    .collect::<Vec<_>>();
                acc
            });

        }

        (*self).clone()
    }
}
