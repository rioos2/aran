use error::Result;
use std::collections::BTreeMap;

use telemetry::metrics::collector::{Collector, CollectorScope};
use telemetry::metrics::prometheus::PrometheusClient;

use protocol::api::base::QueryInput;
use protocol::api::{node, scale};

const METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID: &'static str = "rioos_assemblyfactory_id";
const METRIC_LBL_RIOOS_ASSEMBLY_ID: &'static str = "rioos_assembly_id";

const METRIC_DEFAULT_LAST_X_MINUTE: &'static str = "[5m]";

pub struct Client<'a> {
    prom: &'a PrometheusClient,
}
impl<'a> Client<'a> {
    pub fn new(prom: &'a PrometheusClient) -> Self {
        Client { prom: prom }
    }
    pub fn metrics(
        &self,
        af_id: &str,
        querypair: QueryInput,
    ) -> Result<Option<Vec<scale::ScalingGetResponse>>> {
        let metric_response = match &format!("job={}", querypair.get("job"))[..] {
            node::CONTAINER_JOBS => self.container_metric(&af_id),
            _ => self.assembly_metric(&af_id, querypair),
        };
        let mut response = scale::ScalingGetResponse::new();
        response.set_metrics(metric_response?);
        Ok(Some(vec![response]))
    }

    fn assembly_metric(
        &self,
        af_id: &str,
        querypair: QueryInput,
    ) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
        let mut data = BTreeMap::new();
        let label_collection: Vec<String> = vec![
            format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID, af_id).to_string(),
            format!("job={}", querypair.get("job").to_string()),
            node::IDLEMODE.to_string(),
        ];

        let assembly_cpu_scope = collect_scope(
            vec!["node_cpu".to_string()],
            label_collection,
            METRIC_DEFAULT_LAST_X_MINUTE,
            METRIC_LBL_RIOOS_ASSEMBLY_ID,
        );
        data.insert(
            "cpu".to_string(),
            Collector::new(self.prom, assembly_cpu_scope).metric_by_avg_for_machines()?,
        );
        Ok(data)
    }

    fn container_metric(&self, af_id: &str) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
        let mut data = BTreeMap::new();
        let label_collection: Vec<String> =
            vec![format!("{}={}", METRIC_LBL_RIOOS_ASSEMBLYFACTORY_ID, af_id).to_string()];

        let container_cpu_scope = collect_scope(
            vec!["container_cpu_usage_seconds_total".to_string()],
            label_collection.clone(),
            METRIC_DEFAULT_LAST_X_MINUTE,
            METRIC_LBL_RIOOS_ASSEMBLY_ID,
        );

        let container_mem_scope = collect_scope(
            vec![
                "container_memory_usage_bytes".to_string(),
                "container_spec_memory_limit_bytes".to_string(),
            ],
            label_collection.clone(),
            "",
            "",
        );

        let container_disk_scope = collect_scope(
            vec![
                "container_fs_usage_bytes".to_string(),
                "container_fs_limit_bytes".to_string(),
            ],
            label_collection.clone(),
            "",
            "",
        );
        data.insert(
            "cpu".to_string(),
            Collector::new(self.prom, container_cpu_scope).metric_by_avg_for_containers("cpu")?,
        );
        data.insert(
            "memory".to_string(),
            Collector::new(self.prom, container_mem_scope.clone())
                .metric_by_avg_for_containers("ram")?,
        );
        data.insert(
            "disk".to_string(),
            Collector::new(self.prom, container_disk_scope.clone())
                .metric_by_avg_for_containers("disk")?,
        );
        Ok(data)
    }
}

fn collect_scope(
    metric_scope: Vec<String>,
    labels: Vec<String>,
    duration: &str,
    avg_by: &str,
) -> CollectorScope {
    CollectorScope {
        metric_names: metric_scope,
        labels: labels,
        last_x_minutes: duration.to_string(),
        avg_by_name: avg_by.to_string(),
    }
}
