use super::*;
use super::hooks::{consumption, instance, metric};
use super::hooks::before::{AHooks, HookServiceFn};
use super::super::error::{self, Result};
use chrono::prelude::*;
use itertools::Itertools;
use metrics::hooks::BeforeMetrics;
use metrics::prometheus::PrometheusClient;
use protocol::api::base::MetaFields;
use protocol::api::node;
use protocol::api::node::{Data, InstantVecItem, PromResponse};
use protocol::api::node::{MetricResponse, PrometheusQuery, QueryBuilder};
use serde_json;
use std::collections::BTreeMap;
use std::ops::Div;
use std::sync::Arc;

pub struct Executer {
    client: PrometheusClient,
}

impl Executer {
    pub fn new(client: PrometheusClient) -> Self {
        Executer { client: client }
    }

    pub fn pull_metrics(&self, querys: Vec<QueryBuilder>) -> Result<AHooks> {
        let res = self.client.pull_metrics(
            PrometheusQuery::with_querys(querys),
        )?;
        let data = self.group(res);
        Ok(self.before_hook(data))
    }

    fn group(&self, metrics: MetricResponse) -> BTreeMap<String, PromResponse> {
        let mut metgroups_map = BTreeMap::new();
        for metkey in metrics.data.iter() {
            metgroups_map.insert(metkey.clone().name, metkey.clone());
        }
        metgroups_map
    }

    fn before_hook(&self, content: BTreeMap<String, PromResponse>) -> AHooks {
        let mut ah = AHooks::new();
        let _content = content.clone();
        let cpu = Box::new(HookServiceFn::new(
            node::CAPACITY_CPU.to_string(),
            Box::new(move || -> Option<String> {
                consumption::Consumption::new(
                    _content
                        .get(node::CAPACITY_CPU)
                        .unwrap_or(&PromResponse::new())
                        .clone(),
                ).before()
            }),
        ));

        let _content = content.clone();
        let memory = Box::new(HookServiceFn::new(
            node::CAPACITY_MEMORY.to_string(),
            Box::new(move || -> Option<String> {
                consumption::Consumption::new(
                    _content
                        .get(node::CAPACITY_MEMORY)
                        .unwrap_or(&PromResponse::new())
                        .clone(),
                ).before()
            }),
        ));

        let _content = content.clone();
        let storage = Box::new(HookServiceFn::new(
            node::CAPACITY_STORAGE.to_string(),
            Box::new(move || -> Option<String> {
                consumption::Consumption::new(
                    _content
                        .get(node::CAPACITY_STORAGE)
                        .unwrap_or(&PromResponse::new())
                        .clone(),
                ).before()
            }),
        ));

        let _content = content.clone();
        let mut x = BTreeMap::new();
        for (key, value) in _content {
            if key.starts_with(node::NINJAS) {
                x.insert(key, value);
            }
        }
        let ninjas_intstance = Box::new(HookServiceFn::new(
            node::NINJAS.to_string(),
            Box::new(move || -> Option<String> {
                instance::Instance::new(node::NINJAS, x.clone()).before()
            }),
        ));

        let _content = content.clone();
        let mut x = BTreeMap::new();
        for (key, value) in _content {
            if key.starts_with(node::SENSEIS) {
                x.insert(key, value);
            }
        }
        let senseis_intstance = Box::new(HookServiceFn::new(
            node::SENSEIS.to_string(),
            Box::new(move || -> Option<String> {
                instance::Instance::new(node::SENSEIS, x.clone()).before()
            }),
        ));

        let _content = content.clone();
        let machine_cpu = Box::new(HookServiceFn::new(
            node::MACHINE_CAPACITY_CPU.to_string(),
            Box::new(move || -> Option<String> {
                metric::Metric::new(
                    _content
                        .get(node::MACHINE_CAPACITY_CPU)
                        .unwrap_or(&PromResponse::new())
                        .clone(),
                ).before()
            }),
        ));

        let _content = content.clone();
        let container_cpu = Box::new(HookServiceFn::new(
            node::CONTAINER_CAPACITY_CPU.to_string(),
            Box::new(move || -> Option<String> {
                metric::Metric::new(
                    _content
                        .get(node::CONTAINER_CAPACITY_CPU)
                        .unwrap_or(&PromResponse::new())
                        .clone(),
                ).before()
            }),
        ));

        let _content = content.clone();
        let container_memory = Box::new(HookServiceFn::new(
            node::CONTAINER_CAPACITY_MEMORY.to_string(),
            Box::new(move || -> Option<String> {
                metric::Metric::new(
                    _content
                        .get(node::CONTAINER_CAPACITY_MEMORY)
                        .unwrap_or(&PromResponse::new())
                        .clone(),
                ).before()
            }),
        ));

        let _content = content.clone();
        let container_storage = Box::new(HookServiceFn::new(
            node::CONTAINER_CAPACITY_STORAGE.to_string(),
            Box::new(move || -> Option<String> {
                metric::Metric::new(
                    _content
                        .get(node::CONTAINER_CAPACITY_STORAGE)
                        .unwrap_or(&PromResponse::new())
                        .clone(),
                ).before()
            }),
        ));

        ah.register(cpu);
        ah.register(memory);
        ah.register(storage);
        ah.register(ninjas_intstance);
        ah.register(senseis_intstance);
        ah.register(machine_cpu);
        ah.register(container_cpu);
        ah.register(container_memory);
        ah.register(container_storage);
        ah
    }
}
