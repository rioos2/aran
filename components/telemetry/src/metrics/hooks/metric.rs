// Copyright 2018 The Rio Advancement Inc

//! The startup hook is responsible for setting the sensei nodes during the startup.

use std::collections::BTreeMap;

use metrics::PromResponse;
use metrics::hooks::BeforeMetrics;
use serde_json;

pub struct Metric {
    content: PromResponse,
}

impl Metric {
    pub fn new(content: PromResponse) -> Metric {
        Metric { content: content }
    }
    fn get_content(&mut self) -> Option<String> {
        let metric: BTreeMap<String, String> = self.content.clone().into();
        serde_json::to_string(&metric).ok()
    }
}

impl BeforeMetrics for Metric {
    fn before(&mut self) -> Option<String> {
        self.get_content()
    }
}
