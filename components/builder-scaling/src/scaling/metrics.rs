use error::Result;
use protocol::api::{node, scale};

use protocol::api::base::QueryInput;
use serde_json;
use std::collections::BTreeMap;
use telemetry::metrics::executer::Executer;
use telemetry::metrics::prometheus::PrometheusClient;
use telemetry::metrics::query::QueryMaker;


const CONTAINER_JOBS: &'static str = "job=rioos_sh_containers";

pub struct Client<'a> {
    prom: &'a PrometheusClient,
}
impl<'a> Client<'a> {
    pub fn new(prom: &'a PrometheusClient) -> Self {
        Client { prom: prom }
    }
    pub fn metrics(&self, af_id: &str, querypair: QueryInput) -> Result<Option<Vec<scale::ScalingGetResponse>>> {
        let metric_response = match &format!("job={}", querypair.get("job"))[..] {
            CONTAINER_JOBS => self.container_metric(&af_id),
            _ => self.assembly_metric(&af_id),
        };
        let mut response = scale::ScalingGetResponse::new();
        response.set_metrics(metric_response?);
        Ok(Some(vec![response]))
    }

    fn assembly_metric(&self, af_id: &str) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
        let querys = QueryMaker::new().snapshot_cpu_usage_in_machine(af_id, node::METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID);
        let res = Executer::new(self.prom.clone()).execute(querys)?;
        let mut data = BTreeMap::new();
        data.insert(
            node::CAPACITY_CPU.to_string(),
            serde_json::from_str(&res.get(node::MACHINE_CAPACITY_CPU).unwrap()).unwrap(),
        );
        Ok(data)
    }

    fn container_metric(&self, af_id: &str) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
        let querys = QueryMaker::new().snapshot_cpu_usage_in_contaner(af_id, node::METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID);
        let res = Executer::new(self.prom.clone()).execute(querys)?;
        let mut data = BTreeMap::new();

        data.insert(
            node::CAPACITY_CPU.to_string(),
            serde_json::from_str(&res.get(node::CONTAINER_CAPACITY_CPU).unwrap()).unwrap(),
        );
        data.insert(
            node::CAPACITY_MEMORY.to_string(),
            serde_json::from_str(&res.get(node::CONTAINER_CAPACITY_MEMORY).unwrap()).unwrap(),
        );
        data.insert(
            node::CAPACITY_STORAGE.to_string(),
            serde_json::from_str(&res.get(node::CONTAINER_CAPACITY_STORAGE).unwrap()).unwrap(),
        );
        Ok(data)
    }
}
