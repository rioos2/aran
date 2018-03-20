use std::collections::BTreeMap;
use error::Result;

use rio_net::metrics::prometheus::PrometheusClient;
use rio_net::metrics::collector::{Collector, CollectorScope};

use protocol::api::{scale, node};
use protocol::api::base::QueryInput;

const METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID: &'static str = "rioos_assemblyfactory_id";
const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";

pub struct Client<'a> {
    prom: &'a PrometheusClient,
}
impl<'a> Client<'a> {
    pub fn new(prom: &'a PrometheusClient) -> Self {
        Client { prom: prom }
    }
    pub fn metrics(&self, af_id: &str, querypair: QueryInput) -> Result<Option<Vec<scale::ScalingGetResponse>>> {
        let metric_response = match &format!("job={}", querypair.get("job"))[..] {
            node::CONTAINER_JOBS => self.container_metric(&af_id, querypair),
            _ => self.assembly_metric(&af_id, querypair),
        };
        let mut response = scale::ScalingGetResponse::new();
        response.set_metrics(metric_response?);
        Ok(Some(vec![response]))
    }


    fn assembly_metric(&self, af_id: &str, querypair: QueryInput) -> Result<BTreeMap<String, String>> {
        let metric_scope = vec![querypair.get("source").to_string()];
        let label_collection: Vec<String> = vec![
            format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID, af_id).to_string(),
            format!("job={}", querypair.get("job").to_string()),
            node::IDLEMODE.to_string(),
        ];

        let scope = CollectorScope {
            metric_names: metric_scope,
            labels: label_collection,
            last_x_minutes: METRIC_DEFAULT_LAST_X_MINUTE.to_string(),
            avg_by_name: "rioos_assembly_id".to_string(),
        };
        Ok(Collector::new(self.prom, scope).metric_by_avg()?)
    }

    fn container_metric(&self, af_id: &str, querypair: QueryInput) -> Result<BTreeMap<String, String>> {
        let metric_scope = vec![querypair.get("source").to_string()];
        let label_collection: Vec<String> = vec![
            format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID, af_id).to_string(),
            format!("job={}", querypair.get("job").to_string()),
        ];

        let scope = CollectorScope {
            metric_names: metric_scope,
            labels: label_collection,
            last_x_minutes: METRIC_DEFAULT_LAST_X_MINUTE.to_string(),
            avg_by_name: "rioos_assembly_id".to_string(),
        };
        Ok(Collector::new(self.prom, scope)
            .metric_by_avg_for_containers()?)
    }
}
