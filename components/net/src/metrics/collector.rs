// Copyright (c) 2017 RioCorp Inc.

//! A module containing the health insight for the datacenter

use super::super::error::{self, Result};
use chrono::prelude::*;
use metrics::prometheus::PrometheusClient;
use serde_json;
use std::collections::{BTreeMap, HashMap};
use protocol::nodesrv;
use itertools::Itertools;


pub const CPU_TOTAL: &'static str = "cpu_total";
// const GAUGE_SCOPES: &'static [&'static str] = &[CPU_TOTAL, "ram_total", "disk_total"];

#[derive(Clone)]
pub struct Collector<'a> {
    client: &'a PrometheusClient,
    scope: CollectorScope,
}

#[derive(Clone)]
pub struct CollectorScope {
    pub metric_names: Vec<String>,
    pub labels: Vec<String>,
    pub last_x_minutes: Option<String>,
}

impl<'a> Collector<'a> {
    pub fn new(prom: &'a PrometheusClient, scope: CollectorScope) -> Self {
        Collector {
            client: &*prom,
            scope: scope,
        }
    }

    pub fn metric_by(&mut self) -> Result<Vec<nodesrv::PromResponse>> {
        let content_datas = self.do_collect();
        let metrics = self.set_metrics(Ok(content_datas.clone())); //make it os_usages
        Ok(metrics.unwrap())
    }

    pub fn overall(&mut self) -> Result<(Vec<nodesrv::PromResponse>, Vec<nodesrv::PromResponse>)> {
        let content_datas = self.do_collect();
        let gauges = self.set_gauges(Ok(content_datas.clone()));
        let statistics = self.set_statistics(Ok(content_datas.clone()));
        Ok((gauges.unwrap(), statistics.unwrap()))
    }

    fn do_collect(&self) -> Vec<nodesrv::PromResponse> {
        let mut content_datas = vec![];
        let s: String = self.scope.labels.clone().into_iter().collect();
        let l = self.scope.last_x_minutes.clone().unwrap_or("".to_string());
        let label_group = format!("{}{}{}{}", "{", s, "}", l);
        for scope in self.scope.metric_names.iter() {
            let content = self.client.pull_metrics(
                &format!("{}{}", scope, label_group),
            );
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

    fn set_metrics(&self, response: Result<Vec<nodesrv::PromResponse>>) -> Result<Vec<nodesrv::PromResponse>> {
        match response {
            Ok(proms) => {
                return Ok(
                    proms
                        .into_iter()
                        .map(|mut x| (x.os_usage().clone()))
                        .collect::<Vec<_>>(),
                );
            }
            _ => return Err(error::Error::CryptoError(String::new())),
        }
    }
}

pub trait SumGroup {
    fn sum_group(&mut self) -> nodesrv::PromResponse;
    fn os_usage(&mut self) -> nodesrv::PromResponse;
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

    fn os_usage(&mut self) -> Self {
        use self::nodesrv::*;
        let mut usage = Data::Matrix(vec![]);
        let mut metgroups_map = HashMap::<String, String>::new();
        if let nodesrv::Data::Matrix(ref mut instancevec) = (*self).data {
            let local: DateTime<UTC> = UTC::now();
            let initvec = vec![
                nodesrv::MatrixItem {
                    metric: BTreeMap::new(),
                    values: vec![(local.timestamp() as f64, "0".to_string())],
                },
            ];
            let fms = instancevec
                .iter()
                .flat_map(|s| s.values.clone())
                .collect::<Vec<_>>();

            let instance_changed = instancevec.iter_mut().fold(initvec, |mut acc, ref mut x| {
                acc.iter_mut()
                    .map(|ref mut i| {
                        for (k, v) in &x.metric {
                            i.metric.insert(k.to_string(), v.to_string());
                        }
                        for (metkey, metvalues_group) in &fms.iter().group_by(|fm| &fm.0) {
                            let aggregate: f64 = metvalues_group
                                .map(|x| x.1.trim().parse::<f64>().unwrap_or(1.0))
                                .sum();
                            metgroups_map.entry(metkey.to_string()).or_insert_with(|| {
                                aggregate.to_string()
                            });
                        }
                        let mut data = Vec::<(f64, String)>::new();
                        for (k, v) in metgroups_map.iter() {
                            data.push((k.trim().parse::<f64>().unwrap(), v.to_string()));
                        }
                        i.values = data;
                    })
                    .collect::<Vec<_>>();
                acc
            });
            usage = nodesrv::Data::Matrix(instance_changed.to_vec());
        }
        self.data = usage;
        (*self).clone()
    }
}
