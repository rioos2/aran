#![allow(non_snake_case)]

use std::collections::BTreeMap;

use error::Result;
use protocol::api::scale;

use protocol::api::base::QueryInput;
use serde_json;
use telemetry::metrics;
use telemetry::metrics::executer::Executer;
use telemetry::metrics::prometheus::PrometheusClient;
use telemetry::metrics::query::QueryMaker;

pub struct Client<'a> {
    prom: &'a PrometheusClient,
}
impl<'a> Client<'a> {
    pub fn new(prom: &'a PrometheusClient) -> Self {
        Client { prom: prom }
    }
    pub fn metrics(&self, af_id: &str, querypair: QueryInput) -> Result<Option<Vec<scale::ScalingGetResponse>>> {
        let metric_response = match &format!("job={}", querypair.get("job"))[..] {
            _CONTAINER_JOBS => self.container_metric(&af_id),
            _ => self.assembly_metric(&af_id),
        };
        let mut response = scale::ScalingGetResponse::new();
        response.set_metrics(metric_response?);
        Ok(Some(vec![response]))
    }

    fn assembly_metric(&self, af_id: &str) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
        let querys = QueryMaker::new().snapshot_cpu_usage_in_machine(af_id, metrics::METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID);
        let res = Executer::new(self.prom.clone()).execute(querys)?;
        let mut data = BTreeMap::new();
        data.insert(
            metrics::CAPACITY_CPU.to_string(),
            serde_json::from_str(&res.get(metrics::MACHINE_CAPACITY_CPU).unwrap()).unwrap(),
        );
        Ok(data)
    }

    fn container_metric(&self, af_id: &str) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
        let querys = QueryMaker::new().snapshot_cpu_usage_in_contaner(af_id, metrics::METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID);
        let res = Executer::new(self.prom.clone()).execute(querys)?;
        let mut data = BTreeMap::new();

        data.insert(
            metrics::CAPACITY_CPU.to_string(),
            serde_json::from_str(&res.get(metrics::CONTAINER_CAPACITY_CPU).unwrap()).unwrap(),
        );
        data.insert(
            metrics::CAPACITY_MEMORY.to_string(),
            serde_json::from_str(&res.get(metrics::CONTAINER_CAPACITY_MEMORY).unwrap()).unwrap(),
        );
        data.insert(
            metrics::CAPACITY_STORAGE.to_string(),
            serde_json::from_str(&res.get(metrics::CONTAINER_CAPACITY_STORAGE).unwrap()).unwrap(),
        );
        Ok(data)
    }
}
