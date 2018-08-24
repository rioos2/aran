use super::*;
use metrics::hooks::before::AHooks;
use protocol::api::base::MetaFields;
use protocol::api::node;
use protocol::api::node::{Data, InstantVecItem, PromResponse};
use serde_json;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct Collector {
    content: AHooks,
}

impl Collector {
    pub fn new(content: AHooks) -> Self {
        Collector { content: content }
    }

    pub fn get_reports(&mut self) -> node::HealthzAllGet {
        let mut x = node::HealthzAllGet::new();
        x.set_title("Command center operations".to_string());
        x.set_gauges(self.mk_guages());
        x.set_statistics(self.new_statistics());
        x
    }

    fn mk_guages(&self) -> node::Guages {
        vec![
            serde_json::from_str(&self.content.get(CPU_CONSUMPTION).unwrap()).unwrap(),
            serde_json::from_str(&self.content.get(MEMORY_CONSUMPTION).unwrap()).unwrap(),
            serde_json::from_str(&self.content.get(STORAGE_CONSUMPTION).unwrap()).unwrap(),
        ].into()
    }

    fn new_statistics(&mut self) -> node::Statistics {
        let mut statistics = node::Statistics::new();
        statistics.set_title("Statistics".to_string());
        statistics.set_ninjas(
            serde_json::from_str(&self.content.get("ninjas").unwrap()).unwrap(),
        );
        statistics.set_senseis(
            serde_json::from_str(&self.content.get("senseis").unwrap()).unwrap(),
        );
        statistics
    }
}
