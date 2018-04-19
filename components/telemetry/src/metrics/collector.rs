// Copyright 2018 The Rio Advancement Inc

//! A module containing the health insight for the datacenter
use std::ops::Div;
use std::collections::BTreeMap;

use chrono::prelude::*;
use metrics::prometheus::PrometheusClient;

use serde_json;

use protocol::api::node::{Data, PromResponse, InstantVecItem};
use super::expression::*;
use super::super::error::{self, Result};

pub const CPU_TOTAL: &'static str = "cpu_total";

#[derive(Clone)]
pub struct Collector<'a> {
    client: &'a PrometheusClient,
    scope: CollectorScope,
}

#[derive(Clone)]
pub struct CollectorScope {
    pub metric_names: Vec<String>,
    pub labels: Vec<String>,
    pub last_x_minutes: String,
    pub avg_by_name: String,
}

impl<'a> Collector<'a> {
    pub fn new(prom: &'a PrometheusClient, scope: CollectorScope) -> Self {
        Collector {
            client: &*prom,
            scope: scope,
        }
    }

    //collect the overall memory of all nodes
    pub fn average_memory(&self) -> Result<Vec<PromResponse>> {
        let mut content_datas = vec![];

        let avg = Functions::Sum(AvgInfo {
            operator: Operators::Sum(SumInfo {
                labels: self.scope.labels.clone(),
                metric: self.scope.metric_names.clone(),
                total: "sum".to_string(),
            }),
        });
        let data = format!(
            "{}",
            MetricQueryBuilder::new(MetricQuery {
                functions: avg,
                by: "".to_string(),
            })
        );

        let content = self.client.pull_metrics(&data)?;

        let response: PromResponse = serde_json::from_str(&content.data).unwrap();
        content_datas.push(response);

        let memory_contents_data = self.set_metric_name(Ok(content_datas), "ram_total")?;

        Ok(memory_contents_data)
    }

    //collect the overall disk of all nodes
    pub fn average_disk(&self) -> Result<Vec<PromResponse>> {
        let mut content_datas = vec![];

        let avg = Functions::SumDisk(AvgInfo {
            operator: Operators::SumDisk(SumInfo {
                labels: self.scope.labels.clone(),
                metric: self.scope.metric_names.clone(),
                total: "sum".to_string(),
            }),
        });
        let data = format!(
            "{}",
            MetricQueryBuilder::new(MetricQuery {
                functions: avg,
                by: "".to_string(),
            })
        );

        let content = self.client.pull_metrics(&data)?;

        let response: PromResponse = serde_json::from_str(&content.data).unwrap();
        content_datas.push(response);

        let memory_contents_data = self.set_metric_name(Ok(content_datas), "disk_total")?;

        Ok(memory_contents_data)
    }

    //get the total usage of all node cpu metric and each node cpu metric
    pub fn overall_node_cpu(&mut self) -> Result<(Vec<PromResponse>, Vec<PromResponse>)> {
        let content_datas = self.avg_collect()?;
        let node_contents_data = self.set_metric_name(Ok(content_datas), "cpu_total")?;

        let gauges = self.set_gauges(Ok(node_contents_data.clone()));
        let statistics = self.set_statistics(Ok(node_contents_data.clone()));
        Ok((gauges.unwrap(), statistics.unwrap()))
    }

    pub fn network_metric(&self) -> Result<Vec<PromResponse>> {
        let mut content_datas = vec![];

        let network_query = Functions::Network(AvgInfo {
            operator: Operators::Network(SumInfo {
                labels: self.scope.labels.clone(),
                metric: self.scope.metric_names.clone(),
                total: self.scope.last_x_minutes.clone(),
            }),
        });

        let content = self.client.pull_metrics(&format!("{}", network_query))?;

        let response: PromResponse = serde_json::from_str(&content.data).unwrap();

        content_datas.push(response);

        Ok(content_datas)
    }

    //os usage metric of the assemblys
    pub fn metric_by_os_usage(&mut self) -> Result<(Vec<PromResponse>, Vec<PromResponse>)> {
        let content_datas = self.avg_collect()?;
        let node_contents_data = self.set_metric_name(Ok(content_datas), "cpu_total")?;
        let gauges = self.set_gauges(Ok(node_contents_data.clone()));

        let content = self.os_avg_collect()?;

        Ok((gauges.unwrap(), content))
    }



    //metric for general
    pub fn metric_by_avg(&mut self) -> Result<BTreeMap<String, String>> {
        let content_datas = self.avg_collect()?;

        let metrics = self.set_metrics_average(Ok(content_datas.clone()));
        Ok(metrics.unwrap())
    }
    //metrics for container
    pub fn metric_by_avg_for_containers(&mut self) -> Result<BTreeMap<String, String>> {
        let content_datas = self.avg_collect_for_containers()?;

        let metrics = self.set_metrics_average(Ok(content_datas.clone()));
        Ok(metrics.unwrap())
    }


