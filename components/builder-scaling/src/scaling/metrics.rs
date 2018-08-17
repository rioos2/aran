use error::Result;
use protocol::api::{node, scale};

use protocol::api::base::QueryInput;
use std::collections::BTreeMap;
use telemetry::metrics::collector::Collector;
use telemetry::metrics::prometheus::PrometheusClient;
use telemetry::metrics::query_builder::QueryMaker;

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
        let mut mk_query = QueryMaker::new(self.prom);
        mk_query.set_assembly_cpu_query(af_id);
        let res = Collector::new(mk_query.pull_metrics()?).get_metrics(node::CAPACITY_CPU);
        let mut data = BTreeMap::new();
        data.insert("cpu".to_string(), res);
        Ok(data)
    }

    fn container_metric(&self, af_id: &str) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
        let mut mk_query = QueryMaker::new(self.prom);

        mk_query.set_container_query(af_id);

        let mut res = Collector::new(mk_query.pull_metrics()?);

        let mut data = BTreeMap::new();

        data.insert(
            node::CAPACITY_CPU.to_string(),
            res.get_metrics(node::CAPACITY_CPU),
        );
        data.insert(
            node::CAPACITY_MEMORY.to_string(),
            res.get_metrics(node::CAPACITY_MEMORY),
        );
        data.insert(
            node::CAPACITY_STORAGE.to_string(),
            res.get_metrics(node::CAPACITY_STORAGE),
        );
        Ok(data)
    }
}
