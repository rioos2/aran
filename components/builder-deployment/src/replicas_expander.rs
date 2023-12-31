///replicas expander

use protocol::api::{deploy, node, job, scale};
use protocol::api::base::{MetaFields, WhoAmITypeMeta};

use rio_net::http::schema::type_meta_url;

use job::{JobOutput, job_ds, error};

use db::data_store::DataStoreConn;

const METRIC_LIMIT: i32 = 10;

pub struct ReplicasExpander<'a> {
    conn: &'a DataStoreConn,
    assemblys: Vec<deploy::Assembly>,
    overall_metrics: Option<node::HealthzAllGetResponse>,
    scaling_policy: &'a scale::VerticalScaling,
}

impl<'a> ReplicasExpander<'a> {
    pub fn new(conn: &'a DataStoreConn, assemblys: Vec<deploy::Assembly>, overall_metrics: Option<node::HealthzAllGetResponse>, scale: &'a scale::VerticalScaling) -> Self {
        ReplicasExpander {
            conn: &*conn,
            assemblys: assemblys,
            overall_metrics: overall_metrics,
            scaling_policy: scale,
        }
    }

    /// expand the least resource assembly and create the job
    pub fn expand(&self) -> JobOutput {
        let qualified_assembly = self.qualified_assembly();
        if !self.satisfy_metrics(&qualified_assembly) {
            return Err(error::Error::METRICLIMITERROR);
            // assembly::DataStore::new(&self.conn).status_update(&self.build_assembly_status());
        }
        job_ds::JobDS::create(
            &self.conn,
            &self.build_job(&qualified_assembly, &self.get_scale_type()),
        )
    }

    // should return the least resource utilies assembly
    fn qualified_assembly(&self) -> deploy::Assembly {
        // for x in 0..self.assemblys.len() {
        //
        // }
        self.assemblys[0].clone()
    }

    /// check the datacenter metrics and node metric of assembly
    fn satisfy_metrics(&self, assembly: &deploy::Assembly) -> bool {
        if (self.assembly_metric(assembly) == 0 || self.average_node_metric() == 0) || (self.average_node_metric() < METRIC_LIMIT && self.assembly_metric(assembly) < METRIC_LIMIT) {
            return false;
        }
        return true;
    }

    /// compare the current and disered resource of vs to scale_down or scale_up
    fn get_scale_type(&self) -> String {
        if self.current_cpu() < self.desired_cpu() && self.current_ram() < self.desired_ram() {
            return "verticalScaleUp".to_string();
        }
        return "verticalScaleDown".to_string();
    }

    /// create the new job for scale_up or scale_down
    fn build_job(&self, assembly: &deploy::Assembly, scale_type: &str) -> job::Jobs {
        let mut job_create = job::Jobs::new();

        let ref mut om = job_create.mut_meta(
            job_create.object_meta(),
            assembly.get_name(),
            assembly.get_account(),
        );
        job_create.set_owner_reference(
            om,
            assembly.type_meta().kind,
            assembly.type_meta().api_version,
            assembly.get_name(),
            assembly.get_id(),
        );
        let jackie = job_create.who_am_i();

        job_create.set_meta(type_meta_url(jackie), om.clone());

        job_create.set_spec(job::SpecData::with(
            assembly
                .get_metadata()
                .get("rioos_sh_scheduled_node")
                .unwrap_or(&"".to_string()),
            "assembly",
            scale_type,
        ));
        job_create
    }

    // update the assembly status if scale can't done in the node
    // fn build_assembly_status(&self) -> base::StatusUpdate {}

    fn current_cpu(&self) -> Option<String> {
        Some(
            self.scaling_policy
                .get_status()
                .get_current_resource()
                .get("cpu")
                .unwrap_or(&"".to_string())
                .to_string(),
        )
    }

    fn current_ram(&self) -> Option<String> {
        Some(
            self.scaling_policy
                .get_status()
                .get_current_resource()
                .get("ram")
                .unwrap_or(&"".to_string())
                .to_string(),
        )
    }

    fn desired_cpu(&self) -> Option<String> {
        Some(
            self.scaling_policy
                .get_status()
                .get_desired_resource()
                .get("cpu")
                .unwrap_or(&"".to_string())
                .to_string(),
        )
    }

    fn desired_ram(&self) -> Option<String> {
        Some(
            self.scaling_policy
                .get_status()
                .get_desired_resource()
                .get("ram")
                .unwrap_or(&"".to_string())
                .to_string(),
        )
    }

    fn average_node_metric(&self) -> i32 {
        let mut temp = 0;
        let average: String = self.overall_metrics
            .clone()
            .unwrap()
            .get_results()
            .get_gauges()
            .get_counters()
            .iter()
            .map(|x| {
                if x.get_counter().parse::<i32>().unwrap() > 0 {
                    temp = temp + x.get_counter().parse::<i32>().unwrap_or(0);
                    let avg = (temp + 10) / 3;
                    return avg.to_string();
                }
                return "0".to_string();
            })
            .collect();
        average.parse::<i32>().unwrap_or(0)
    }

    fn assembly_metric(&self, assembly: &deploy::Assembly) -> i32 {
        if assembly.get_spec().get_metrics().is_some() {
            return assembly
                .get_spec()
                .get_metrics()
                .unwrap()
                .get(&assembly.get_id())
                .unwrap_or(&"0".to_string())
                .parse::<i32>()
                .unwrap_or(0);
        }
        return 0;
    }
}