    fn set_metric_name(&self, response: Result<Vec<PromResponse>>, name: &str) -> Result<Vec<PromResponse>> {
        match response {
            Ok(proms) => {
                return Ok(
                    proms
                        .into_iter()
                        .map(|mut p| {
                            if let Data::Vector(ref mut instancevec) = p.data {
                                instancevec
                                    .iter_mut()
                                    .map(|x| {
                                        x.metric.insert("__name__".to_string(), name.to_string());
                                    })
                                    .collect::<Vec<_>>();
                            }
                            p
                        })
                        .collect::<Vec<_>>(),
                );
            }
            _ => return Err(error::Error::CryptoError(String::new())),
        }
    }

    // collect the average data for the cpu usage from prometheus
    fn avg_collect(&self) -> Result<Vec<PromResponse>> {
        let mut content_datas = vec![];
        for scope in self.scope.metric_names.iter() {
            let avg = Functions::Avg(AvgInfo {
                operator: Operators::IRate(IRateInfo {
                    labels: self.scope.labels.clone(),
                    metric: scope.to_string(),
                    last_x_minutes: self.scope.last_x_minutes.clone(),
                }),
            });
            let data = format!(
                "100 - ({} * 100)",
                MetricQueryBuilder::new(MetricQuery {
                    functions: avg,
                    by: format!("avg by ({})", self.scope.avg_by_name.clone()),
                })
            );

            let content = self.client.pull_metrics(&data)?;


            let response: PromResponse = serde_json::from_str(&content.data).unwrap();
            content_datas.push(response);

        }

        Ok(content_datas)
    }
    // collect the average data for the cpu usage from prometheus
    fn avg_collect_for_containers(&self) -> Result<Vec<PromResponse>> {
        let mut content_datas = vec![];
        for scope in self.scope.metric_names.iter() {
            let sum = Functions::Sum(AvgInfo {
                operator: Operators::IRate(IRateInfo {
                    labels: self.scope.labels.clone(),
                    metric: scope.to_string(),
                    last_x_minutes: self.scope.last_x_minutes.clone(),
                }),
            });
            let content = self.client.pull_metrics(&format!(
                "sum by({}) ({})*100",
                self.scope.avg_by_name.clone(),
                sum
            ))?;


            let response: PromResponse = serde_json::from_str(&content.data).unwrap();
            content_datas.push(response);

        }

        Ok(content_datas)
    }

    // collect the average data for the cpu usage from prometheus
    fn os_avg_collect(&self) -> Result<Vec<PromResponse>> {
        let mut content_datas = vec![];
        for scope in self.scope.metric_names.iter() {
            let avg = Functions::Avg(AvgInfo {
                operator: Operators::IRate(IRateInfo {
                    labels: self.scope.labels.clone(),
                    metric: scope.to_string(),
                    last_x_minutes: self.scope.last_x_minutes.clone(),
                }),
            });
            let data = format!(
                "100 - ({} * 100)",
                MetricQueryBuilder::new(MetricQuery {
                    functions: avg,
                    by: format!("avg by ({})", self.scope.avg_by_name.clone()),
                })
            );

            let content = self.client.pull_osusage(&data)?;

            let response: PromResponse = serde_json::from_str(&content.data).unwrap();
            content_datas.push(response);

        }

        Ok(content_datas)
    }
    fn set_gauges(&self, response: Result<Vec<PromResponse>>) -> Result<Vec<PromResponse>> {
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

    fn set_statistics(&self, response: Result<Vec<PromResponse>>) -> Result<Vec<PromResponse>> {
        match response {
            Ok(proms) => {
                return Ok(
                    proms
                        .into_iter()
                        .filter(|x| {
                            match (*x).data {
                                Data::Vector(ref ins) => {
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

    fn set_metrics_average(&self, response: Result<Vec<PromResponse>>) -> Result<BTreeMap<String, String>> {
        match response {
            Ok(proms) => {
                let mut data = BTreeMap::new();
                proms
                    .into_iter()
                    .map(|mut x| if let Data::Vector(ref mut instancevec) = x.data {
                        instancevec
                            .iter_mut()
                            .map(|x| for (_k, v) in &x.metric {
                                data.insert(v.to_string(), x.value.1.clone());
                            })
                            .collect::<Vec<_>>();
                    })
                    .collect::<Vec<_>>();
                Ok(data)
            }
            _ => return Err(error::Error::CryptoError(String::new())),
        }
    }
}

pub trait SumGroup {
    fn sum_group(&mut self) -> PromResponse;
}
impl SumGroup for PromResponse {
    fn sum_group(&mut self) -> Self {
        let mut sum = Data::Vector(vec![]);
        if let Data::Vector(ref mut instancevec) = (*self).data {
            let local: DateTime<Utc> = Utc::now();
            let initvec = vec![
                InstantVecItem {
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

            sum = Data::Vector(
                instance_changed
                    .iter()
                    .map(|x| if x.value.1 != "0" {
                        let avg_metric_val = (
                            x.value.0,
                            (x.value.1.trim().parse::<f64>().unwrap_or(1.0).div(
                                (*instancevec).len() as
                                    f64,
                            )).to_string(),
                        );

                        InstantVecItem {
                            metric: x.metric.clone(),
                            value: avg_metric_val,
                        }
                    } else {
                        InstantVecItem {
                            metric: x.metric.clone(),
                            value: (x.value.0, "".to_string()),
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        }
        self.data = sum;
        (*self).clone()
    }
}