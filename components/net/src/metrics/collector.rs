// Copyright (c) 2017 RioCorp Inc.

//! A module containing the health insight for the datacenter

use super::super::error::{self, Result};
use chrono::prelude::*;
use metrics::prometheus::PrometheusClient;
use serde_json;
use std::collections::BTreeMap;
use protocol::nodesrv;

const CPU_TOTAL: &'static str = "cpu_total";
const GAUGE_SCOPES: &'static [&'static str] = &[CPU_TOTAL, "ram_total", "disk_total"];

#[derive(Clone)]
pub struct Collector<'a> {
    client: &'a PrometheusClient,
    scope: CollectorScope,
}

pub struct CollectorScope {
    scopes: Vec<String>,
    group: String,
}

impl<'a> Collector<'a> {
    pub fn new(prom: &'a PrometheusClient, scope: CollectorScope) -> Self {
        Collector {
            client: &*prom,
            scope: scope,
        }
    }

    pub fn metric_by(&mut self) -> Result<(Vec<nodesrv::PromResponse>, Vec<nodesrv::PromResponse>)> {
        let content_datas = do_collect();
        let statistics = self.set_statistics(Ok(content_datas.clone())); //make it os_usages
        Ok((gauges.unwrap(), statistics.unwrap()))
    }

    pub fn overall(&mut self) -> Result<(Vec<nodesrv::PromResponse>, Vec<nodesrv::PromResponse>)> {
        let content_datas = do_collect();
        let gauges = self.set_gauges(Ok(content_datas.clone()));
        let statistics = self.set_statistics(Ok(content_datas.clone()));
        Ok((gauges.unwrap(), statistics.unwrap()))
    }

    fn do_collect(&self) -> Vec {
        let mut content_datas = vec![];
        let label_group = format!("{group={}}", self.group);

        for scope in self.scopes.iter() {
            let content = self.client.pull_metrics(format!("{}{}", scope, self.group));
            if content.is_ok() {
                let response: nodesrv::PromResponse = serde_json::from_str(&content.unwrap().data).unwrap();
                content_datas.push(response);
            }
        }
        content_datas
    }

    fn set_gauges(&self, response: Result<Vec<nodesrv::PromResponse>>) -> Result<Vec<nodesrv::PromResponse>> {
        match response {
            Ok(proms) => {
                return Ok(
                    proms
                        .into_iter()
                        .map(|mut p| (p.sum_group().clone()))
                        .collect::<Vec<_>>(),
                )
            }
            _ => return Err(error::Error::CryptoError(String::new())),
        }
    }

    fn set_statistics(&self, response: Result<Vec<nodesrv::PromResponse>>) -> Result<Vec<nodesrv::PromResponse>> {
        match response {
            Ok(proms) => {
                return Ok(
                    proms
                        .into_iter()
                        .filter(|x| {
                            match (*x).data {
                                nodesrv::Data::Vector(ref ins) => {
                                    return (*ins)
                                        .clone()
                                        .into_iter()
                                        .find(|m| {
                                            m.metric.get("__name__").unwrap_or(&"nop".to_string()) == CPU_TOTAL
                                        })
                                        .is_some()
                                }
                                _ => return false,
                            };

                        })
                        .collect::<Vec<_>>()
                        .to_vec(),
                )
            }
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
        let mut sum = Data::Vector(vec![]);
        if let nodesrv::Data::Vector(ref mut instancevec) = (*self).data {
            let local: DateTime<UTC> = UTC::now();
            let initvec = vec![
                nodesrv::InstantVecItem {
                    metric: BTreeMap::new(),
                    value: (local.timestamp() as f64, "0".to_string()),
                },
            ];
            let instance_changed = instancevec.iter_mut().fold(initvec, |mut acc, ref mut x| {

                acc.iter_mut()
                    .map(|ref mut i| {
                        for (k, v) in &x.metric {
                            i.metric.insert(k.to_string(), v.to_string());
                        }
                        i.value.0 = x.value.clone().0;
                        let b = x.value.1.trim().parse::<f64>().unwrap_or(1.0);
                        let a = i.value.1.trim().parse::<f64>().unwrap_or(1.0);
                        i.value.1 = (a + b).to_string();
                    })
                    .collect::<Vec<_>>();
                acc
            });
            sum = nodesrv::Data::Vector(instance_changed.to_vec());
        }
        self.data = sum;
        (*self).clone()
    }
}
