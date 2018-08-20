

use super::models::assemblyfactory;
use db::data_store::DataStoreConn;
use human_size::Size;

use job::{error, models, JobOutput};
use protocol::api::{deploy, job, node, scale};
use protocol::api::base::{MetaFields, WhoAmITypeMeta, Status, IdGet};
///replicas expander
use protocol::api::schema::type_meta_url;


const METRIC_LIMIT: &'static str = "10";
use rand::{self, Rng};
use rand::distributions::Alphanumeric;
const PRE_NAME_LEN: usize = 5;

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

        let id_get = IdGet::with_id(qualified_assembly.get_owner_references()[0].get_uid());

        let factory = assemblyfactory::DataStore::new(&self.conn)
            .show(&id_get)
            .unwrap();

        if factory.is_some() {
            let mut factory = factory.unwrap();
            let mut resources = factory.get_resources().clone();
            for (key, value) in resources.iter_mut() {
                if key == node::CAPACITY_MEMORY {
                    *value = self.scaling_policy
                        .get_status()
                        .get_desired_resource()
                        .get(node::CAPACITY_MEMORY)
                        .unwrap_or(&"0 KiB".to_string())
                        .to_string();
                } else if key == node::CAPACITY_CPU {
                    *value = self.scaling_policy
                        .get_status()
                        .get_desired_resource()
                        .get(node::CAPACITY_CPU)
                        .unwrap_or(&"0".to_string())
                        .to_string();
                }
            }
            factory.set_resources(resources.clone());
            assemblyfactory::DataStore::new(&self.conn).update(&factory);
        }
        models::jobs::DataStore::new(&self.conn).create(&self.build_job(&qualified_assembly, &self.get_scale_type()))
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
        if (self.average_node_metric() == None || self.assembly_metric(assembly) == None) || (self.average_node_metric() < self.metric_limit() && self.assembly_metric(assembly) < self.metric_limit()) {
            return false;
        }
        return true;
    }

    /// compare the current and disered resource of vs to scale_down or scale_up
    fn get_scale_type(&self) -> String {
        if self.current_cpu() < self.desired_cpu() || self.current_ram() < self.desired_ram() {
            return "scale_up".to_string();
        }
        return "scale_down".to_string();
    }

    /// create the new job for scale_up or scale_down
    fn build_job(&self, assembly: &deploy::Assembly, scale_type: &str) -> job::Jobs {
        let mut job_create = job::Jobs::new();

        let ref mut om = job_create.mut_meta(
            job_create.object_meta(),
            format!("{}-{}", self.pre_name(), assembly.get_name()),
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

        job_create.set_status(Status::pending());

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
                .unwrap_or(&"0".to_string())
                .to_string(),
        )
    }

    fn current_ram(&self) -> Option<f64> {
        Some(
            self.scaling_policy
                .get_status()
                .get_current_resource()
                .get("ram")
                .unwrap_or(&"0 KiB".to_string())
                .parse::<Size>()
                .unwrap()
                .into_bytes(),
        )
    }

    fn desired_cpu(&self) -> Option<String> {
        Some(
            self.scaling_policy
                .get_status()
                .get_desired_resource()
                .get("cpu")
                .unwrap_or(&"0".to_string())
                .to_string(),
        )
    }

    fn desired_ram(&self) -> Option<f64> {
        Some(
            self.scaling_policy
                .get_status()
                .get_desired_resource()
                .get("ram")
                .unwrap_or(&"0 KiB".to_string())
                .parse::<Size>()
                .unwrap()
                .into_bytes(),
        )
    }

    fn average_node_metric(&self) -> Option<String> {
        let mut temp = 0.0;
        let average: Option<String> = self.overall_metrics
            .clone()
            .unwrap()
            .get_results()
            .get_gauges()
            .get_counters()
            .iter()
            .map(|x| {
                if x.get_counter().as_str() > "0" {
                    temp = temp + x.get_counter().parse::<f64>().unwrap_or(0.0);
                    let avg = (temp + 10.0) / 3.0;
                    return Some(avg.to_string());
                }
                None
            })
            .collect();
        average
    }

    fn assembly_metric(&self, assembly: &deploy::Assembly) -> Option<String> {
        if assembly.get_spec().get_metrics().is_some() {
            return Some(
                assembly
                    .get_spec()
                    .get_metrics()
                    .unwrap()
                    .get(&assembly.get_id())
                    .unwrap()
                    .to_string(),
            );

        }
        return None;
    }

    fn metric_limit(&self) -> Option<String> {
        Some(METRIC_LIMIT.to_string())
    }

    fn pre_name(&self) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(PRE_NAME_LEN)
            .collect::<String>()
            .to_lowercase()
    }
}
